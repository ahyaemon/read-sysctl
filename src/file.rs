use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_exists() {
        let actual = read_lines("resources/test/file/test.txt").unwrap();
        let li: Vec<String> = actual.flatten().collect();
        assert_eq!(li, ["a", "", "bb"]);
    }

    #[test]
    #[should_panic]
    fn file_not_exists() {
        read_lines("resources/xxx.txt").unwrap();
    }
}
