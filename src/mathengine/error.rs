use alloc::string::String;

pub enum CalcError {
    DivByZero,
    InvalidNumber(String),
    InvalidOperator(String),
    UnmatchedParenthesis,
}
