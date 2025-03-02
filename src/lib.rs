use std::error::Error;

const DEBUG: bool = false;

use nom::error::VerboseError;

use parser::*;
use smaz::{compress,decompress};

pub mod parser;
pub mod atom;

pub type Num = i128;
pub type Float = f64;

use atom::*;


#[derive(Debug, PartialEq, Clone)] pub enum Expr {
    Constant(Atom),
    /// (arg1 arg2...func-name)
    Function(Vec<Atom>),
    /// (if predicate do-this)
    If(Box<Expr>, Box<Expr>),
    /// (if predicate do-this otherwise-do-this)
    IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
}


pub struct Putt {
    /// Memory of Putt VM
    pub stack: Vec<Atom>,

    pub src: Option<Expr>,

    /// Instructions
    pub inst: Vec<Atom>,
    /// Program counter, used for jumps
    pub pc: usize
}

impl Putt {
    /// Create a new Putt VM
    pub fn new() -> Self {
        Putt {
            stack: Vec::new(),
            src: None,
            inst: Vec::new(),
            pc: 0
        }
    }

    pub fn parse(&mut self, src: &str) -> Result<(), Box<dyn Error>> {
        parse_expr(src)
            .map_err(|e: nom::Err<VerboseError<&str>>| format!("{:#?}", e))
            .and_then(|(_, exp)| { 
                self.src = Some(exp); 
                if DEBUG {
                    println!("Src: {:?}", self.src);
                }
                Ok(())
            })?;

        Ok(())

    }

    /// This function tries to reduce the AST.
    /// This has to return an Expression rather than an Atom because quoted s_expressions
    /// can't be reduced
    pub fn eval_expression(&mut self) -> Result<(), &str> {
        if let Some(e) = self.src.clone() {
            match e {
                Expr::Function(head) => {
                    for atom in head {
                        self.inst.push(atom);

                        if DEBUG {
                            println!("Stack Dump: {:?}", self.stack);
                        }

                        let top = self.inst.get_mut(self.pc).unwrap().clone();
                        if let Atom::BuiltIn(bi) = top {
                            bi.call(&mut self.stack, &mut self.pc);
                            self.pc += 1;
                        } else {
                            self.stack.push(top);
                            self.pc += 1;
                        }

                    }
                },
                _ => unreachable!()
            };
        }

        Ok(())
    }
}


fn factorial(num: Float) -> Float {
    match num as usize {
        0 => 1.0,
        1 => 1.0,
        _ => factorial(num - 1.0) * num,
    }
}

#[cfg(test)]
mod tests {
    use super::{Putt, Atom};

    macro_rules! putt_eq {
        ($p:expr,$e:expr) => {
            let mut putt = Putt::new();
            putt.parse($p).unwrap();
            putt.eval_expression().unwrap();
            assert_eq!(putt.stack.pop().unwrap(), $e)
        }
    }

    /// Test expressions
    #[test]
    fn test_expr() {
        putt_eq!("1 1+", Atom::Float(2.0));
        putt_eq!("1 1/", Atom::Float(1.0));
        putt_eq!("10 1/", Atom::Float(10.0));
        putt_eq!("10 1s", Atom::Float(10.0));
        putt_eq!("10 1ss", Atom::Float(1.0));
        putt_eq!("10 1x", Atom::Float(10.0));
        putt_eq!("10 1+", Atom::Float(11.0));
        putt_eq!("X 1+", Atom::Float(11.0));
        putt_eq!("2 3+11*1+", Atom::Float(34.0));
        putt_eq!("\"Hi\"\"Hello!\"+", Atom::Str(String::from("HiHello!")));
        putt_eq!("6!", Atom::Float(720.0));
    }

    /// Test string
    #[test]
    fn test_str() {
        putt_eq!("\"Hi\"", Atom::Str("Hi".to_string()));
    }
}
