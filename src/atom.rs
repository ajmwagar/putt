use super::*;
// struct Atom {
//     args: u8,
//     call: Call
// }

// impl Atom {
//     pub fn new(args: u8, call: Call) -> Self {
//         Atom {
//             args,
//             call
//         }
//     }
// }

// enum Call {
//     Lambda(Box<dyn FnMut(Box<Vec<&mut Atom>>, Box<Vec<&mut Atom>>)>),
//     Op(Op)
// }

// enum Op {
//     Add,
//     Sub,
//     Multi,
//     Divis,
//     Factorial,
//     Modulus,
//     Equal,
//     Power
// }

// const Atoms: [(&str, &Atom); 1] = [
//     ("?", &Atom::new(3, Call::Lambda(Box::new(|stack: Box<Vec<Atom>>, (a, b, c)| { if {stack.push(b)} else {stack.push(a)}))),
// ];

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Num(Num),
    Float(Float),
    Keyword(String),
    Str(String),
    Boolean(bool),
    Arr(Vec<Atom>),
    BuiltIn(BuiltIn),
}

impl std::ops::Add for Atom {
    type Output = Self;
    fn add(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Num(lhs), Atom::Num(rhs)) => Atom::Num(lhs + rhs),
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs + rhs),
            (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't add those types")
        }
    }
}

impl std::ops::Sub for Atom {
    type Output = Self;
    fn sub(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Num(lhs), Atom::Num(rhs)) => Atom::Num(lhs - rhs),
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs - rhs),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't subtract those types")
        }
    }
}

impl std::ops::Mul for Atom {
    type Output = Self;
    fn mul(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Num(lhs), Atom::Num(rhs)) => Atom::Num(lhs * rhs),
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs * rhs),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't multiply those types")
        }
    }
}

impl std::ops::Div for Atom {
    type Output = Self;
    fn div(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Num(lhs), Atom::Num(rhs)) => Atom::Num(lhs / rhs),
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs / rhs),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't divide those types")
        }
    }
}

impl Atom {
    fn pow(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Num(lhs), Atom::Num(rhs)) => Atom::Num(lhs.pow(rhs as u32)),
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs.powf(rhs)),
            (Atom::Num(lhs), Atom::Float(rhs)) => Atom::Float((lhs as Float).powf(rhs)),
            (Atom::Float(lhs), Atom::Num(rhs)) => Atom::Float(lhs.powf(rhs as Float)),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't raise those types")
        }
    }

    fn modu(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Num(lhs), Atom::Num(rhs)) => Atom::Num(lhs % rhs),
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs % rhs),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't mod those types")
        }
    }

    fn fact(self) -> Self {
        match self {
            Atom::Num(lhs)=> Atom::Num(factorial(lhs)),
            Atom::Float(lhs)=> Atom::Float(factorial(lhs as Num) as Float),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            _ => panic!("I can't factorial those types")
        }
    }
}


impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match &self {
            Atom::Num(num) => num.to_string(),
            Atom::Boolean(bo) => bo.to_string(),
            Atom::Str(st) => st.to_string(),
            Atom::Float(f) => format!("{}", f),
            Atom::Keyword(f) => format!("{}", f),
            Atom::BuiltIn(bi) => match bi {
                _ => unreachable!()
            },
            _ => "".to_string()
        } .to_string())
    }

}

/// Starting from the most basic, we define some built-in functions that our lisp has
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BuiltIn {
    // Operators
    Plus,
    Minus,
    Times,
    Divide,
    Equal,
    Power,
    Root,
    Modulus,
    Factorial,

    // Keywords
    Not,
    Print,
    PrintLn,
    Cmp,
    Dcmp

}

impl BuiltIn {
    pub fn call(&self, stack: &mut Vec<Atom>) {
        use BuiltIn::*;
        match self {
            // Operators
            Plus => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a + b);
                }
            },
            Minus => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a - b);
                }
            },
            Times => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a * b);
                }
            },
            Divide => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a / b);
                }
            },
            Equal => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(Atom::Boolean(a == b));
                }
            },
            Power => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a.pow(b));
                }
            },
            Root => {
                if let Some(Atom::Num(b)) = stack.pop() {
                    stack.push(Atom::Float((b as Float).sqrt()));
                }
            },
            Modulus => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a.modu(b));
                }
            },
            Factorial => {
                if let Some(a) = stack.pop() {
                    stack.push(a.fact());
                }
            },

            // Keywords
            Not => {
                if let Some(Atom::Boolean(bo)) = stack.pop() {
                    stack.push(Atom::Boolean(!bo));
                }
            },
            PrintLn => {
                if let Some(first_elem) = stack.pop() {
                    println!("{}", first_elem)
                }
            }, 
            Print => {
                if let Some(first_elem) = stack.pop() {
                    print!("{} ", first_elem)
                }
            },
            Cmp => {
                if let Some(Atom::Str(first_elem)) = stack.pop() {
                    stack.push(Atom::Str(unsafe {String::from_utf8_unchecked(compress(first_elem.as_bytes()))}))
                }
            },
            Dcmp => {
                if let Some(Atom::Str(sym_str)) = stack.pop() {
                    let atom = Atom::Str(String::from_utf8(decompress(sym_str.as_bytes()).unwrap()).unwrap());
                    stack.push(atom);
                }
            }
        }
    }
}
