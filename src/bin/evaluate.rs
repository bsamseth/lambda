#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = args.join(" ");
    let parsed: lambda::ParseResult = code.parse();

    println!(
        "'{}' -> {}",
        code,
        parsed.map_or_else(|e| format!("{}", e), |p| format!("{}", p.evaluate()))
    );
}
