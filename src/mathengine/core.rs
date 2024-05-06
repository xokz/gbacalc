use crate::mathengine::solve;
use crate::mathengine::token::{tokenize, Token, Token::*};
use agb::{hash_map, println};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use core::any;
use core::f64::consts::{E, PI, TAU};

use super::math_function::{Function, FunctionHandling};
use super::solve::Solver;
use super::token::TokenHandling;
use super::validate::{is_valid_lhs_function, validate_token_list};


pub struct CalcEngine {
    pub prev_answers: Vec<Token>,
    pub variables: agb::hash_map::HashMap<Vec<u8>, Token>,
    pub functions: agb::hash_map::HashMap<Vec<u8>, Function>,
    pub use_radians: bool,
}

impl CalcEngine {
    pub fn new() -> CalcEngine {
        let mut calc_engine = CalcEngine {
            prev_answers: Vec::new(),
            variables: agb::hash_map::HashMap::<Vec<u8>, Token>::new(),
            functions: agb::hash_map::HashMap::<Vec<u8>, Function>::new(),
            use_radians: true
        };
        calc_engine.prev_answers.push(Number(0.0));
        calc_engine.variables.insert("pi".as_bytes().to_vec(), Number(PI));
        calc_engine.variables.insert("e".as_bytes().to_vec(), Number(E));
        calc_engine.variables.insert("tau".as_bytes().to_vec(), Number(TAU));

        calc_engine
    }
}

pub trait Calc {
    fn eval(&mut self, input: Vec<u8>) -> Result<f64, &'static str>;
}

impl Calc for CalcEngine {
    fn eval(&mut self, input: Vec<u8>) -> Result<f64, &'static str> {
        // remove whitespace from input
        let mut trimmed_input: Vec<u8> = Vec::with_capacity(input.len());
        let trim_input = input.split(|x| *x == 32);
        for s in trim_input {
            trimmed_input.append(&mut s.to_vec());
        }

        // turn string input into a list of tokens
        let mut tokens;
        match tokenize(&trimmed_input) {
            Ok(token_vec) => {
                tokens = token_vec;
            }
            Err(e) => {
                return Err(e);
            }
        }

        // make sure token list is a valid equation or assignment
        match validate_token_list(&tokens) {
            Some(e) => {
                return Err(e);
            }
            None => (),
        }

        // if creating/reassigning a variable/function (expression contains a '=')
        if tokens.contains(&Assignment) {

            // split the expression into the parts before and after the '='
            let parts: Vec<_> = tokens.split(|t| t == &Assignment).collect();
            let mut lhs = parts[0].to_vec();
            let mut rhs = parts[1].to_vec();

            // check if lhs is a variable, or a function
            if lhs.len() == 1 {
                if let Variable(name) = &lhs[0] {
                    // lhs is a variable, assign value to new variable
                    // resolve variables on the rhs
                    match self.resolve_variables(&mut rhs) {
                        Some(e) => {
                            return Err(e);
                        }
                        _ => ()
                    }
                    let value;
                    match self.solve(rhs) {
                        Ok(answer) => value = answer[0].clone(),
                        Err(e) => return Err(e)
                    }
                    
                    self.variables.insert(name.clone(), value);
                    return Err("assigned value to variable");
                } else {
                    return Err("lhs must be variable or function");
                }
            } else {
                match is_valid_lhs_function(&lhs) {
                    None => {
                        // lhs is a function, assign value to new function
                        match self.create_function(&mut lhs, &mut rhs) {
                            Ok(func) => {
                                let func_name = func.name.clone();
                                self.functions.insert(func_name, func);
                                return Err("created function");
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    Some(e) => {
                        return Err(e);
                    }
                }
            }
		// otherwise just solve it
        } else {

            // resolve variables on the right side
            match self.resolve_variables(&mut tokens) {
                Some(e) => {
                    return Err(e);
                }
                _ => ()
            }
        
            let answer;
            match self.solve(tokens) {
                Ok(a) => answer = a[0].clone(),
                Err(e) => return Err(e)
            }

            if let Number(n) = answer {
                self.prev_answers[0] = Number(n);
                self.variables.insert(b"ans".to_vec(), self.prev_answers[0].clone());
                println!("answer: {}", n);
                return Ok(n);
            } else {
				return Err("Couldn't solve equation");
            }
        }
    }
}

pub struct TokenStringResult {
    pub length: usize,
    pub operator: Token,
}
