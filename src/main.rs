#[macro_use]
extern crate nom;

use nom::IResult;

#[derive(PartialEq, Eq, Debug)]
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

// TODO: extract token
/// &str -> Inst
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
    ws!(alt!(
        tag!(">") | tag!("<") | tag!("+") | tag!("-") | tag!(".") | tag!(",") | tag!("[") |
            tag!("]") |
            map!(nom::alphanumeric, |x| -> &[u8] { b"_" })
    ))
);

// TODO: if error occurs or invalid form is come, raise error
// REF: https://qiita.com/tatsuya6502/items/cd41599291e2e5f38a4a#%E3%82%A8%E3%83%A9%E3%83%BC%E6%83%85%E5%A0%B1%E3%82%92%E6%94%B9%E5%96%84%E3%81%99%E3%82%8B
fn parse_symbol(input: &[u8]) -> Option<(Inst, &[u8])> {
    match parser(input) {
        IResult::Done(rest, token) => Some((from_bytes(token), rest)),
        IResult::Incomplete(_) => None,
        IResult::Error(_) => panic!("Parse Error!"),
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

struct Processor {
    sp: usize,
    ip: usize,
    memory: [u8; 256],
}

impl Processor {
    fn new() -> Processor {
        Processor {
            sp: 0,
            ip: 0,
            memory: [0; 256],
        }
    }

    fn set_mem(&mut self, val: u8) {
        self.memory[self.sp] = val;
    }

    fn get_mem(&self) -> u8 {
        self.memory[self.sp]
    }

    fn pinc(&mut self) {
        self.sp += 1;
        self.ip += 1;
    }

    fn pdec(&mut self) {
        self.sp -= 1;
        self.ip += 1;
    }

    fn inc(&mut self) {
        let val = self.get_mem() + 1;
        self.set_mem(val);
        self.ip += 1;
    }

    fn dec(&mut self) {
        let val = self.get_mem() - 1;
        self.set_mem(val);
        self.ip += 1;
    }

    fn put(&mut self) {
        print!("{}", self.get_mem() as char);
        self.ip += 1;
    }

    fn get(&mut self) {
        use std::io::*;
        // ignore every error
        self.set_mem(stdin().bytes().nth(0).unwrap().unwrap());
        self.ip += 1;
    }

    fn bgn(&mut self, exec: &Vec<Inst>) {
        if self.get_mem() == 0 {
            let len = exec.len();
            self.ip += 1;

            // TODO: need some error
            while self.ip < len {
                if exec[self.ip] == Inst::End {
                    self.ip += 1;
                    break;
                } else {
                    self.ip += 1;
                }
            }
        } else {
            self.ip += 1;
        }
    }

    fn end(&mut self, exec: &Vec<Inst>) {
        if self.get_mem() != 0 {
            let mut nest_level = 0;
            self.ip -= 1;

            loop {
                let inst = &exec[self.ip];
                if inst == &Inst::Bgn {
                    if nest_level == 0 {
                        self.ip += 1;
                        break;
                    } else {
                        nest_level -= 1;
                    }
                } else if inst == &Inst::End {
                    nest_level += 1;
                } else {
                    self.ip -= 1;
                }
            }
        } else {
            self.ip += 1;
        }
    }

    fn nop(&mut self) {
        // this means inst that do nothing
        self.ip += 1;
    }

    fn exec(&mut self, exec: &Vec<Inst>) {
        use Inst::*;

        let len = exec.len();

        while self.ip < len {
            match &exec[self.ip] {
                &PInc => self.pinc(),
                &PDec => self.pdec(),
                &Inc => self.inc(),
                &Dec => self.dec(),
                &Put => self.put(),
                &Get => self.get(),
                &Bgn => self.bgn(exec),
                &End => self.end(exec),
                &Nop => self.nop(),
            }
        }
    }
}

fn main() {
    let mut machine = Processor::new();
    let hello_world = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.------------.<++++++++.--------.+++.------.--------.>+.";

    machine.exec(&parse(hello_world));
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

#[test]
fn test_from_str() {
    assert_eq!(from_str(">"), Inst::PInc);
    assert_eq!(from_str("-"), Inst::Dec);
    assert_eq!(from_str("Foo"), Inst::Nop);
}

#[test]
fn test_parse() {
    let input = "><<<<".as_bytes();
    assert_eq!(
        parse_symbol(input).unwrap(),
        (Inst::PInc, "<<<<".as_bytes())
    );
}

#[test]
#[should_panic(expected = "Parse Error!")]
fn test_parse_panic() {
    let input = "test".as_bytes();
    parse_symbol(input);
}

#[test]
fn test_machine_new() {
    let machine = Processor::new();
    assert_eq!(machine.sp, 0);
    // assert_eq!(machine.memory, [0; 256]); type check
}

#[test]
fn test_machine_pinc() {
    let mut m = Processor::new();
    m.pinc();
    assert_eq!(m.sp, 1);
}

#[test]
fn test_machine_inc() {
    let mut m = Processor::new();
    m.inc();
    assert_eq!(m.memory[0], 1);
}
