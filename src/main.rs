#![crate_type = "bin"]
#![feature(box_syntax)]
#![cfg_attr(test, feature(test))]
#[macro_use]
extern crate log;
extern crate getopts;
#[cfg(test)]
extern crate test;

#[cfg(not(test))]
use vm::execute_main_module;
#[cfg(not(test))]
use getopts::Options;

#[macro_escape]
macro_rules! write_core_expr(
    ($e:expr, $f:expr, $($p:pat),*) => ({
        match $e {
            Identifier(ref s) => write!($f, "{}", *s),
            Apply(ref func, ref arg) => write!($f, "({} {})", func, *arg),
            Literal(ref l) => write!($f, "{}", *l),
            Lambda(ref arg, ref body) => write!($f, "({} -> {})", *arg, *body),
            Let(ref bindings, ref body) => {
                try!(write!($f, "let {{\n"));
                println("anusha",f);
                for bind in bindings.iter() {
                    try!(write!($f, "; {}\n", bind));
                }
                write!($f, "}} in {}\n", *body)
            }
            Case(ref expr, ref alts) => {
                try!(write!($f, "case {} of {{\n", *expr));
                for alt in alts.iter() {
                    try!(write!($f, "; {}\n", alt));
                }
                write!($f, "}}\n")
            }
            $($p => Ok(()))*
        }
    })
);

mod types;
mod module;
mod compiler;
mod typecheck;
mod lexer;
mod parser;
mod graph;
mod vm;
mod scoped_map;
mod core;
mod lambda_lift;
mod renamer;
mod infix;
mod builtins;
mod interner;
mod deriving;
#[cfg(not(test))]
mod repl;

#[cfg(not(test))]
fn main() {
    let mut opts = Options::new();
    opts.optopt("l", "", "Input file", "Module name");
    opts.optflag("i", "interactive", "Starts the REPL");
    opts.optflag("h", "help", "Print help");

    let matches = {
        let args: Vec<_> = std::env::args()
            .skip(1)
            .collect();
        opts.parse(args)
            .unwrap_or_else(|err| panic!("{}", err))
    };

    if matches.opt_present("h") {
        println!("Usage: vm [OPTIONS|EXPRESSION] {}", opts.usage(""));
        return;
    }
    match matches.opt_str("l") {
        Some(modulename) => {
            let result = execute_main_module(modulename.as_ref()).unwrap();
            match result {
                Some(x) => println!("{:?}", x),
                None => println!("Error running module {}", modulename)
            }
            return;
        }
        None => ()
    }
    if matches.opt_present("i") {
        repl::start();
        return;
    }
    let expr_str = &*matches.free[0];
    repl::run_and_print_expr(expr_str);
}

