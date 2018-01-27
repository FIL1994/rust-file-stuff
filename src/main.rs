use std::error::Error;
use std::io::prelude::*;
use std::fs::{self, File};
use std::path::Path;

#[macro_use]
extern crate clap;
use clap::{App};

static LOREM_IPSUM: &'static str =
    "Lorem ipsum dolor sit amet, consectetur adipisicing elit, sed do eiusmod
    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam,
    quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo
    consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse
    cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non
    proident, sunt in culpa qui officia deserunt mollit anim id est laborum.
    ";

fn main() {
    // allows user to use -h or --help (e.g. cargo run -- -h)
    let matches = App::new("rust-file-stuff")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Reads and writes files")
        // .before_help("something before")
        // .after_help("something after")
        .get_matches();

    // READ FILE

    // create a path to the file
    let path = Path::new("hello.txt");
    let display = path.display();

    // open the path in read-only mode, returns 'io::Result<File>'
    let mut file = match File::open(&path) {
        // the 'description' method of 'io::Error' returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // read the file contents into a string, returns 'io::Result<usize>'
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
    // 'file' goes out of scope, and the "hello.txt" file gets closed
    // READ FILE END

    // CREATE FILE
    let new_path = Path::new("out/lorem_ipsum.txt");
    let new_display = new_path.display();

    // open a file in write-only mode, returns 'io::Result<File>'
    let mut file = match File::create(&new_path) {
        Err(why) => panic!("couldn't create {}: {}", new_display, why.description()),
        Ok(file) => file,
    };

    // write the 'LOREM_IPSUM' string to 'file', returns 'io::Result<()>'
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", new_display, why.description())
        },
        Ok(_) => println!("successfully wrote to {}", new_display),
    }
    // END CREATE FILE

    // print contents of directory
    match print_dir_contents() {
        Ok(s) =>  println!("{}", s),
        Err(e) => println!("Error: {}", e.to_string()),
    }

    // check if a directory called src exists
    println!("src exists: {}", (Path::new("src")).is_dir());

    // try to create a new directory
    match fs::create_dir("some/two/four") {
        Ok(_) => println!("Successfully created directory"),
        Err(e) => println!("Eror: {}", e), // directory probably exists already
    }
}

fn print_dir_contents() -> Result<String, Box<Error>> {
    // Open path
    let dir = Path::new("src");

    // Check if it is a directory
    if !dir.is_dir() {
        return Err(Box::from("Is not a directory!"));
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let file_name = path.file_name().unwrap();
        println!("{}", file_name.to_string_lossy());
    }

    Ok("Done".into())
}