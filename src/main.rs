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
            &Inst::PInc => "pinc",
            &Inst::PDec => "pdec",
            &Inst::Inc => "inc",
            &Inst::Dec => "dec",
            &Inst::Put => "put",
            &Inst::Get => "get",
            &Inst::Bgn => "bgn",
            &Inst::End => "end",
            &Inst::Nop => "nop",
        };

        write!(f, "{}", repr)
    }
}

impl std::fmt::Debug for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

fn from_str(symbol: &str) -> Inst {
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

fn from_bytes(symbol: &[u8]) -> Inst {
    from_str(std::str::from_utf8(symbol).unwrap())
}

named!(
    parser,
    alt!(tag!(">") | tag!("<") | tag!("+") | tag!("-") | tag!(".") | tag!(",") | tag!("[") | tag!("]"))
);

// TODO: if error occurs or invalid form is come, raise error
fn parse_symbol(input: &[u8]) -> Option<(Inst, &[u8])> {
    match parser(input) {
        IResult::Done(rest, token) => Some((from_bytes(token), rest)),
        _ => None,
    }
}

fn parse(input: &str) -> Vec<Inst> {
    let mut v = Vec::with_capacity(input.len());
    let mut rest = input.as_bytes();

    while let Some((inst, next)) = parse_symbol(rest) {
        rest = next;
        v.push(inst);
    }

    v
}

fn main() {
    let sample: &str = "+++++++++.";
    let mut pointer: u8 = 0;
    let mut memory: [u8; 256] = [0; 256];

    let v = parse(sample);

    println!("{:?}", v);
}

#[test]
fn test_inst_string() {
    assert_eq!(Inst::PInc.to_string(), "pinc");
    assert_eq!(Inst::PDec.to_string(), "pdec");
    assert_eq!(Inst::Inc.to_string(), "inc");
    assert_eq!(Inst::Dec.to_string(), "dec");
    assert_eq!(Inst::Put.to_string(), "put");
    assert_eq!(Inst::Get.to_string(), "get");
    assert_eq!(Inst::Bgn.to_string(), "bgn");
    assert_eq!(Inst::End.to_string(), "end");
    assert_eq!(Inst::Nop.to_string(), "nop");
}
