use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Float(Float),
    // Keyword(String),
    Str(String),
    Arr(Vec<Atom>),
    BuiltIn(BuiltIn),
}

impl std::ops::Add for Atom {
    type Output = Self;
    fn add(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs + rhs),
            (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't add those types"),
        }
    }
}

impl std::ops::Sub for Atom {
    type Output = Self;
    fn sub(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs - rhs),
            (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't subtract those types"),
        }
    }
}

impl std::ops::Mul for Atom {
    type Output = Self;
    fn mul(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs * rhs),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't multiply those types"),
        }
    }
}

impl std::ops::Div for Atom {
    type Output = Self;
    fn div(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs / rhs),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't divide those types"),
        }
    }
}

impl Atom {
    fn pow(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs.powf(rhs)),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't raise those types"),
        }
    }

    fn modu(self, rhs: Atom) -> Self {
        match (self, rhs) {
            (Atom::Float(lhs), Atom::Float(rhs)) => Atom::Float(lhs % rhs),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            (_, _) => panic!("I can't mod those types"),
        }
    }

    fn fact(self) -> Self {
        match self {
            Atom::Float(lhs) => Atom::Float(factorial(lhs) as Float),
            // (Atom::Str(lhs), Atom::Str(rhs)) => Atom::Str(format!("{}{}", lhs, rhs)),
            _ => panic!("I can't factorial those types"),
        }
    }
}

impl std::fmt::Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Atom::Str(st) => st.to_string(),
                Atom::Float(f) => format!("{}", f),
                Atom::Arr(f) => {
                    f.iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<String>>()
                        .join(" ")
                }
                Atom::BuiltIn(bi) => match bi {
                    _ => "BuiltIn".to_string(),
                },
            }
            .to_string()
        )
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
    Negate,
    Abs,
    Range,
    Sum,
    Avg,

    // Stack operators
    Len,
    Swap,
    Dupe,
    Drop,
    Clear,
    Jmp,

    // Keywords
    Not,
    Print,
    PrintLn,
    Cmp,
    Dcmp,
    InChar,
}

impl BuiltIn {
    pub fn call(&self, stack: &mut Vec<Atom>, pc: &mut usize) {
        use BuiltIn::*;
        match self {
            // Operators
            Plus => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a + b);
                }
            }
            Minus => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a - b);
                }
            }
            Times => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a * b);
                }
            }
            Divide => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a / b);
                }
            }
            Equal => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(if a == b {
                        Atom::Float(1.0)
                    } else {
                        Atom::Float(0.0)
                    });
                }
            }
            Power => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a.pow(b));
                }
            }
            Root => {
                if let Some(Atom::Float(b)) = stack.pop() {
                    stack.push(Atom::Float(b.sqrt()));
                }
            }
            Modulus => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(a.modu(b));
                }
            }
            Factorial => {
                if let Some(a) = stack.pop() {
                    stack.push(a.fact());
                }
            }
            Negate => {
                if let Some(Atom::Float(a)) = stack.pop() {
                    stack.push(Atom::Float(-1.0 * a));
                }
            }
            Abs => {
                if let Some(Atom::Float(a)) = stack.pop() {
                    stack.push(Atom::Float(a.abs()))
                }
            }
            Range => {
                if let (Some(Atom::Float(_)), Some(Atom::Float(_))) = (stack.last(), stack.last()) {
                    let b = match stack.pop() {
                        Some(Atom::Float(a)) => a,
                        None => 1.0,
                        _ => 0.0,
                    };
                    let a = match stack.pop() {
                        Some(Atom::Float(a)) => a,
                        None => 1.0,
                        _ => 0.0,
                    };

                    stack.push(Atom::Arr(
                        ((a as usize)..((b + 1.0) as usize))
                            .into_iter()
                            .map(|x| Atom::Float(x as f64))
                            .collect::<Vec<Atom>>(),
                    ));
                }
            }
            Sum => {
                if let Some(Atom::Float(a)) = stack.pop() {
                    let mut total = 0.0;
                    for _ in 0..(a as usize) {
                        total += match stack.pop() {
                            Some(atom) => match atom {
                                Atom::Float(f) => f,
                                _ => 0.0
                            },
                            _ => 0.0
                        }
                    }
                    stack.push(Atom::Float(total));
                }
                // let mut total = 0.0;
                // for atom in stack {
                //     match atom {
                //         Atom::Float => total += total;
                //     }
                // }
            }
            Avg => { 
                if let Some(Atom::Float(a)) = stack.pop() { 
                    let mut total = 0.0;
                    for _ in 0..(a as usize) {
                        total += match stack.pop() {
                            Some(atom) => match atom {
                                Atom::Float(f) => f,
                                _ => 0.0
                            },
                            _ => 0.0
                        }
                    }
                    stack.push(Atom::Float(total / a));
                }
            }
            Len => stack.push(Atom::Float(stack.len() as f64)),
            Swap => {
                if let (Some(b), Some(a)) = (stack.pop(), stack.pop()) {
                    stack.push(b);
                    stack.push(a);
                }
            }
            Dupe => {
                if let Some(a) = stack.last() {
                    stack.push(a.clone())
                }
            }
            Drop => if let Some(_) = stack.pop() {},
            Clear => {
                *stack = Vec::new();
            }
            Jmp => {
                if let Some(Atom::Float(a)) = stack.pop() {
                    *pc = {
                        let target: usize = a.clone() as usize;
                        target
                    };
                }
            }

            Not => {
                if let Some(Atom::Float(bo)) = stack.pop() {
                    stack.push(Atom::Float(match bo as usize {
                        1 => 0.0,
                        0 => 1.0,
                        _ => 1.0,
                    }))
                }
            }
            PrintLn => {
                if let Some(first_elem) = stack.pop() {
                    println!("{}", first_elem)
                }
            }
            Print => {
                if let Some(first_elem) = stack.pop() {
                    print!("{} ", first_elem)
                }
            }
            Cmp => {
                if let Some(Atom::Str(first_elem)) = stack.pop() {
                    stack.push(Atom::Str(unsafe {
                        String::from_utf8_unchecked(compress(first_elem.as_bytes()))
                    }))
                }
            }
            Dcmp => {
                if let Some(Atom::Str(sym_str)) = stack.pop() {
                    let atom = Atom::Str(
                        String::from_utf8(decompress(sym_str.as_bytes()).unwrap()).unwrap(),
                    );
                    stack.push(atom);
                }
            }
            InChar => {}
        }
    }
}
