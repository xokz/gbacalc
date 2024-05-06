use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::mathengine::{core::CalcEngine, core::TokenStringResult, token::Token::*};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Addition,       //              +
    Subtraction,    //           -
    Division,       //              /
    Multiplication, //        *
    Truncation,     //            //
    Modulation,     //            %
    Exponentation,  //         **

    BitwiseLeftShift,  //      <<
    BitwiseRightShift, //     >>
    BitwiseAnd,        //            &
    BitwiseOr,         //             |
    BitwiseXor,        //            ^

    LeftBracket,  //           (
    RightBracket, //          )

    Comma, //                 ,

    Assignment, //            =

    UnresolvedString(Vec<u8>),
    Variable(Vec<u8>),
    FunctionName(Vec<u8>),
    FunctionArg(usize),

    Number(f64),
}

pub fn match_token(key: &[u8]) -> Option<Token> {
    let result = match key {
        b"<<" => Some(BitwiseLeftShift),
        b">>" => Some(BitwiseRightShift),
        b"^" => Some(Exponentation),
        b"//" => Some(Truncation),
        b"+" => Some(Addition),
        b"-" => Some(Subtraction),
        b"/" => Some(Division),
        b"*" => Some(Multiplication),
        b"%" => Some(Modulation),
        b"&" => Some(BitwiseAnd),
        b"|" => Some(BitwiseOr),
        b"^^" => Some(BitwiseXor),
        b"(" => Some(LeftBracket),
        b")" => Some(RightBracket),
        b"," => Some(Comma),
        b"=" => Some(Assignment),
        _ => None,
    };
    result
}

pub fn tokenize(input: &Vec<u8>) -> Result<Vec<Token>, &'static str> {
    let mut tokens = get_tokens(input)?;
    resolve_strings(&mut tokens);
    Ok(tokens)
}

fn get_tokens(input: &[u8]) -> Result<Vec<Token>, &'static str> {
    // this will store the list of tokens, and will be returned
    let mut tokens: Vec<Token> = Vec::new();

    let is_number_part = |x: u8| (x as char).is_digit(10) || x == b'.';
    let is_string_part = |x: u8| (x as char).is_alphabetic() || x == b'_';

    // this loop parses the input bytes into a vec of raw tokens
    let mut i: usize = 0;
    let len = input.len();
    'outer: 
    while i < len {
        // numbers
        if is_number_part(input[i]) {
            let slice_bounds = get_token_bounds(is_number_part, i, &input);
            match String::from_utf8(input[slice_bounds.0..slice_bounds.1].to_vec()).unwrap().parse::<f64>() {
                Ok(n) => {
                    tokens.push(Number(n));
                }
                Err(_) => return Err("unable to parse number"),
            }
            i = slice_bounds.1;
        }
        // strings
        else if is_string_part(input[i]) {
            let slice_bounds: (usize, usize) = get_token_bounds(is_string_part, i, &input);
            // just stored as a string for now, will later be turned into a variable, function, or command
            tokens.push(UnresolvedString(input[slice_bounds.0..slice_bounds.1].to_vec()));
            i = slice_bounds.1;
        } else if input[i].is_ascii() {
            //operator token
            for j in (0..3).rev() {
                if i + j <= len {
                    if let Some(t) = match_token(&input[i..i + j]) {
                        tokens.push(t.clone());
                        i += j;
                        continue 'outer;
                    }
                }
            }
            return Err("unknown operator".into());
        } else {
            return Err("invalid input".into());
        }
    }

    
    

    // handle signs (negative, positive)
    let mut i: usize = 0;
    while i < tokens.len()-1 {
        match tokens[i] {
            Subtraction => {
                if let Number(n) = tokens[i + 1] {
                    if i == 0 {
                        tokens[i + 1] = Number(-n);
                        tokens.remove(i);
                    } else if tokens[i - 1] == Subtraction
                        || tokens[i - 1] == Addition
                        || tokens[i - 1] == Multiplication
                        || tokens[i - 1] == Division
                        || tokens[i - 1] == Exponentation
                        || tokens[i - 1] == Truncation
                        || tokens[i - 1] == Modulation
                        || tokens[i - 1] == Division
                        || tokens[i - 1] == LeftBracket
                        || tokens[i - 1] == RightBracket
                        || tokens[i - 1] == Assignment
                        || tokens[i - 1] == Comma
                    {
                        tokens[i + 1] = Number(-n);
                        tokens.remove(i);
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
            Addition => {
                if let Number(n) = tokens[i + 1] {
                    if i == 0 {
                        tokens[i + 1] = Number(n);
                        tokens.remove(i);
                    } else if tokens[i - 1] == Subtraction
                        || tokens[i - 1] == Addition
                        || tokens[i - 1] == Multiplication
                        || tokens[i - 1] == Division
                        || tokens[i - 1] == Exponentation
                        || tokens[i - 1] == Truncation
                        || tokens[i - 1] == Modulation
                        || tokens[i - 1] == Division
                        || tokens[i - 1] == LeftBracket
                        || tokens[i - 1] == RightBracket
                        || tokens[i - 1] == Assignment
                        || tokens[i - 1] == Comma
                    {
                        tokens[i + 1] = Number(n);
                        tokens.remove(i);
                    } else {
                        i += 1;
                    }
                } else {
                    i += 1;
                }
            }
            _ => {
                i += 1;
            }
        }
    }

    Ok(tokens)
}

fn resolve_strings(tokens: &mut Vec<Token>) {
    for i in 0..tokens.len() {
        if let UnresolvedString(name) = &tokens[i] {
            if i < tokens.len() - 1 {
                match tokens[i + 1] {
                    LeftBracket => tokens[i] = FunctionName(name.clone()),
                    _ => tokens[i] = Variable(name.clone()),
                }
            } else {
                tokens[i] = Variable(name.clone());
            }
        }
    }
}

pub trait TokenHandling {
    fn resolve_variables(&self, tokens: &mut Vec<Token>) -> Option<&'static str>; 
}

impl TokenHandling for CalcEngine {
    fn resolve_variables(&self, tokens: &mut Vec<Token>) -> Option<&'static str> {
        for token in tokens {
            if let Variable(name) = token {
                let hash_try = self.variables.get(name);
                match hash_try {
                    Some(number) => {
                        *token = number.clone();
                    }
                    None => {
                        return Some("Variable does not exist");
                    }
                }
            }
        }
        None
    }

    
}

fn get_token_bounds<F>(f: F, start: usize, input_chars: &[u8]) -> (usize, usize)
where
    F: Fn(u8) -> bool,
{
    let mut end: usize = start + 1;

    while end < input_chars.len() && f(input_chars[end]) {
        end += 1;
    }

    (start, end)
}
