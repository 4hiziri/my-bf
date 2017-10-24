#[macro_use]
extern crate nom;

use nom::{IResult, space, alpha};

enum Inst {
    PInc,
    PDec,
    Inc,
    Dec,
    Put,
    Get,
    Begin,
    End,
}

named!(
    pinc,
    alt!(tag!(">") | tag!("<") | tag!("+") | tag!("-") | tag!(".") | tag!(",") | tag!("[") | tag!("]"))
);

fn parse(input: &[u8]) -> (Inst, &[u8]) {}

// Load all instruction at beginning?

fn main() {
    let sample = "++++++++++++++++++++++++++.";
    let mut pointer: u8 = 0;
    let mut memory: [u8; 256] = [0; 256];


    match pinc(sample.as_bytes()) {
        IResult::Done(i, o) => {
            println!(
                "{}, {}",
                String::from_utf8(i.to_vec()).unwrap(), // world!
                String::from_utf8(o.to_vec()).unwrap(), // Hello, 
            )
        }
        _ => println!("Other!\n"),
    };
}
