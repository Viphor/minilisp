use minilisp::{convert, lexer, parser, stdlib};

fn main() {
    let test = "(test '(\"cool \\\"string\\\" stuff\" 123))";
    println!("String to test:\n{}\n", test);
    let mut lexed = lexer::lex(test).unwrap();
    println!("Lexer output:\n{:?}\n", lexed);
    let parsed = parser::parse(&mut lexed);
    println!("Parser output:\n{:?}\n", parsed);

    let converted = convert::convert(parsed.expect("Could not parse the input"));
    println!("Converter output:\n{:?}\n", converted);

    println!("Converted output in pretty print:");
    for i in converted.iter() {
        println!("{}", i);
    }

    println!("\n\n\n");

    //let program = "((lambda (x y) x y) 5 8)";
    //let program = "((lambda (x) (+ x x)) 5)";
    //let program = "(eval ((lambda (x) (+ x x)) 5))";
    //let program = "'((lambda (x) (+ x x)) 5)";
    let program = "(eval '((lambda (x) (+ x x)) 5))";
    //let program = "(quote true)";
    //let program = "(lambda (x) x)";
    println!("Program:\n{}", program);
    let data = convert::convert(
        parser::parse(&mut lexer::lex(program).unwrap()).expect("Could not parse the input"),
    );
    println!("Converted data:\n{}", data.first().unwrap());
    let answer = stdlib::eval(&data.first().unwrap(), &mut stdlib::stdlib())
        .expect("Could not evaluate the input");
    if let stdlib::EnvItem::Data(a) = answer {
        println!("Answer:\n{}", a);
    } else {
        println!("Answer (not pretty):\n{:?}", answer);
    }
}
