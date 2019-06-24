// Copyright 2019 statiolake <statiolake@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::quote;
use quote::ToTokens;
use std::borrow::BorrowMut;
use std::mem::replace;
use syn::spanned::Spanned;
use syn::{parse_macro_input, parse_quote};
use syn::{Block, Expr, ExprMacro, ItemFn, Path, Stmt};

pub fn main(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut itemfn: ItemFn = parse_macro_input!(input as ItemFn);

    if !attr.is_empty() {
        let (start, end) = crate::get_span(attr);

        let compile_error = crate::compile_error_at(
            quote!("no extra attribute is suppported."),
            Span2::from(start),
            Span2::from(end),
        );

        itemfn.block.stmts = vec![compile_error];
        return itemfn.into_token_stream().into();
    }

    // replace print
    replace_print_macro_in_block(&mut itemfn.block);

    // adds codes for preparing / flushing BufWriter to the function body.
    insert_bufwriter_to_block(&mut itemfn.block);

    itemfn.into_token_stream().into()
}

fn replace_print_macro_in_block(block: &mut Block) -> Vec<Span2> {
    let mut did_replace = Vec::new();

    for stmt in &mut block.stmts {
        did_replace.extend(replace_print_macro_in_stmt(stmt));
    }

    did_replace
}

fn replace_print_macro_in_stmt(stmt: &mut Stmt) -> Vec<Span2> {
    let expr = match stmt {
        Stmt::Local(local) => match &mut local.init {
            Some((_eq, init)) => init.borrow_mut(),
            _ => return Vec::new(),
        },
        Stmt::Expr(expr) => expr,
        Stmt::Semi(expr, _) => expr,
        _ => return Vec::new(),
    };

    replace_print_macro_in_expr(expr)
}

