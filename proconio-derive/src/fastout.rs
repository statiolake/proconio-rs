use proc_macro::TokenStream;
use quote::quote;
use std::borrow::BorrowMut;
use std::mem::replace;
use syn::parse_quote;
use syn::{Block, Expr, ItemFn, Path, Stmt};

pub fn main(attr: TokenStream, input: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        panic!("no extra attribute is supported.");
    }

    let mut itemfn: ItemFn = syn::parse(input).expect("failed to parse item");

    // replace print
    replace_print(&mut itemfn.block);

    // add stdout bufwriter
    add_stdout_bufwriter(&mut itemfn.block);

    quote!(#itemfn).into()
}

fn replace_print(block: &mut Block) {
    for stmt in &mut block.stmts {
        replace_print_stmt(stmt);
    }
}

fn replace_print_stmt(stmt: &mut Stmt) {
    let expr = match stmt {
        Stmt::Local(local) => match &mut local.init {
            Some((_eq, init)) => init.borrow_mut(),
            _ => return,
        },
        Stmt::Expr(expr) => expr,
        Stmt::Semi(expr, _) => expr,
        _ => return,
    };

    replace_print_expr(expr);
}

fn replace_print_expr(expr: &mut Expr) {
    macro_rules! rbox {
        ($i:ident) => {
            rbox!($i.expr);
        };

        ($e:expr) => {
            replace_print_expr($e.borrow_mut())
        };
    }

    macro_rules! riter {
        ($i:expr) => {
            $i.iter_mut().for_each(replace_print_expr)
        };
    }

    let mac = match expr {
        Expr::Macro(mac) => &mut mac.mac,

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
            replace_print(&mut i.then_branch);
            if let Some((_, e)) = &mut i.else_branch {
                rbox!((e));
            }
            return;
        }
        Expr::While(i) => {
            rbox!(i.cond);
            replace_print(&mut i.body);
            return;
        }
        Expr::ForLoop(i) => {
            rbox!(i);
            replace_print(&mut i.body);
            return;
        }
        Expr::Loop(i) => return replace_print(&mut i.body),
        Expr::Match(i) => return rbox!(i),
        Expr::Closure(i) => return rbox!(i.body),
        Expr::Unsafe(i) => return replace_print(&mut i.block),
        Expr::Block(i) => return replace_print(&mut i.block),
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
                .for_each(|f| replace_print_expr(&mut f.expr));
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
        Expr::Async(i) => return replace_print(&mut i.block),
        Expr::TryBlock(i) => return replace_print(&mut i.block),
        Expr::Yield(i) => {
            if let Some(e) = &mut i.expr {
                rbox!((e));
            }
            return;
        }
        _ => return,
    };

    macro_rules! path {
        ($($tt:tt)*) => {
            syn::parse::<Path>(quote!($($tt)*).into())
                .expect("failed to parse path")
        };
    }

    // replace print! -> write!
    let path_print = vec![path!(::std::print), path!(std::print), path!(print)];

    let path_println = vec![path!(::std::println), path!(std::println), path!(println)];

    let modify_to = if path_print.contains(&mac.path) {
        Some(path!(std::write))
    } else if path_println.contains(&mac.path) {
        Some(path!(std::writeln))
    } else {
        None
    };

    if let Some(modify_to) = modify_to {
        replace(&mut mac.path, modify_to);

        let tts = mac.tts.clone();
        replace(&mut mac.tts, quote!(__proconio_stdout, #tts));

        let mac = mac.clone();
        replace(expr, parse_quote!(#mac.unwrap()));
    }
}

fn add_stdout_bufwriter(block: &mut Block) {
    let replaced: Block = parse_quote! {{
        use std::io::Write as _;
        let __proconio_stdout = std::io::stdout();
        let mut __proconio_stdout = std::io::BufWriter::new(__proconio_stdout.lock());
        let __proconio_res = #block;
        __proconio_stdout.flush().unwrap();
        return __proconio_res;
    }};

    replace(block, replaced);
}
