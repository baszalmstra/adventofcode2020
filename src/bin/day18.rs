use std::iter::Peekable;

#[derive(Debug, Copy, Clone)]
enum BinaryOp {
    Multiply,
    Add,
}

#[derive(Debug)]
enum Expr {
    Literal(isize),
    BinaryOp {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

impl Expr {
    fn eval(&self) -> isize {
        match self {
            Expr::Literal(v) => *v,
            Expr::BinaryOp { op, left, right } => {
                let left = left.eval();
                let right = right.eval();
                match op {
                    BinaryOp::Multiply => left * right,
                    BinaryOp::Add => left + right,
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Token {
    LParan,
    RParan,
    Number(isize),
    Plus,
    Multiply,
}

impl Token {
    fn to_op(self) -> Option<BinaryOp> {
        match self {
            Token::Plus => Some(BinaryOp::Add),
            Token::Multiply => Some(BinaryOp::Multiply),
            _ => None,
        }
    }
}

struct Tokenizer<'a> {
    input: &'a str,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.char_indices().peekable();
        while chars.peek()?.1.is_whitespace() {
            chars.next();
        }
        let (first_char_index, first_char) = chars.next()?;
        let result = match first_char {
            '(' => Token::LParan,
            ')' => Token::RParan,
            '+' => Token::Plus,
            '*' => Token::Multiply,
            c if c.is_ascii_digit() => {
                let last_char_index = loop {
                    match chars.peek() {
                        None => break self.input.len(),
                        Some((last_char_index, c)) if !c.is_ascii_digit() => {
                            break *last_char_index
                        }
                        _ => chars.next(),
                    };
                };
                let number_str = &self.input[first_char_index..last_char_index];
                Token::Number(number_str.parse().unwrap())
            }
            c => unreachable!("unknown char: '{}'", c),
        };
        if let Some((next_char_index, _)) = chars.peek() {
            self.input = &self.input[*next_char_index..];
        } else {
            self.input = "";
        };
        Some(result)
    }
}

fn parse_atom<I: Iterator<Item = Token>, F: Fn(BinaryOp) -> u8>(
    tokens: &mut Peekable<I>,
    precedence_selector: &F,
) -> Expr {
    match tokens.next().expect("unexpected end of line") {
        Token::LParan => {
            let expr = parse_expr(tokens, precedence_selector, 0);
            let r_paran = tokens.next();
            assert_eq!(r_paran, Some(Token::RParan));
            expr
        }
        Token::Number(n) => Expr::Literal(n),
        c => unreachable!("unexpected: {:?}", c),
    }
}

fn parse_expr<I: Iterator<Item = Token>, F: Fn(BinaryOp) -> u8>(
    tokens: &mut Peekable<I>,
    precedence_selector: &F,
    precedence: u8,
) -> Expr {
    let mut expr = parse_atom(tokens, precedence_selector);
    while let Some(op) = tokens.peek().and_then(|t| t.to_op()) {
        let op_precedence = precedence_selector(op);
        if op_precedence < precedence {
            break;
        }
        tokens.next();
        let right = parse_expr(tokens, precedence_selector, op_precedence + 1);
        expr = Expr::BinaryOp {
            op,
            left: Box::new(expr),
            right: Box::new(right),
        };
    }
    expr
}

fn parse<F: Fn(BinaryOp) -> u8>(input: &str, precedence_selector: &F) -> Vec<Expr> {
    input
        .lines()
        .map(|line| {
            let tokens = Tokenizer { input: line };
            parse_expr(&mut tokens.peekable(), precedence_selector, 0)
        })
        .collect()
}

fn main() {
    let input = std::fs::read_to_string("inputs/day18/input").unwrap();

    println!(
        "Solution 1: {}",
        parse(&input, &|_| 1)
            .into_iter()
            .map(|expr| expr.eval())
            .sum::<isize>()
    );
    println!(
        "Solution 2: {}",
        parse(&input, &|op| match op {
            BinaryOp::Multiply => 1,
            BinaryOp::Add => 3,
        })
        .into_iter()
        .map(|expr| expr.eval())
        .sum::<isize>()
    );
}
