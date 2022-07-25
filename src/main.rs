use chumsky::error::Simple;
use chumsky::primitive::{end, just};
use chumsky::recursive::recursive;
use chumsky::text::TextParser;
use chumsky::{text, Parser};

#[derive(Debug)]
enum Token {
    Integer(i32),
    Decimal(f32),
    String(str),

    Neg(Box<Token>),
    Add(Box<Token>, Box<Token>),
    Sub(Box<Token>, Box<Token>),
    Mul(Box<Token>, Box<Token>),
    Div(Box<Token>, Box<Token>),
}

fn eval(token: &Token) -> Result<i32, String> {
    match token {
        Token::Integer(x) => Ok(*x),
        Token::Neg(x) => Ok(-eval(x)?),

        Token::Add(a, b) => Ok(eval(a)? + eval(b)?),
        Token::Sub(a, b) => Ok(eval(a)? - eval(b)?),
        Token::Mul(a, b) => Ok(eval(a)? * eval(b)?),
        Token::Div(a, b) => Ok(eval(a)? / eval(b)?),
        _ => todo!(),
    }
}

fn parser() -> impl Parser<char, Token, Error = Simple<char>> {
    recursive(|expr| {
        let integer = text::int(10).map(|s: String| Token::Integer(s.parse().unwrap()));
        //.padded()
        let atom = integer.or(expr.delimited_by(just('('), just(')'))).padded();

        let op = |c| just(c).padded();

        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| Token::Neg(Box::new(rhs)));

        let product = unary
            .clone()
            .then(
                op('*')
                    .to(Token::Mul as fn(_, _) -> _)
                    .or(op('/').to(Token::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        let sum = product
            .clone()
            .then(
                op('+')
                    .to(Token::Add as fn(_, _) -> _)
                    .or(op('-').to(Token::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        sum
    })
    .then_ignore(end())
}

fn main() {
    let arg1 = std::env::args().nth(1).unwrap();
    let src = std::fs::read_to_string(arg1).unwrap();
    match parser().parse(src) {
        Ok(ast) => match eval(&ast) {
            Ok(output) => {
                println!("{:?}", ast);
                println!("{:?}", output);
            }
            Err(eval_err) => println!("Evaluation error: {}", eval_err),
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }
}
