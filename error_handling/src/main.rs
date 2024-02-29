// use core::panic;
use std::fs::read_to_string;
// use std::io::ErrorKind;
use std::io::{self, Read};

// Propagating errors Method #1
// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("username.txt");
//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();
//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
// }

// Propagating errors Method #2
fn read_username_from_file() -> Result<String, io::Error> {
    // 1st method
    // let mut username_file = File::open("username.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // 2nd method
    // let mut username = String::new();
    // File::open("username.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    // 3rd method
    read_to_string("username.txt")
}

fn main() {
    // #1 Method
    // let greetings_file_result = File::open("hello.txt");
    // let greetings_file = match greetings_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(er) => panic!("The file cannot be created, {:?}", er),
    //         },
    //         _ => panic!("Not sure about the error though"),
    //     },
    // };

    // 2nd Method
    // let greetings_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|err| {
    //             panic!("The file cannot be created for some reason, {:?}", err);
    //         })
    //     } else {
    //         panic!("Not sure about the error though");
    //     }
    // });
    // println!("{:?}", greetings_file);

    // Propagating errors Method #1
    // let result = read_username_from_file();
    // println!("{:?}", result);

    // Propagating errors Method #2
    let result = read_username_from_file();
    println!("{:?}", result);
}
