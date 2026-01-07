#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenType {
    Number,
    Addition,
    Subtract,
    Multiply,
    Division,
    Exponent,
    ParenOpen,
    ParenClose,
}

#[allow(non_snake_case)]
pub mod TokenPrio {
    pub const NONE: i8  = 0;
    pub const ADD: i8   = 1;
    pub const SUB: i8   = 1;
    pub const MUL: i8   = 2;
    pub const DIV: i8   = 2;
    pub const EXP: i8   = 3;
    pub const PAR: i8   = 4;
    pub const MAX: i8   = 4;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Token {
    pub value: String,
    pub ttype: TokenType,
    pub prio: i8,
}
