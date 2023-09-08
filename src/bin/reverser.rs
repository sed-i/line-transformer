fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    const REVERSER: fn(&str) -> Option<String> =
        |line: &str| Some(line.chars().rev().collect::<String>());

    line_transformer::handle(stdin, stdout, REVERSER);
}
