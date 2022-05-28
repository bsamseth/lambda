#![allow(dead_code)]
#![allow(unused_variables)]

use lambda::parser::ParseResult;

fn try_parse(code: &str) {}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = args.join(" ");
    let parsed: ParseResult = code.parse();

    println!(
        "'{}' -> {}",
        code,
        parsed.map_or_else(|e| format!("{}", e), |p| format!("{}", p),),
    );
}
