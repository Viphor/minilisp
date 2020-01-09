use minilisp::{convert, lexer, parser, stdlib, vm};
use rustyline::Editor;
use std::{env, fs};

fn eval_files(files: Vec<String>) {
    let mut machine = vm::Machine::default();
    for file in files.iter() {
        let content = fs::read_to_string(file);
        assert!(content.is_ok(), "Could not read file '{}'", file);
        let result = eval(content.unwrap(), &mut machine);
        assert!(result.is_ok(), "Could not evaluate file '{}'", file);
        println!("{}", result.unwrap());
    }
}

fn interactive() {
    let mut rl = Editor::<()>::new();
    let mut machine = vm::Machine::default();

    'repl: loop {
        let readline = rl.readline(">> ");
        let mut line;
        match readline {
            Ok(l) => {
                line = l.clone();
                loop {
                    match eval(line.clone(), &mut machine) {
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

fn eval(input: String, machine: &mut vm::Machine) -> Result<String, parser::error::ParserError> {
    let data = convert::convert(parser::parse(&mut lexer::lex(&input).unwrap())?);

    Ok(data
        .into_iter()
        .map(|d| {
            let answer = machine
                .eval(d.clone())
                .expect("Could not evaluate the input");
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
