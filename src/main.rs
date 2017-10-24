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
    Bgn,
    End,
    Nop,
}

impl std::fmt::Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let repr = match self {
            PInc => "pinc",
            PDec => "pdec",
            Inc => "inc",
            Dec => "dec",
            Put => "put",
            Get => "get",
            Bgn => "bgn",
            End => "end",
            Nop => "nop",
        };

        write!(f, "{}", repr)
    }
}

named!(
    parser,
    alt!(tag!(">") | tag!("<") | tag!("+") | tag!("-") | tag!(".") | tag!(",") | tag!("[") | tag!("]"))
);

fn str2inst(symbol: &str) -> Inst {
    match symbol {
        ">" => Inst::PInc,
        "<" => Inst::PDec,
        "+" => Inst::Inc,
        "-" => Inst::Dec,
        "." => Inst::Put,
        "," => Inst::Get,
        "[" => Inst::Bgn,
        "]" => Inst::End,
        _ => Inst::Nop,
    }
}

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

    let (inst, rest) = parse(sample);

    println!("{}", str2inst(std::str::from_utf8(rest).unwrap()));
}
