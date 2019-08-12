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
                            bi.call(&mut self.stack);
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


fn factorial(num: Float) -> Float {
    match num {
        0.0 => 1.0,
        1.0 => 1.0,
        _ => factorial(num - 1.0) * num,
    }
}

// /// To start we define a couple of helper functions
// fn get_num_from_ref(e: &Atom) -> Option<Num> {
//     if let Atom::Num(n) = e {
//         Some(*n)
//     } else {
//         None
//     }
// }

// /// To start we define a couple of helper functions
// fn get_num_from_atom(e: Atom) -> Option<Num> {
//     if let Atom::Num(n) = e {
//         Some(n)
//     } else {
//         None
//     }
// }

// fn get_bool_from_ref(e: &Atom) -> Option<bool> {
//     if let Atom::Boolean(b) = e {
//         Some(*b)
//     } else {
//         None
//     }
// }

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
        putt_eq!("1 1/", Atom::Num(1));
        putt_eq!("10 1/", Atom::Num(10));
        putt_eq!("10 1+", Atom::Num(11));
        putt_eq!("X 1+", Atom::Num(11));
        putt_eq!("2 3+11*1+", Atom::Num(56));
        putt_eq!("\"Hi\"\"Hello!\"+", Atom::Str(String::from("HiHello!")));
        putt_eq!("6!", Atom::Num(720));
    }

    /// Test string
    #[test]
    fn test_str() {
        putt_eq!("\"Hi\"", Atom::Str("Hi".to_string()));
    }
}
