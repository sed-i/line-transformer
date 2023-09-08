/// Print only even-length lines (omit lines whose length is odd).
fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    const LINE_FILTER: fn(&str) -> Option<String> = |line| {
        if line.len() % 2 == 0 {
            Some(line.to_string())
        } else {
            None
        }
    };

    line_transformer::handle(&mut stdin.lock(), &mut stdout.lock(), LINE_FILTER);
}