fn replace_print_macro_in_expr(expr: &mut Expr) -> Vec<Span2> {
    macro_rules! rbox {
        ($e:expr) => {
            replace_print_macro_in_expr($e.borrow_mut())
        };
    }

    macro_rules! riter {
        ($i:expr) => {
            $i.iter_mut().fold(Vec::new(), |mut did_replace, e| {
                did_replace.extend(replace_print_macro_in_expr(e));
                did_replace
            })
        };
    }

    let emac = match expr {
        Expr::Macro(emac) => emac,
        Expr::Box(i) => return rbox!(i.expr),
        Expr::InPlace(i) => {
            let mut did_replace = Vec::new();
            did_replace.extend(rbox!(i.place));
            did_replace.extend(rbox!(i.value));
            return did_replace;
        }
        Expr::Array(i) => return riter!(i.elems),
        Expr::Call(i) => return riter!(i.args),
        Expr::MethodCall(i) => {
            let mut did_replace = Vec::new();
            did_replace.extend(rbox!(i.receiver));
            did_replace.extend(riter!(i.args));
            return did_replace;
        }
        Expr::Tuple(i) => return riter!(i.elems),
        Expr::Binary(i) => {
            let mut did_replace = Vec::new();
            did_replace.extend(rbox!(i.left));
            did_replace.extend(rbox!(i.right));
            return did_replace;
        }
        Expr::Unary(i) => return rbox!(i.expr),
        Expr::Cast(i) => return rbox!(i.expr),
        Expr::Type(i) => return rbox!(i.expr),
        Expr::Let(i) => return rbox!(i.expr),
        Expr::If(i) => {
            let mut did_replace = rbox!(i.cond);
            did_replace.extend(replace_print_macro_in_block(&mut i.then_branch));
            if let Some((_, e)) = &mut i.else_branch {
                did_replace.extend(rbox!(e));
            }
            return did_replace;
        }
        Expr::While(i) => {
            let mut did_replace = rbox!(i.cond);
            did_replace.extend(replace_print_macro_in_block(&mut i.body));
            return did_replace;
        }
        Expr::ForLoop(i) => {
            let mut did_replace = rbox!(i.expr);
            did_replace.extend(replace_print_macro_in_block(&mut i.body));
            return did_replace;
        }
        Expr::Loop(i) => return replace_print_macro_in_block(&mut i.body),
        Expr::Match(i) => return rbox!(i.expr),
        Expr::Closure(i) => {
            let did_replace = rbox!(i.body);

            if !did_replace.is_empty() {
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
                let mut stmts: Vec<Stmt> = Vec::new();
                for span in did_replace {
                    let compile_error = crate::compile_error_at(
                        quote!(
                            "Closures in a #[fastout] function cannot contain `print!` or \
                            `println!` macro\n\
                            \n\
                            note: If you want to run your entire logic in a thread having extended \
                            size of stack, you can define a new function instead.  See \
                            documentation (https://docs.rs/proconio/0.1.2/proconio/#\
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
                        span,
                        span,
                    );

                    stmts.push(compile_error)
                }

                *expr = parse_quote!({ #(#stmts)* });
            }

            // To avoid multiple compilation error for the same println, return empty vec.
            return Vec::new();
        }
        Expr::Unsafe(i) => return replace_print_macro_in_block(&mut i.block),
        Expr::Block(i) => return replace_print_macro_in_block(&mut i.block),
        Expr::Assign(i) => {
            let mut did_replace = Vec::new();
            did_replace.extend(rbox!(i.left));
            did_replace.extend(rbox!(i.right));
            return did_replace;
        }
        Expr::AssignOp(i) => {
            let mut did_replace = Vec::new();
            did_replace.extend(rbox!(i.left));
            did_replace.extend(rbox!(i.right));
            return did_replace;
        }
        Expr::Field(i) => return rbox!(i.base),
        Expr::Index(i) => {
            let mut did_replace = Vec::new();
            did_replace.extend(rbox!(i.expr));
            did_replace.extend(rbox!(i.index));
            return did_replace;
        }
        Expr::Range(i) => {
            let mut did_replace = Vec::new();
            if let Some(from) = &mut i.from {
                did_replace.extend(rbox!(from));
            }
            if let Some(to) = &mut i.to {
                did_replace.extend(rbox!(to));
            }
            return did_replace;
        }
        Expr::Reference(i) => return rbox!(i.expr),
        Expr::Break(i) => {
            return match &mut i.expr {
                Some(e) => rbox!(e),
                None => Vec::new(),
            };
        }
        Expr::Return(i) => {
            return match &mut i.expr {
                Some(e) => rbox!(e),
                None => Vec::new(),
            };
        }
        Expr::Struct(i) => {
            let mut did_replace = i.fields.iter_mut().fold(Vec::new(), |mut did_replace, f| {
                did_replace.extend(replace_print_macro_in_expr(&mut f.expr));
                did_replace
            });
            if let Some(rest) = &mut i.rest {
                did_replace.extend(rbox!(rest));
            }
            return did_replace;
        }
        Expr::Repeat(i) => {
            let mut did_replace = Vec::new();
            did_replace.extend(rbox!(i.expr));
            did_replace.extend(rbox!(i.len));
            return did_replace;
        }
        Expr::Paren(i) => return rbox!(i.expr),
        Expr::Group(i) => return rbox!(i.expr),
        Expr::Try(i) => return rbox!(i.expr),
        Expr::Async(i) => return replace_print_macro_in_block(&mut i.block),
        Expr::TryBlock(i) => return replace_print_macro_in_block(&mut i.block),
        Expr::Yield(i) => {
            return match &mut i.expr {
                Some(e) => rbox!(e),
                None => Vec::new(),
            }
        }
        Expr::Lit(_) => return Vec::new(),
        Expr::Path(_) => return Vec::new(),
        Expr::Continue(_) => return Vec::new(),
        Expr::Verbatim(_) => return Vec::new(),
    };

    let emac = replace(
        emac,
        parse_quote!(compile_error!(
            "Replacing print macro did not complete.  This is a bug."
        )),
    );

    let (did_replace, replaced) = generate_print_replaced_expr(emac);
    *expr = replaced;
    did_replace
}

fn generate_print_replaced_expr(emac: ExprMacro) -> (Vec<Span2>, Expr) {
    // helper macro to parse path
    macro_rules! path {
        ($($tt:tt)*) => {
            syn::parse::<Path>(quote!($($tt)*).into())
                .expect("Failed to parse path.  This is a bug.")
        };
    }

    // replace print! -> write!
    let path_print = vec![path!(::std::print), path!(std::print), path!(print)];
    let path_println = vec![path!(::std::println), path!(std::println), path!(println)];

    let has_newline = if path_print.contains(&emac.mac.path) {
        false
    } else if path_println.contains(&emac.mac.path) {
        true
    } else {
        // If the macro is not `print*!`, do nothing.
        return (Vec::new(), Expr::Macro(emac));
    };

    // preserve span for returning
    let span = emac.mac.span();
    // if macro is with new line version, support that.
    let format_args_args = if has_newline {
        // take ownership of TokenStream from the macro temporary.
        let mut tts = emac.mac.tts.into_iter();

        // `lit` must be the format string literal.
        //  I don't care if `lit` is actually a string literal here since that will be error later
        //  in `format_args!`.
        let lit = tts.next();

        // `rest` are format arguments.
        let rest: TokenStream2 = tts.collect();

        // generate the newlined version of format string and interpolate it.
        match lit {
            Some(first) => quote!(concat!(#first, "\n") #rest),
            None => quote!("\n"),
        }
    } else {
        // otherwise, no translations are needed.
        emac.mac.tts
    };

    // replace the expression of `print!` and `println!` macro call.
    let replaced = parse_quote! {
        <::std::io::BufWriter<::std::io::StdoutLock> as ::std::io::Write>::write_fmt(
            &mut __proconio_stdout,
            format_args!(#format_args_args)
        )
        .unwrap()
    };

    (vec![span], replaced)
}

fn insert_bufwriter_to_block(block: &mut Block) {
    let replaced: Block = parse_quote! {{
        let __proconio_stdout = ::std::io::stdout();
        let mut __proconio_stdout = ::std::io::BufWriter::new(__proconio_stdout.lock());
        let __proconio_res = #block;
        <::std::io::BufWriter<::std::io::StdoutLock> as ::std::io::Write>::flush(&mut __proconio_stdout).unwrap();
        return __proconio_res;
    }};

    replace(block, replaced);
}
