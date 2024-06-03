use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    let mut file = File::create(file).expect("cannot create or open");
    file.write_all(content.as_bytes())
        .expect("cannot write");
}

fn main() {
    let path = "a.txt";
    File::create(path).unwrap();
    open_or_create(path, "content to be written");

    let mut file = File::open(path).unwrap();

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("{}", s);
}

// And its output:

// $ cargo run
// content to be written
// $
