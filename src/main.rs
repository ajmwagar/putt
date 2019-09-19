use std::{io, io::prelude::*, error::Error, fs::File, path::PathBuf};
use structopt::StructOpt;
use putt::*;
use putt::atom::*;

#[derive(StructOpt, Debug)]
#[structopt(name = "putt")]
struct PuttCLI {
    #[structopt(name = "FILE")]
    /// File to read
    path: Option<PathBuf>
}

fn main() -> Result<(), Box<dyn Error>> {
    let puttcli = PuttCLI::from_args();

    let mut putt = Putt::new();

    // Load file or open REPL
    if let Some(path) = puttcli.path {
        let mut file = File::open(path)?;
        let mut fstring = String::new();
        file.read_to_string(&mut fstring)?;

        putt.parse(&fstring)?;
        putt.eval_expression()?;

        if let Some(atom) = putt.stack.last() {
            match atom {
                Atom::BuiltIn(b) => {
                    match b {
                        BuiltIn::Print => {},
                        BuiltIn::PrintLn => {},
                        _ => println!("{}", atom)
                    }
                }
                _ => println!("{}", atom)
            }
        }
        // println!("{:?}", putt.stack.pop());
    }
    else {
        println!("PUTT REPL v0.0.1");
        print!(">> ");
        io::stdout().flush()?;

        let stdin = io::stdin();
        // Start reading lines
        for line in stdin.lock().lines() {
            putt.parse(&line.unwrap())?;

            putt.eval_expression()?;

            // Inject printing code at the end
            if let Some(atom) = putt.stack.last() {
                match atom {
                    Atom::BuiltIn(b) => {
                        match b {
                            BuiltIn::Print => {},
                            BuiltIn::PrintLn => {},
                            _ => println!("{}", atom)
                        }
                    }
                    _ => println!("{}", atom)
                }
            }

            print!("\n>> ");
            io::stdout().flush()?;
        }
    }

    Ok(())
}
