use minilisp::{convert, lexer, parser, stdlib};
use rustyline::Editor;
use std::{env, fs};

fn eval_files(files: Vec<String>) {
    let mut env = stdlib::stdlib();
    for file in files.iter() {
        let content = fs::read_to_string(file);
        assert!(content.is_ok(), "Could not read file '{}'", file);
        let result = eval(content.unwrap(), &mut env);
        assert!(result.is_ok(), "Could not evaluate file '{}'", file);
        println!("{}", result.unwrap());
    }
}

fn interactive() {
    let mut rl = Editor::<()>::new();
    let mut env = stdlib::stdlib();

    'repl: loop {
        let readline = rl.readline(">> ");
        let mut line;
        match readline {
            Ok(l) => {
                line = l.clone();
                loop {
                    match eval(line.clone(), &mut env) {
                        Ok(result) => {
                            println!("{}", result);
                            break;
                        }
                        Err(_) => match rl.readline(".. ") {
                            Ok(l) => {
                                line.push('\n');
                                line.push_str(&l);
                            }
                            Err(_) => {
                                println!("An error occurred");
                                break 'repl;
                            }
                        },
                    }
                }
            }
            Err(_) => {
                println!("An error occurred");
                break;
            }
        }
        rl.add_history_entry(line);
    }
}

fn eval(
    input: String,
    env: &mut stdlib::Environment,
) -> Result<String, parser::error::ParserError> {
    let data = convert::convert(parser::parse(&mut lexer::lex(&input).unwrap())?);

    Ok(data
        .iter()
        .map(|d| {
            let answer = stdlib::eval(d, env).expect("Could not evaluate the input");
            if let stdlib::EnvItem::Data(a) = answer {
                format!("{}", a)
            } else {
                format!("{:?}", answer)
            }
        })
        .collect::<Vec<String>>()
        .join("\n"))
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if !args.is_empty() {
        eval_files(args);
    } else {
        interactive();
    }
}
