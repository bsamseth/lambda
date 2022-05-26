#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let bad = lambda::lexer::lex("invalid/");
    println!("{:?}", bad);

    // let tokens = lambda::lexer::lex("\\x.λfoo. x foo");
    let tokens = lambda::lexer::lex("x (y z)");
    println!("{:?}", tokens);

    println!("{:?}", lambda::parser::parse(&tokens));
}
