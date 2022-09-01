use std::error::Error;
use std::fs::File;

fn main() {
    open_panic()
    // open_panic1()
}

fn open_panic() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn open_panic1() {
    let greeting_file = File::open("hello.txt").expect("Problem opening the file!");
}

/*
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}

*/
