use smaz::{decompress};
use std::io::prelude::*;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{alpha1, char as ch, digit1, multispace0, multispace1, one_of},
    number::complete::{double},
    combinator::{cut, map, map_res, opt},
    error::{context, VerboseError},
    multi::many0,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

// Language tags
const TRUE: &str = "#t";
const FALSE: &str = "#f";
const NOT: &str = "n";
const PRINTLN: &str = ",";
const PRINT: &str = ".";
const CMP: &str = "cmp";
const DCMP: &str = "dmp";
const IF: &str = "?";
const _ELSE: &str = "|";

use super::*;

pub struct RomanNumeral {
    symbol: &'static str,
    value: u128
}

 
const NUMERALS: [RomanNumeral; 25] = [
    RomanNumeral {symbol: "Mk",  value: 1_000_000},
    RomanNumeral {symbol: "CMk",  value: 900_000},
    RomanNumeral {symbol: "Dk",  value: 500_000},
    RomanNumeral {symbol: "CDk",  value: 400_000},
    RomanNumeral {symbol: "Ck",  value: 100_000},
    RomanNumeral {symbol: "XCk",  value: 90_000},
    RomanNumeral {symbol: "Lk",  value: 50_000},
    RomanNumeral {symbol: "XLk",  value: 40_000},
    RomanNumeral {symbol: "Xk",  value: 10_000},
    RomanNumeral {symbol: "IXk",  value: 9_000},
    RomanNumeral {symbol: "Vk",  value: 5_000},
    RomanNumeral {symbol: "IVk",  value: 4_000},
    RomanNumeral {symbol: "M",  value: 1000},
    RomanNumeral {symbol: "CM", value: 900},
    RomanNumeral {symbol: "D",  value: 500},
    RomanNumeral {symbol: "CD", value: 400},
    RomanNumeral {symbol: "C",  value: 100},
    RomanNumeral {symbol: "XC", value: 90},
    RomanNumeral {symbol: "L",  value: 50},
    RomanNumeral {symbol: "XL", value: 40},
    RomanNumeral {symbol: "X",  value: 10},
    RomanNumeral {symbol: "IX", value: 9},
    RomanNumeral {symbol: "V",  value: 5},
    RomanNumeral {symbol: "IV", value: 4},
    RomanNumeral {symbol: "I",  value: 1}
];

// pub struct Constant {
//     symbol: &'static str,
//     value: Atom
// }

// const CONSTANTS: [Constant; 1] = [
//     Constant {symbol: "N", value: Atom::Str({
//         let mut string = String::new();
//         std::io::stdin().read_to_string(&mut string);
//         string
//     })}
// ];

 
pub fn from_roman(roman: &str) -> u128 {
    match NUMERALS.iter().find(|num| roman.starts_with(num.symbol)) {
        Some(num) => num.value + from_roman(&roman[num.symbol.len()..]),
        None => 0, // if string empty, add nothing
    }
}


/// Use nom to parse builtin operators
fn parse_builtin_op<'a>(i: &'a str) -> IResult<&'a str, BuiltIn, VerboseError<&'a str>> {
    // one_of matches one of the characters we give it
    let (i, t) = one_of("R+-*/=!^%")(i)?;

    // because we are matching single character tokens, we can do the matching logic
    // on the returned value
    Ok((
            i,
            match t {
                '+' => BuiltIn::Plus,
                '-' => BuiltIn::Minus,
                '*' => BuiltIn::Times,
                '/' => BuiltIn::Divide,
                '=' => BuiltIn::Equal,
                '!' => BuiltIn::Factorial,
                '^' => BuiltIn::Power,
                'R' => BuiltIn::Root,
                '%' => BuiltIn::Modulus,
                _ => unreachable!(),
            },
            ))
}

fn parse_builtin<'a>(i: &'a str) -> IResult<&'a str, BuiltIn, VerboseError<&'a str>> {
    // alt gives us the result of first parser that succeeds, of the series of
    // parsers we give it
    delimited(multispace0, alt((
            parse_builtin_op,
            // map lets us process the parsed output, in this case we know what we parsed,
            // so we ignore the input and return the BuiltIn directly
            map(tag(NOT), |_| BuiltIn::Not),
            map(tag(PRINTLN), |_| BuiltIn::PrintLn),
            map(tag(PRINT), |_| BuiltIn::Print),
            map(tag(CMP), |_| BuiltIn::Cmp),
            map(tag(DCMP), |_| BuiltIn::Dcmp),
            )), multispace0)(i)
}

