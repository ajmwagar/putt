use std::error::Error;

const DEBUG: bool = false;

use nom::error::VerboseError;

use parser::*;
use smaz::{compress};

pub mod parser;

pub type Num = i128;
pub type Float = f64;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
    Num(Num),
    Float(f64),
    Keyword(String),
    Str(String),
    Boolean(bool),
    BuiltIn(BuiltIn),
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
            }
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
    Modulus,
    Factorial,

    // Keywords
    Not,
    Print,
    PrintLn,
    Cmp

}
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
    pub src: Option<Expr>
}

impl Putt {
    /// Create a new Putt VM
    pub fn new() -> Self {
        Putt {
            stack: Vec::new(),
            src: None
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

    /// Run the loaded program
    pub fn run(&self) -> Vec<String> {
        Vec::new()
    }

    /// This function tries to reduce the AST.
    /// This has to return an Expression rather than an Atom because quoted s_expressions
    /// can't be reduced
    pub fn eval_expression(&mut self) -> Result<(), &str> {
        if let Some(e) = self.src.clone() {
            match e {
                // Constants and quoted s-expressions are our base-case
                // Expr::Constant(_) => Some(e),
                // // we then recursively `eval_expression` in the context of our special forms
                // // and built-in operators
                // Expr::If(pred, true_branch) => {
                //   let reduce_pred = eval_expression(*pred)?;
                //   if get_bool_from_ref(reduce_pred)? {
                //     eval_expression(*true_branch)
                //   } else {
                //     None
                //   }
                // }
                // Expr::IfElse(pred, true_branch, false_branch) => {
                //   let reduce_pred = eval_expression(*pred)?;
                //   if get_bool_from_ref(reduce_pred)? {
                //     eval_expression(*true_branch)
                //   } else {
                //     eval_expression(*false_branch)
                //   }
                // }
                Expr::Function(head) => {
                    for atom in head {
                        self.stack.push(atom);

                        if DEBUG {
                            println!("Stack Dump: {:?}", self.stack);
                        }

                        let top = self.stack.pop().unwrap();
                        if let Atom::BuiltIn(bi) = top {

                            if DEBUG {
                                println!("Executing command!");
                            }
                            match bi {
                                BuiltIn::Plus => {
                                    if self.stack.len() < 2 {
                                        // Err("Not enough variables in stack");
                                    } else {
                                        // TODO: Graceful exit if not enough in stack
                                        let a = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        let b = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        self.stack.push(Atom::Num(a + b));
                                    }
                                },
                                BuiltIn::Times => {
                                    if self.stack.len() < 2 {
                                        // Err("Not enough variables in stack");
                                    } else {
                                        // TODO: Graceful exit if not enough in stack
                                        let a = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        let b = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        self.stack.push(Atom::Num(a * b));
                                    }
                                },
                                BuiltIn::Equal => self.stack.push(Atom::Boolean(self.stack.iter().zip(self.stack.iter().skip(1)).all(|(a, b)| a == b))),
                                BuiltIn::Not => {
                                    if self.stack.len() < 1 {
                                        // Err("Not enough variables in stack");
                                    } else {
                                        let bo = self.stack.pop().unwrap();
                                        self.stack.push(Atom::Boolean(!get_bool_from_ref(&bo).unwrap()));
                                    }
                                }
                                BuiltIn::Minus => {
                                    if self.stack.len() < 2 {
                                        // Err("Not enough variables in stack");
                                    } else {
                                        // TODO: Graceful exit if not enough in stack
                                        let a = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        let b = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        self.stack.push(Atom::Num(a - b));
                                    }
                                },
                                BuiltIn::Divide => {
                                    if self.stack.len() < 2 {
                                        // Err("Not enough variables in stack");
                                    } else {
                                        // TODO: Graceful exit if not enough in stack
                                        let a = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        let b = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        self.stack.push(Atom::Num(a / b));
                                    }
                                },
                                BuiltIn::Power => {
                                    if self.stack.len() < 2 {
                                        // Err("Not enough variables in stack");
                                    } else {
                                        // TODO: Graceful exit if not enough in stack
                                        let a = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        let b = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        self.stack.push(Atom::Num(a.pow(b as u32)));
                                    }
                                },
                                BuiltIn::Modulus => {
                                    if self.stack.len() < 2 {
                                        // Err("Not enough variables in stack");
                                    } else {
                                        // TODO: Graceful exit if not enough in stack
                                        let a = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        let b = get_num_from_atom(self.stack.pop().unwrap()).unwrap();
                                        self.stack.push(Atom::Num(a % b));
                                    }
                                },
                                BuiltIn::Factorial => { 
                                    if let Some(first_elem) = self.stack.pop() {
                                        self.stack.push(Atom::Num(factorial(get_num_from_ref(&first_elem).unwrap())))
                                    }
                                },
                                BuiltIn::PrintLn => {
                                    if let Some(first_elem) = self.stack.pop() {
                                        println!("{}", first_elem)
                                    }
                                }, 
                                BuiltIn::Print => {
                                    if let Some(first_elem) = self.stack.pop() {
                                        print!("{} ", first_elem)
                                    }
                                },
                                BuiltIn::Cmp => {
                                    if let Some(Atom::Str(first_elem)) = self.stack.pop() {
                                        self.stack.push(Atom::Str(String::from_utf8_lossy(&compress(first_elem.as_bytes())).to_string()))
                                    }
                                }
                            } 
                        } else {
                            self.stack.push(top);
                        }
                    }
                },
                _ => unreachable!()
            };
        }

        Ok(())
    }


    // // TODO: Define output type
    // pub fn exec(program: &str) -> Vec<String> {
    //     let mut putt = Putt::new();
    //     putt.parse(program);
    //     putt.run()
    // }
}

mod test {
    #[test]
    fn expressions() {
        // assert_eq!()
    }
}

fn factorial(num: Num) -> Num {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

/// To start we define a couple of helper functions
fn get_num_from_ref(e: &Atom) -> Option<Num> {
    if let Atom::Num(n) = e {
        Some(*n)
    } else {
        None
    }
}

/// To start we define a couple of helper functions
fn get_num_from_atom(e: Atom) -> Option<Num> {
    if let Atom::Num(n) = e {
        Some(n)
    } else {
        None
    }
}

fn get_bool_from_ref(e: &Atom) -> Option<bool> {
    if let Atom::Boolean(b) = e {
        Some(*b)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{Putt, Atom};

    macro_rules! putt_eq {
        ($p:expr,$e:expr) => {
            let mut putt = Putt::new();
            putt.parse($p);
            putt.eval_expression();
            assert_eq!(putt.stack.pop().unwrap(), $e)
        }
    }

    /// Test expressions
    #[test]
    fn test_expr() {
        putt_eq!("1 1+", Atom::Num(2));
        putt_eq!("10 1+", Atom::Num(11));
        putt_eq!("X 1+", Atom::Num(11));
    }
    /// Test string
    #[test]
    fn test_str() {
        putt_eq!("\"Hi\"", Atom::Str("Hi".to_string()));
    }
}
