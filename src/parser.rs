use nom::{
  branch::alt,
  bytes::complete::{tag, escaped},
  character::complete::{alphanumeric1, alpha1, char, digit1, multispace0, multispace1, one_of},
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
const PRINT: &str = ",";
const IF: &str = "?";
const ELSE: &str = "|";

type Num = i32;
type Float = f64;

#[derive(Debug, PartialEq, Clone)]
pub enum Atom {
  Num(Num),
  Float(f64),
  Keyword(String),
  Str(String),
  Boolean(bool),
  BuiltIn(BuiltIn),
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
  Print

}
#[derive(Debug, PartialEq, Clone)] pub enum Expr {
  Constant(Atom),
  /// (func-name arg1 arg2)
  Application(Vec<Expr>, Box<Expr>),
  /// (if predicate do-this)
  If(Box<Expr>, Box<Expr>),
  /// (if predicate do-this otherwise-do-this)
  IfElse(Box<Expr>, Box<Expr>, Box<Expr>),
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
  alt((
    parse_builtin_op,
    // map lets us process the parsed output, in this case we know what we parsed,
    // so we ignore the input and return the BuiltIn directly
    map(tag(NOT), |_| BuiltIn::Not),
    map(tag(PRINT), |_| BuiltIn::Print),
  ))(i)
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
    parse_str,
    char('\"')
  )), |sym_str: &str| {Atom::Str(sym_str.to_string())})(i)
}

fn parse_str<'a>(i: &'a str) -> IResult<&'a str, &'a str, VerboseError<&'a str>> {
  escaped(alphanumeric1, '\\', one_of("\"n\\"))(i)
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
    map_res(digit1, |digit_str: &str| digit_str.parse::<Float>().map(Atom::Float)),
    map(preceded(tag("-"), digit1), |digit_str: &str| {
      Atom::Float(-1.0 * digit_str.parse::<Float>().unwrap())
    }),
  ))(i)
}

/// Parse atomics
fn parse_atom<'a>(i: &'a str) -> IResult<&'a str, Atom, VerboseError<&'a str>> {
  alt((parse_num, parse_float, parse_bool, parse_string, map(parse_builtin, Atom::BuiltIn), parse_keyword))(i)
}

/// We then add the Expr layer on top
fn parse_constant<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  map(parse_atom, |atom| Expr::Constant(atom))(i)
}

/// Before continuing, we need a helper function to parse lists.
/// A list starts with `(` and ends with a matching `)`.
/// By putting whitespace and newline parsing here, we can avoid having to worry about it
/// in much of the rest of the parser.
///
/// Unlike the previous functions, this function doesn't take or consume input, instead it
/// takes a parsing function and returns a new parsing function.
fn s_exp<'a, O1, F>(inner: F) -> impl Fn(&'a str) -> IResult<&'a str, O1, VerboseError<&'a str>>
where
  F: Fn(&'a str) -> IResult<&'a str, O1, VerboseError<&'a str>>,
{
  delimited(
    char('('),
    preceded(multispace0, inner),
    context("closing paren", cut(preceded(multispace0, char(')')))),
  )
}

/// We can now use our new combinator to define the rest of the `Expr`s.
///
/// Starting with function application, we can see how the parser mirrors our data
/// definitions: our definition is `Application(Box<Expr>, Vec<Expr>)`, so we know
/// that we need to parse an expression and then parse 0 or more expressions, all
/// wrapped in an S-expression.
///
/// `tuple` is used to sequence parsers together, so we can translate this directly
/// and then map over it to transform the output into an `Expr::Application`
fn parse_application<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  let application_inner = map(tuple((many0(parse_expr), parse_expr)), |(head, tail)| {
    Expr::Application(head, Box::new(tail))
  });
  // finally, we wrap it in an s-expression
  s_exp(application_inner)(i)
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
  s_exp(if_inner)(i)
}


/// We tie them all together again, making a top-level expression parser!

fn parse_expr<'a>(i: &'a str) -> IResult<&'a str, Expr, VerboseError<&'a str>> {
  preceded(multispace0, alt((parse_constant, parse_application, parse_if)))(i)
}

/// And that's it!
/// We can now parse our entire lisp language.
///
/// But in order to make it a little more interesting, we can hack together
/// a little interpreter to take an Expr, which is really an
/// [Abstract Syntax Tree](https://en.wikipedia.org/wiki/Abstract_syntax_tree) (AST),
/// and give us something back

/// To start we define a couple of helper functions
fn get_num_from_expr(e: Expr) -> Option<Num> {
  if let Expr::Constant(Atom::Num(n)) = e {
    Some(n)
  } else {
    None
  }
}

fn get_bool_from_expr(e: Expr) -> Option<bool> {
  if let Expr::Constant(Atom::Boolean(b)) = e {
    Some(b)
  } else {
    None
  }
}

/// This function tries to reduce the AST.
/// This has to return an Expression rather than an Atom because quoted s_expressions
/// can't be reduced
fn eval_expression(e: Expr) -> Option<Expr> {
  println!("Expr: {:?}", e);
  match e {
      Expr::Application(head, tail) => {
          println!("Head: {:?}", head, tail);
      },

  }
}

/// And we add one more top-level function to tie everything together, letting
/// us call eval on a string directly
pub fn eval_from_str(src: &str) -> Result<Expr, String> {
  parse_expr(src)
    .map_err(|e: nom::Err<VerboseError<&str>>| format!("{:?}", e))
    .and_then(|(_, exp)| eval_expression(exp).ok_or("Eval failed".to_string()))
}

fn factorial(num: Num) -> Num {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}
