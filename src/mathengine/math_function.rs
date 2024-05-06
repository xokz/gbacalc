use core::f64::consts::PI;

use agb::{fixnum::Num, println};
use alloc::{string::String, vec::Vec};

use crate::mathengine::token::{self, Token::*};

use super::{core::{Calc, CalcEngine}, solve::Solver, token::{Token, TokenHandling}};

use libm::{sin, cos, tan, asin, acos, atan, sqrt, log, log2};

#[derive(Clone)]
pub struct Function {
    pub name: Vec<u8>,
    pub arg_count: usize,
    pub func: Vec<Token>,
}

impl Function {
    fn new() -> Function {
        Function {
            name: Vec::new(),
            arg_count: 0,
            func: Vec::new()
        }
    }
}

pub trait FunctionHandling {
    fn create_function(&mut self, lhs: &mut Vec<Token>, rhs: &mut Vec<Token>) -> Result<Function, &'static str>;
    fn solve_function(&mut self, name: &Vec<u8>, args: &[Token]) -> Result<Token, &'static str>;
}

impl FunctionHandling for CalcEngine {
    fn create_function(&mut self, lhs: &mut Vec<Token>, rhs: &mut Vec<Token>) -> Result<Function, &'static str> {
        let mut arg_count: usize = 0;
        let mut func = Function::new();
        for i in 0..lhs.len() {
            if let Variable(arg_name) = &lhs[i] {
                // replace cooresponding rhs variable(s) with a function argument index
                for j in 0..rhs.len() {
                    if let Variable(var_name) = &rhs[j] {
                        if arg_name == var_name {
                            rhs[j] = FunctionArg(arg_count);
                        }
                    }
                }

                arg_count += 1;
            }
        }

        self.resolve_variables(rhs);

        // assign proper values to func
        if let FunctionName(func_name) = &lhs[0] {
            func.name = func_name.clone();
        } else {
            return Err("function must have a name");
        }
        func.arg_count = arg_count;
        match self.resolve_variables(rhs) {
            None => (),
            _ => return Err("invalid variables"),
        }
        func.func = rhs.to_vec();

        Ok(func)
        
    }

    fn solve_function(&mut self, name: &Vec<u8>, arg_slice: &[Token]) -> Result<Token, &'static str> {
        let mut args: Vec<f64> = Vec::new();

        for arg in arg_slice.split(|t| t == &Comma) {
            if let Number(n) = self.solve(arg.to_vec()).unwrap()[0].clone() {
                args.push(n);
            }
            
        }

        let angle_mode = if self.use_radians {
            1.0
        } else {
            PI / 180.0
        };
        match String::from_utf8(name.clone()).unwrap().as_str() {
            "sin" => return Ok(Number(sin(args[0] * angle_mode))),
            "cos" => return Ok(Number(cos(args[0] * angle_mode))),
            "tan" => return Ok(Number(tan(args[0] * angle_mode))),
            "asin" => return Ok(Number(asin(args[0]) * angle_mode)),
            "acos" => return Ok(Number(acos(args[0]) * angle_mode)),
            "atan" => return Ok(Number(atan(args[0]) * angle_mode)),
            "sqrt" => return Ok(Number(sqrt(args[0]))),
            "log" => return Ok(Number(log(args[0]))),
            "ln" => return Ok(Number(log2(args[0]))),
            _ => {
                match self.functions.get(name) {
                    Some(function) => {
                        if function.arg_count != args.len() {
                            return Err("incorrect argument count");
                        }
                        let mut expr = function.func.clone();
                        for token in &mut expr {
                            if let FunctionArg(index) = token {
                                *token = Number(args[*index]);
                            }
                        }
                        match self.solve(expr) {
                            Ok(answer) => return Ok(answer[0].clone()),
                            Err(e) => return Err(e),
                        };
                    }
                    None => {
                        return Err("function does not exist")
                    }
                }
            }
        }
    }
}
