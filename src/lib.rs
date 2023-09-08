use std::io::{BufRead, BufReader, Read, Write};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};


// Workaround for trait aliases
// https://users.rust-lang.org/t/why-cant-type-aliases-be-used-for-traits/10002/8
// https://github.com/rust-lang/rfcs/pull/1733#issuecomment-243840014
pub trait Transformer: for<'a> Fn(&'a str) -> Option<String> {}
impl<T: for<'a> Fn(&'a str) -> Option<String>> Transformer for T {}

pub fn handle<R: Read, W: Write, T: Fn(&str) -> Option<String>>(
    reader: R,
    writer: W,
    transformer: T,
) {

    let reader = Arc::new(Mutex::new(reader));
    let writer = Arc::new(Mutex::new(writer));

    let mut r = reader.lock().unwrap();
    let mut buf_reader: BufReader<&mut R> = BufReader::new(r.deref_mut());

    // let mut handles = Vec::new();
    // for _ in 1..5 {
    //     let v = Arc::clone(&v);
    //     handles.push(thread::spawn(move || {
    //         let thread_id = thread::current().id();
    //         println!("{thread_id:?}: {v:?}");
    //     }));
    // }

    let mut line = String::new();
    loop {
        match buf_reader.read_line(&mut line) {
            Ok(0) => {
                break;
            } // EOF
            Ok(_) => {
                // When the transformer returns a None it means we print nothing.
                if let Some(text) = transformer(line.trim_end_matches('\n')) {
                    let transformed = text + &'\n'.to_string();
                    writer
                        .lock()
                        .unwrap()
                        .write_all(transformed.as_bytes())
                        .expect("Failed to write");
                };
            }
            Err(error) => {
                panic!("Error reading from stdin: {error}");
            }
        }

        line.clear();
    }
}

#[cfg(test)]
mod test_reversal {
    use super::*;
    use std::io::Cursor;

    const REVERSER: fn(&str) -> Option<String> =
        |line: &str| Some(line.chars().rev().collect::<String>());

    #[test]
    fn test_single_line() {
        let mut input = Cursor::new(b"12345");
        let mut output: Vec<u8> = vec![];

        handle(&mut input, &mut output, REVERSER);
        assert_eq!("54321\n".as_bytes(), output);
        let mut remaining: String = String::new();
        input.read_to_string(&mut remaining).unwrap();
        assert_eq!("", remaining);
    }

    #[test]
    fn test_with_endline() {
        let mut input = Cursor::new(b"abc\n");
        let mut output: Vec<u8> = vec![];

        handle(&mut input, &mut output, REVERSER);
        assert_eq!("cba\n".as_bytes(), output);
    }

    #[test]
    fn test_multiline() {
        let mut input = Cursor::new(b"ab\ncd");
        let mut output: Vec<u8> = vec![];

        handle(&mut input, &mut output, REVERSER);
        assert_eq!("ba\ndc\n".as_bytes(), output);
    }
}

#[cfg(test)]
mod test_conditional_output {
    use super::*;
    use std::io::Cursor;

    const LINE_FILTER: fn(&str) -> Option<String> = |line| {
        if line.len() % 2 == 0 {
            Some(line.to_string())
        } else {
            None
        }
    };

    #[test]
    fn test_single_line() {
        let mut input = Cursor::new(b"12\n123\n12");
        let mut output: Vec<u8> = vec![];

        handle(&mut input, &mut output, LINE_FILTER);
        assert_eq!("12\n12\n".as_bytes(), output);
    }
}
