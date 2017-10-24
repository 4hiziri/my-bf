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
    parser,
    alt!(tag!(">") | tag!("<") | tag!("+") | tag!("-") | tag!(".") | tag!(",") | tag!("[") | tag!("]"))
);

fn parse(input: &str) -> (Inst, &[u8]) {
    match parser(input.as_bytes()) {
        IResult::Done(i, o) => (Inst::End, o),
        _ => (Inst::End, &[]),
    }
}

// Load all instruction at beginning?

fn main() {
    let sample: &str = "++++++++++++++++++++++++++.";
    let mut pointer: u8 = 0;
    let mut memory: [u8; 256] = [0; 256];

    // match parser(sample.as_bytes()) {
    //     IResult::Done(i, o) => {
    //         println!(
    //             "{}, {}",
    //             String::from_utf8(i.to_vec()).unwrap(),
    //             String::from_utf8(o.to_vec()).unwrap(),
    //         )
    //     }
    //     _ => println!("Other!\n"),
    // };

    let (inst, rest) = parse(sample);

    println!("{}", std::str::from_utf8(rest).unwrap());
}
