use std::{io, io::prelude::*, error::Error, fs::File, path::PathBuf};
use smaz::{compress,decompress};

pub mod parser;


struct Putt {
    // /// Memory of Putt VM
    // stack: Vec<Type>,
}

impl Putt {
    /// Create a new Putt VM
    pub fn new() -> Self {
        Putt {
            // stack: Vec::new()
        }
    }

    // TODO: Replace with helpful parsing error
    // TODO: Figure out best format for program
    pub fn parse(&mut self, p: &str) -> Result<(), Box<dyn Error>> {
        // TODO: Parse

        Ok(())
    }

    /// Run the loaded program
    pub fn run(&self) -> Vec<String> {
        Vec::new()
    }

    // TODO: Define output type
    pub fn exec(program: &str) -> Vec<String> {
        let mut putt = Putt::new();
        putt.parse(program);
        putt.run()
    }
}

mod test {
    #[test]
    fn expressions() {
        // assert_eq!()
    }
}
