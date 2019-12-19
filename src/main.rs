extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(".lang_history").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let expr = lang::ExprParser::new().parse(line.as_str());
                match expr {
                    Ok(result) => {
                        println!("{:?}", result);
                    },
                    Err(reason) => {
                        println!("Error: {:?}", reason);
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("^D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(".lang_history").unwrap();
}

pub mod ast;
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub lang);
#[test]
fn lang() {
    let expr = lang::ExprParser::new()
        .parse("22 * 44 + 66")
        .unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
