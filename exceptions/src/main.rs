use core::panic;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hallo.txt")
        .expect("Die Datei fehlt.");
        
}

