use proc_macro::TokenStream;
use quote::quote;
use std::borrow::BorrowMut;
use std::mem::replace;
use syn::parse_quote;
use syn::{Block, Expr, ExprMacro, ItemFn, Path, Stmt};

pub fn main(attr: TokenStream, input: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        panic!("no extra attribute is supported.");
    }

    let mut itemfn: ItemFn = syn::parse(input).expect("failed to parse item");

    // replace print
    replace_print_macro_in_block(&mut itemfn.block);

    // adds codes for preparing / flushing BufWriter to the function body.
    insert_bufwriter_to_block(&mut itemfn.block);

    quote!(#itemfn).into()
}

fn replace_print_macro_in_block(block: &mut Block) {
    for stmt in &mut block.stmts {
        replace_print_macro_in_stmt(stmt);
    }
}

fn replace_print_macro_in_stmt(stmt: &mut Stmt) {
    let expr = match stmt {
        Stmt::Local(local) => match &mut local.init {
            Some((_eq, init)) => init.borrow_mut(),
            _ => return,
        },
        Stmt::Expr(expr) => expr,
        Stmt::Semi(expr, _) => expr,
        _ => return,
    };

    replace_print_macro_in_expr(expr);
}

fn replace_print_macro_in_expr(expr: &mut Expr) {
    macro_rules! rbox {
        ($i:ident) => {
            rbox!($i.expr);
        };

        ($e:expr) => {
            replace_print_macro_in_expr($e.borrow_mut())
        };
    }

    macro_rules! riter {
        ($i:expr) => {
            $i.iter_mut().for_each(replace_print_macro_in_expr)
        };
    }

    let emac = match expr {
        Expr::Macro(emac) => emac,

        Expr::Box(i) => return rbox!(i),
        Expr::InPlace(i) => {
            rbox!(i.place);
            rbox!(i.value);
            return;
        }
        Expr::Array(i) => return riter!(i.elems),
        Expr::Call(i) => return riter!(i.args),
        Expr::MethodCall(i) => {
            rbox!(i.receiver);
            riter!(i.args);
            return;
        }
        Expr::Tuple(i) => return riter!(i.elems),
        Expr::Binary(i) => {
            rbox!(i.left);
            rbox!(i.right);
            return;
        }
        Expr::Unary(i) => return rbox!(i),
        Expr::Cast(i) => return rbox!(i),
        Expr::Type(i) => return rbox!(i),
        Expr::Let(i) => return rbox!(i),
        Expr::If(i) => {
            rbox!(i.cond);
            replace_print_macro_in_block(&mut i.then_branch);
            if let Some((_, e)) = &mut i.else_branch {
                rbox!((e));
            }
            return;
        }
        Expr::While(i) => {
            rbox!(i.cond);
            replace_print_macro_in_block(&mut i.body);
            return;
        }
        Expr::ForLoop(i) => {
            rbox!(i);
            replace_print_macro_in_block(&mut i.body);
            return;
        }
        Expr::Loop(i) => return replace_print_macro_in_block(&mut i.body),
        Expr::Match(i) => return rbox!(i),
        Expr::Closure(i) => return rbox!(i.body),
        Expr::Unsafe(i) => return replace_print_macro_in_block(&mut i.block),
        Expr::Block(i) => return replace_print_macro_in_block(&mut i.block),
        Expr::Assign(i) => {
            rbox!(i.left);
            rbox!(i.right);
            return;
        }
        Expr::AssignOp(i) => {
            rbox!(i.left);
            rbox!(i.right);
            return;
        }
        Expr::Field(i) => return rbox!(i.base),
        Expr::Index(i) => {
            rbox!(i);
            rbox!(i.index);
            return;
        }
        Expr::Range(i) => {
            if let Some(from) = &mut i.from {
                rbox!((from));
            }
            if let Some(to) = &mut i.to {
                rbox!((to));
            }
            return;
        }
        Expr::Reference(i) => return rbox!(i),
        Expr::Break(i) => {
            if let Some(e) = &mut i.expr {
                rbox!((e));
            }
            return;
        }
        Expr::Return(i) => {
            if let Some(e) = &mut i.expr {
                rbox!((e));
            }
            return;
        }
        Expr::Struct(i) => {
            i.fields
                .iter_mut()
                .for_each(|f| replace_print_macro_in_expr(&mut f.expr));
            if let Some(rest) = &mut i.rest {
                rbox!((rest));
            }
            return;
        }
        Expr::Repeat(i) => {
            rbox!(i);
            rbox!(i.len);
            return;
        }
        Expr::Paren(i) => return rbox!(i),
        Expr::Group(i) => return rbox!(i),
        Expr::Try(i) => return rbox!(i),
        Expr::Async(i) => return replace_print_macro_in_block(&mut i.block),
        Expr::TryBlock(i) => return replace_print_macro_in_block(&mut i.block),
        Expr::Yield(i) => {
            if let Some(e) = &mut i.expr {
                rbox!((e));
            }
            return;
        }
        Expr::Lit(_) => return,
        Expr::Path(_) => return,
        Expr::Continue(_) => return,
        Expr::Verbatim(_) => return,
    };

    let emac = replace(
        emac,
        parse_quote!(compile_error!(
            "Replacing print macro did not complete.  This is a bug."
        )),
    );
    *expr = generate_print_replaced_expr(emac);
}

fn generate_print_replaced_expr(emac: ExprMacro) -> Expr {
    // helper to parse path
    macro_rules! path {
        ($($tt:tt)*) => {
            syn::parse::<Path>(quote!($($tt)*).into())
                .expect("failed to parse path")
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
        return Expr::Macro(emac);
    };

    // if macro is with new line version, support that.
    let format_args_args = if has_newline {
        // take ownership of TokenStream from the macro temporary.
        let mut tts = emac.mac.tts.into_iter();

        // `lit` must be the format string literal.
        //  I don't care if `lit` is actually a string literal here since that will be error later
        //  in `format_args!`.
        let lit = tts.next();

        // `rest` are format arguments.
        let rest: proc_macro2::TokenStream = tts.collect();

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
    parse_quote! {
        <::std::io::BufWriter<::std::io::StdoutLock> as ::std::io::Write>::write_fmt(
            &mut __proconio_stdout,
            format_args!(#format_args_args)
        )
        .unwrap()
    }
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
