#[macro_use]
extern crate nom;

use nom::{IResult, space, alpha};

named!(parens, delimited!(char!('('), is_not!(")"), char!(')')));

named!(test1(&[u8]) -> &[u8], tag!("Hello,"));
named!(test2<&[u8], &[u8]>, tag!("Hello,"));
named!(test3, tag!("Hello,"));

fn test4(input: &[u8]) -> IResult<&[u8], &[u8]> {
    tag!(input, "Hello,")
}

// test1~4 are the same functionally

fn main() {
    let sample = "Hello, world!\nHello, hello";

    let tes: &[u8] = &[0; 4];

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
