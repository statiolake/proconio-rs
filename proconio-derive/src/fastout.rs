// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proc_macro::TokenStream;
use proc_macro2::Span as Span2;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;
use syn::visit::{self, Visit};
use syn::{parse_macro_input, parse_quote};
use syn::{Block, ExprClosure, ExprMacro, ItemFn, Macro, Path, Stmt};

pub fn main(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut itemfn: ItemFn = parse_macro_input!(input as ItemFn);

    if !attr.is_empty() {
        let (start, end) = crate::get_span_range(attr);

        let compile_error = crate::compile_error_at(
            quote!("no extra attribute is suppported."),
            Span2::from(start),
            Span2::from(end),
        );

        itemfn.block.stmts = vec![compile_error];
        return itemfn.into_token_stream().into();
    }

    if let Err(compile_errors) = error_for_print_macros_in_closures(&itemfn.block) {
        itemfn.block.stmts = compile_errors;
        return itemfn.into_token_stream().into();
    }

    itemfn.block = Box::new(insert_new_print_macros(&itemfn.block));

    itemfn.into_token_stream().into()
}

fn error_for_print_macros_in_closures(block: &Block) -> std::result::Result<(), Vec<Stmt>> {
    let mut visitor = BlockVisitor::default();
    visitor.visit_block(block);
    return if visitor.compile_errors.is_empty() {
        Ok(())
    } else {
        Err(visitor.compile_errors)
    };

    #[derive(Default)]
    struct BlockVisitor {
        compile_errors: Vec<Stmt>,
    }

    impl<'ast> Visit<'ast> for BlockVisitor {
        fn visit_expr_closure(&mut self, item: &'ast ExprClosure) {
            let mut visitor = ClosureVisitor::default();
            visitor.visit_expr_closure(item);
            self.compile_errors.extend(visitor.compile_errors);
        }
    }

    #[derive(Default)]
    struct ClosureVisitor {
        compile_errors: Vec<Stmt>,
    }

    impl<'ast> Visit<'ast> for ClosureVisitor {
        fn visit_expr_macro(&mut self, item: &'ast ExprMacro) {
            let Macro { path, .. } = &item.mac;
            if equals(path, "print") || equals(path, "println") {
                // Closure containing print macro is prohibited because closure *may* passed to the
                // function which requires the closure to be `Send`.  For example, std::thread::spawn()
                // takes an closure and run the closure in another thread.  That causes an error since
                // StdoutLock is not thread-safe.  The problem is, the error message is too-complecated
                // for beginners since the error originates from invisible codes inserted by this
                // procedural macro.  Yes, if the closure don't have to be `Send`, it's OK to have
                // print macros in it.  However such a trait boundary is not yet resolved at the time
                // of macro expansion, so it is impossible to change behavior selectively here.  For
                // that reason, all closures must not have print! call in it regardless of whether it
                // requires `Send` or not for now.

                // emit an error for each position of print macro.
                self.compile_errors.push(crate::compile_error_at(
                    quote!(
                        "Closures in a #[fastout] function cannot contain `print!` or \
                        `println!` macro\n\
                        \n\
                        note: If you want to run your entire logic in a thread having extended \
                        size of stack, you can define a new function instead.  See \
                        documentation (https://docs.rs/proconio/#\
                        closures-having-print-or-println-in-fastout-function) for more \
                        details.\n\
                        \n\
                        note: This is because if you use this closure with \
                        `std::thread::spawn()` or any other functions requiring `Send` for an \
                        argument closure, the compiler emits an error about thread unsafety for \
                        our internal implementations.  If you are using the closure just in a \
                        single thread, it's actually no problem, but we cannot check the trait \
                        bounds at the macro-expansion time.  So for now, all closures having \
                        `print!` or `println!` is prohibited regardless of the `Send` \
                        requirements."
                    ),
                    item.span(),
                    item.span(),
                ));
            }
            visit::visit_expr_macro(self, item);
        }
    }

    fn equals(path: &Path, ident: &str) -> bool {
        matches!(path.get_ident(), Some(path_ident) if path_ident == ident)
    }
}

fn insert_new_print_macros(block: &Block) -> Block {
    parse_quote! {{
        let __proconio_stdout = ::std::io::stdout();
        let mut __proconio_stdout = ::std::io::BufWriter::new(__proconio_stdout.lock());

        #[allow(unused_macros)]
        macro_rules! print {
            ($($tt:tt)*) => {
                <::std::io::BufWriter<::std::io::StdoutLock<'_>> as ::std::io::Write>::write_fmt(
                    &mut __proconio_stdout,
                    format_args!($($tt)*),
                )
                .unwrap()
            };
        }

        #[allow(unused_macros)]
        macro_rules! println {
            () => {
                <::std::io::BufWriter<::std::io::StdoutLock<'_>> as ::std::io::Write>::write_all(
                    &mut __proconio_stdout,
                    b"\n",
                )
                .unwrap()
            };
            ($fmt:literal $($tt:tt)*) => {
                <::std::io::BufWriter<::std::io::StdoutLock<'_>> as ::std::io::Write>::write_fmt(
                    &mut __proconio_stdout,
                    format_args!(::std::concat!($fmt, "\n") $($tt)*),
                )
                .unwrap()
            };
        }

        let __proconio_res = #block;
        <::std::io::BufWriter<::std::io::StdoutLock> as ::std::io::Write>::flush(&mut __proconio_stdout).unwrap();
        return __proconio_res;
    }}
}
