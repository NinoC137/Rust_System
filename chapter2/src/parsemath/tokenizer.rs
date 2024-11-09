//Nino
use std::iter::Peekable;
use std::str::Chars;

use super::token::Token;

pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>
    /*
    在 Rust 中，`<'a>` 是一个生命周期参数，用于指示一个引用的有效范围。
    它的主要作用是确保在编译时检查引用的有效性，避免出现悬垂引用的问题。
    在你的代码中，`Tokenizer<'a>` 结构体中的 `expr` 字段是一个 `Peekable<Chars<'a>>` 类型的变量，
    这意味着 `Tokenizer` 结构体持有一个 `Chars` 迭代器，它引用了某个生命周期为 `'a` 的字符串。
    这个生命周期 `'a` 表示 `Tokenizer` 结构体实例在使用 `expr` 字段时，所引用的字符串必须在这个实例的生命周期内有效。
    这样做的好处是可以确保在你使用 `Tokenizer` 的时候，确保其内部的引用不会超出原始数据的有效范围，
    从而避免潜在的运行时错误。
    */
}

impl<'a> Tokenizer<'a>  {
    pub fn new(new_expr: &'a str) -> Self {
        Tokenizer {
            expr: new_expr.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..= '9') => {
                let mut number = next_char?.to_string();

                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.'{
                        number.push(self.expr.next()?);
                    }else if next_char == &'(' {
                        return None;
                    }else {
                        break;
                    }
                }

                Some(Token::Num(number.parse::<f64>().unwrap()))
            },
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}