#[macro_use]
extern crate nom;

use nom::{IResult, space, alpha};



named!(point_inc, tag!(">"));
named!(point_dec, tag!("<"));
named!(inc, tag!("+"));
named!(dec, tag!("-"));
named!(output, tag!("."));
named!(input, tag!(","));

// Load all instruction at beginning?


fn main() {
    let mut pointer: u8 = 0;
    let mut memory: [u8; 256] = [0; 256];


    match test4(sample.as_bytes()) {
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
