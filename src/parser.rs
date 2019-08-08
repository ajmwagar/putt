use nom::{
    is_a,
    branch::alt,
    bytes::complete::{tag, escaped},
    character::complete::{alphanumeric0, alpha1, char, digit1, multispace0, multispace1, one_of},
    number::complete::{double},
    combinator::{rest, cut, map, map_res, opt},
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
const IF: &str = "?";
const ELSE: &str = "|";

use super::*;

pub struct RomanNumeral {
    symbol: &'static str,
    value: u32
}
 
const NUMERALS: [RomanNumeral; 13] = [
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
 
pub fn from_roman(roman: &str) -> u32 {
    match NUMERALS.iter().find(|num| roman.starts_with(num.symbol)) {
        Some(num) => num.value + from_roman(&roman[num.symbol.len()..]),
        None => 0, // if string empty, add nothing
    }
}


/// Use nom to parse builtin operators
fn parse_builtin_op<'a>(i: &'a str) -> IResult<&'a str, BuiltIn, VerboseError<&'a str>> {
    // one_of matches one of the characters we give it
    let (i, t) = one_of("+-*/=!^%")(i)?;

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
            )), multispace0)(i)
}

/// Our boolean values are also constant, so we can do it the same way
fn parse_bool<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    alt((map(tag(TRUE), |_| Atom::Boolean(true)), map(tag(FALSE), |_| Atom::Boolean(false))))(i)
}

/// The next easiest thing to parse are keywords.
/// We introduce some error handling combinators: `context` for human readable errors
/// and `cut` to prevent back-tracking.
///
/// Put plainly: `preceded(tag(":"), cut(alpha1))` means that once we see the `:`
/// character, we have to see one or more alphabetic chararcters or the input is invalid.
fn parse_keyword<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    map(context("keyword", preceded(tag(":"), cut(alpha1))), |sym_str: &str| {
        Atom::Keyword(sym_str.to_string())
    })(i)
}

fn parse_string<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    map(context("string", delimited(
                char('\"'),
                alphanumeric0,
                char('\"')
    )), |sym_str: &str| {
        if super::DEBUG {
            println!("Str: {}", sym_str);
        }
        Atom::Str(sym_str.to_string())
    })(i)
}

fn parse_com_string<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
    map(context("string", delimited(
                char('`'),
                alphanumeric0,
                char('`')
    )), |sym_str: &str| {
        if super::DEBUG {
            println!("Str: {}", sym_str);
        }
        Atom::Str(sym_str.to_string())
    })(i)
}

// fn parse_str<'a>(i: &'a str) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
//     // println!("Parse Str: {}", i);
    
//     // Ok(("",i))
// }

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
    preceded(multispace0, alt((parse_string, parse_num, parse_bool, map(parse_builtin, Atom::BuiltIn), parse_roman)))(i)
}


/// We then add the Expr layer on top
fn parse_constant<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  map(parse_atom, |atom| Expr::Constant(atom))(i)
}

/// We can now use our new combinator to define the rest of the `Expr`s.
///
/// Starting with function application, we can see how the parser mirrors our data
/// definitions: our definition is `Function(Vec<Expr>, Box<Expr>)`, so we know
/// that we need to parse an expression and then parse 0 or more expressions, all
/// wrapped in an S-expression.
///
/// `tuple` is used to sequence parsers together, so we can translate this directly
/// and then map over it to transform the output into an `Expr::Function`
fn parse_func<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  let application_inner = map(preceded(multispace0, many0(parse_atom)), |head| {
    Expr::Function(head)
  });
  // finally, we wrap it in an s-expression
  application_inner(i)
}

/// Because `Expr::If` and `Expr::IfElse` are so similar (we easily could have
/// defined `Expr::If` to have an `Option` for the else block), we parse both
/// in a single function.
///
/// In fact, we define our parser as if `Expr::If` was defined with an Option in it,
/// we have the `opt` combinator which fits very nicely here.
fn parse_if<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  let if_inner = context(
    "if expression",
    map(
      preceded(
        // here to avoid ambiguity with other names starting with `if`, if we added
        // variables to our language, we say that if must be terminated by at least
        // one whitespace character
        terminated(tag("if"), multispace1),
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
