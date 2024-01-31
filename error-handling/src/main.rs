// use std::error::Error;
// use std::fs::{read_to_string, File};
// use std::io::ErrorKind;
//
// fn main() -> Result<(), Box<dyn Error>> {
//     // let greeting_file_result = File::open("hello.txt");
//
//     // let greeting_file = match greeting_file_result {
//     //     Ok(file) => file,
//     //     Err(error) => match error.kind() {
//     //         ErrorKind::NotFound => match File::create("hello.txt") {
//     //             Ok(fc) => fc,
//     //             Err(e) => panic!("Problem creating the file: {:?}", e),
//     //         },
//     //         other_error => {
//     //             panic!("Problem opening the file: {:?}", other_error);
//     //         }
//     //     },
//     // };
//
//     use std::fs;
//     use std::io::{self, Read};
//
//     // fn read_username_from_file() -> Result<String, io::Error> {
//     //     let mut username = String::new();
//     //
//     //     File::open("hello.txt")?.read_to_string(&mut username)?;
//     //
//     //     Ok(username)
//     // }
//     // fn read_username_from_file_short() -> Result<String, io::Error> {
//     //     fs::read_to_string("hello.txt")
//     // }
//     //
//     // fn last_char_of_first_line(text: &str) -> Option<char> {
//     //     text.lines().next()?.chars().last()
//     // }
//     // let mut username = String::new();
//     // username = match read_username_from_file_short() {
//     //     Ok(string) => string,
//     //     Err(_) => String::from("error"),
//     // };
//     // let last = last_char_of_first_line(&username);
//     // last.unwrap_or('N');
//     //
//     // println!("{:?}", last);
//
//     let greeting_file = File::open("hello.txt")?;
//     Ok(())
// }

fn main() {
    use std::net::IpAddr;

    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
}
