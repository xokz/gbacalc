use agb::println;
use alloc::vec::Vec;

use crate::mathengine::token::Token::{self, *};

// just all of the operator tokens as a macro so I dont have to type all of them every time
#[macro_export]
macro_rules! operators {
    () => {
        Assignment
            | Addition
            | Subtraction
            | Division
            | Multiplication
            | Truncation
            | Modulation
            | Exponentation
            | BitwiseLeftShift
            | BitwiseRightShift
            | BitwiseAnd
            | BitwiseOr
            | BitwiseXor
    };
}

pub fn validate_token_list(tokens: &Vec<Token>) -> Option<&'static str> {
    // make sure input is not empty
    if tokens.len() == 0 {
        return Some("")
    }

    // make sure there are one or less '='
    let mut equal_count = 0;
    for token in tokens {
        if token == &Assignment {
            equal_count += 1;
        }
    }
    if equal_count > 1 {
        return Some("cannot have more than 1 \'=\'")
    }

    // make sure first and last tokens are valid
    match tokens[0] {
        Comma | RightBracket | operators!() => return Some("invalid first token"),
        _ => (),
    }
    match tokens[tokens.len() - 1] {
        Comma | LeftBracket | FunctionName(_) | operators!() => return Some("invalid last token"),
        _ => (),
    }

    // make sure all tokens are proceeded by a valid token
    for i in 0..tokens.len() - 1 {
        if !is_next_token_valid(&tokens[i], &tokens[i + 1]) {
            return Some("invalid token sequence");
        }
    }

    // make sure all brackets are closed
    let mut bracket_depth: i32 = 0;
    for token in tokens {
        match token {
            LeftBracket => bracket_depth += 1,
            RightBracket => bracket_depth -= 1,
            _ => (),
        }
    }
    if bracket_depth != 0 {
        return Some("imbalanced brackets");
    }
    bracket_depth = 0;

    // make sure commas are only found inside of functions
    let mut in_function_brackets: bool = false;
    for token in tokens {
        match token {
            FunctionName(_) => {
                in_function_brackets = true;
            }
            LeftBracket => {
                if in_function_brackets {
                    bracket_depth += 1;
                }
            }
            RightBracket => {
                if in_function_brackets {
                    bracket_depth -= 1;
                    if bracket_depth == 0 {
                        in_function_brackets = false;
                    }
                }
            }
            Comma => {
                if !in_function_brackets {
                    return Some("commas only go in functions");
                }
            }
            _ => {}
        }
    }
    
    // token list is valid and ready for solving
    None
}

fn is_next_token_valid(current: &Token, next: &Token) -> bool {
    match current {
        // current token is a number or var
        Number(_) | Variable(_) => {
            // if next token matches any of these, its invalid
            match next {
                Number(_) => return false,
                Variable(_) => return false,
                FunctionName(_) => return false,
                LeftBracket => return false,
                _ => return true,
            };
        }

        // current token is an operator
        operators!() => {
            // if next token matches any of these, its invalid
            match next {
                operators!() => return false,
                RightBracket => return false,
                Comma => return false,
                _ => return true,
            };
        }

        // current token is a '('
        LeftBracket => {
            // if next token matches any of these, its invalid
            match next {
                operators!() => return false,
                Comma => return false,
                _ => return true,
            };
        }

        // current token is a ')'
        RightBracket => {
            // if next token matches any of these, its invalid
            match next {
                Number(_) => return false,
                Variable(_) => return false,
                FunctionName(_) => return false,
                LeftBracket => return false,
                _ => return true,
            };
        }

        // current token is a function
        FunctionName(_) => {
            // the only valid next token is a left bracket
            match next {
                LeftBracket => return true,
                _ => return false,
            };
        }

        // current token is a ','
        Comma => {
            // if next token matches any of these, its invalid
            match next {
                operators!() => return false,
                Comma => return false,
                RightBracket => return false,
                _ => return true,
            };
        }

        _ => return false,
    };

}

pub fn is_valid_lhs_function(tokens: &Vec<Token>) -> Option<&'static str> {
    println!("{:?}", tokens);
    // make sure first token is a function name that is not reserved
    match &tokens[0] {
        FunctionName(name) => {
            match &name[..] {
                b"sin" => return Some("sin() cannot be reassigned"),
                b"cos" => return Some("cos() cannot be reassigned"),
                b"tan" => return Some("tan() cannot be reassigned"),
                b"asin" => return Some("asin() cannot be reassigned"),
                b"acos" => return Some("acos() cannot be reassigned"),
                b"atan" => return Some("atan() cannot be reassigned"),
                b"sqrt" => return Some("sqrt() cannot be reassigned"),
                b"fact" => return Some("fact() cannot be reassigned"),
                b"log" => return Some("log() cannot be reassigned"),
                b"ln" => return Some("ln() cannot be reassigned"),
                _ => ()
            }
        }
        _ => return Some("Not a function"),
    }

    // args must be encased in brackets
    if &tokens[1] != &LeftBracket {
        return Some("func args must be in brackets");
    }
    if &tokens[tokens.len()-1] != &RightBracket {
        return Some("func args must be in brackets");
    }

    // contents of brackets must be variables and commas in alternating order
    // eg. func(a,b,c)
    let mut prev_token_was_arg: bool = false;

    for token in &tokens[2..tokens.len()-1] {
        if prev_token_was_arg {
            match token {
                Comma => {
                    prev_token_was_arg = false;
                    continue;
                }
                _ => return Some("expected comma")
            }
        } else {
            match token {
                Variable(name) => {
                    prev_token_was_arg = true;
                    continue;
                }
                _ => return Some("expected arg")
            }
        }
    }

    None
}