/// Our boolean values are also constant, so we can do it the same way
fn parse_bool<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((map(tag(TRUE), |_| Atom::Boolean(true)), map(tag(FALSE), |_| Atom::Boolean(false))))(i)
}

/// Parse string literal
fn parse_string<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    if super::DEBUG {
        println!("String parser");
    }
    let res = map(context("string", delimited(ch('"'), is_not("\""), ch('"'))), |sym_str: &str| {
        Atom::Str(sym_str.to_string())
    })(i);
    res
}

/// Parse and decompress a smaz encoded string
fn parse_com_string<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    if super::DEBUG {
        println!("String parser");
    }
    let res = map(context("string", delimited(ch('`'), is_not("`"), ch('`'))), |sym_str: &str| {
        Atom::Str(String::from_utf8(decompress(sym_str.as_bytes()).unwrap()).unwrap())
    })(i);
    res
}

/// Parse roman numeral literal
fn parse_roman<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    map(context("roman numeral", preceded(multispace0, alpha1)), |numeral: &str| {
        let num = from_roman(numeral) as Num;
        if super::DEBUG {
            println!("Roman: {} Hindu: {}", numeral, num);
        }
        Atom::Num(num)
    })(i)
}

/// Parse an integer, either singed or unsigned
fn parse_num<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
            map_res(digit1, |digit_str: &str| digit_str.parse::<Num>().map(Atom::Num)),
            map(preceded(tag("-"), digit1), |digit_str: &str| {
                Atom::Num(-1 * digit_str.parse::<Num>().unwrap())
            }),
            ))(i)
}

/// Parse a floating point number
fn parse_float<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((
            map(double, |f: f64| Atom::Float(f)),
            map(preceded(tag("-"), double), |digit_str: f64| {
                Atom::Float(-1.0 * digit_str)
            }),
            ))(i)
}

/// Parse atomics
fn parse_atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    // TODO: Delimite floating points
    preceded(multispace0, alt((/*parse_float,*/ parse_num, parse_bool, parse_com_string, parse_string, map(parse_builtin, Atom::BuiltIn), parse_roman, )))(i)
}


/// Parse expressions
fn parse_func<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  let application_inner = map(preceded(multispace0, many0(parse_atom)), |head| {
    Expr::Function(head)
  });
  // finally, we wrap it in an s-expression
  application_inner(i)
}

// TODO: If/else statments
fn parse_if<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  let if_inner = context(
    "if expression",
    map(
      preceded(
        // here to avoid ambiguity with other names starting with `if`, if we added
        // variables to our language, we say that if must be terminated by at least
        // one whitespace character
        terminated(tag(IF), multispace1),
        cut(tuple((parse_expr, parse_expr, opt(parse_expr)))),
      ),
      |(pred, true_branch, maybe_false_branch)| {
        if let Some(false_branch) = maybe_false_branch {
          Expr::IfElse(Box::new(pred), Box::new(true_branch), Box::new(false_branch))
        } else {
          Expr::If(Box::new(pred), Box::new(true_branch))
        }
      },
    ),
  );
  if_inner(i)
}


/// We tie them all together again, making a top-level expression parser!

pub fn parse_expr<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  preceded(multispace0, alt((parse_func, parse_if)))(i)
}



#[cfg(test)]
mod tests {
    macro_rules! nom_eq {
    ($p:expr,$e:expr) => (
        assert_eq!($p.unwrap().1, $e)
    )
    }

    fn atom_str(s: &str) -> Atom {
        Atom::Str(s.to_string())
    }

    #[cfg(test)]
    fn atom_num(s: isize) -> Atom {
        Atom::Num(s as i128)
    }

    use super::*;
    #[test]
    fn assert_parse_string() {
        nom_eq!(parse_string("\"Hello, World!\""), atom_str("Hello, World!"));
        nom_eq!(parse_string("\"Hello, World\""), atom_str("Hello, World"));
    }

    #[test]
    fn assert_roman() {
        nom_eq!(parse_roman("CMD"), atom_num(1400));
        nom_eq!(parse_roman("CMk"), atom_num(900_000));
        nom_eq!(parse_roman("Mk"), atom_num(1_000_000));
        nom_eq!(parse_roman("Dk"), atom_num(500_000));
        nom_eq!(parse_roman("Lk"), atom_num(50_000));
        nom_eq!(parse_roman("C"), atom_num(100));
        nom_eq!(parse_roman("X"), atom_num(10));
        nom_eq!(parse_roman("V"), atom_num(5));
        nom_eq!(parse_roman("IV"), atom_num(4));
        nom_eq!(parse_roman("I"), atom_num(1));
    }
}
