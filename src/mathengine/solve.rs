use agb::fixnum::Number;
use alloc::vec::{self, Vec};
use crate::mathengine::token::Token::{self, *};

use libm::pow;

use super::{core::CalcEngine, math_function::FunctionHandling};

pub trait Solver {
    fn solve(&mut self, equation: Vec<Token>) -> Result<Vec<Token>, &'static str>;
    fn solve_rec(&mut self, token_list: &mut Vec<Token>) -> Result<Vec<Token>, &'static str>;
}

impl Solver for CalcEngine {
    fn solve(&mut self, equation: Vec<Token>) -> Result<Vec<Token>, &'static str> {
        if equation.len() == 0 {
            let mut v = Vec::new();
            v.push(Number(0.0));
            return Ok(v);
        }
        let mut token_list = equation.clone();
        let answer = self.solve_rec(&mut token_list);
        answer
    }

    fn solve_rec(&mut self, token_list: &mut Vec<Token>) -> Result<Vec<Token>, &'static str> {
        let mut i = 0;
        while i < token_list.len() {
            match &token_list[i] {
                Token::LeftBracket => {
                    let right_bracket_index = get_matching_bracket_index(&token_list[i + 1..]) + i + 1;
                    match self.solve_rec(&mut token_list[i + 1..right_bracket_index].into()) {
                        Ok(answer) => token_list.splice(i..right_bracket_index, answer),
                        Err(e) => return Err(e),
                    };
                    i = 0;
                }
                Token::FunctionName(func_name) => {
                    let right_bracket_index = get_matching_bracket_index(&token_list[i + 2..]) + i + 1;
                    match self.solve_function(&func_name, &token_list[i+2..right_bracket_index + 1]) {
                        Ok(answer) => token_list.splice(i..right_bracket_index, Vec::from([answer])),
                        Err(e) => return Err(e),
                    }
                    ;
                    i = 0;
                }
                _ => {
                    i += 1;
                }
            }
        }
        i = 0;
        while i < token_list.len() {
            match token_list[i] {
                Token::Exponentation => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = pow(a, b);
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        i = 0;
        while i < token_list.len() {
            match token_list[i] {
                Token::Multiplication => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a * b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::Division => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a / b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::Modulation => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a % b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
        i = 0;
        while i < token_list.len() {
            match token_list[i] {
                Token::Addition => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a + b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                Token::Subtraction => {
                    if let Token::Number(a) = token_list[i - 1] {
                        if let Token::Number(b) = token_list[i + 1] {
                            let answer = a - b;
                            token_list[i - 1] = Token::Number(answer);
                            token_list.remove(i);
                            token_list.remove(i);
                            i = 0;
                        }
                    }
                }
                _ => {
                    i += 1;
                }
            }
        }
    
        Ok(token_list.to_vec())
    }
}




pub fn get_matching_bracket_index(token_list: &[Token]) -> usize {
    let mut depth = 1;
    for i in 0..token_list.len() {
        match token_list[i] {
            Token::LeftBracket => depth += 1,
            Token::RightBracket => depth -= 1,
            _ => (),
        }
        if depth == 0 {
            return i;
        }
    }

    0
}
