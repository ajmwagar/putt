use std::{io, io::prelude::*, error::Error, fs::File, path::PathBuf};
use structopt::StructOpt;
use putt::*;

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
        // println!("{:?}", putt.stack.pop());
    }
    else {
        print!("> ");
        io::stdout().flush()?;

        let stdin = io::stdin();
        // Start reading lines
        for line in stdin.lock().lines() {
            putt.parse(&line.unwrap())?;
            // Inject printing code at the end
            // if let Some(Expr::Function(mut src)) = putt.src {
            //     src.push(Atom::BuiltIn(BuiltIn::PrintLn));
            //     putt.src = Some(Expr::Function(src));
            // }

            putt.eval_expression()?;

            print!("> ");
            io::stdout().flush()?;
        }
    }

    Ok(())
}
