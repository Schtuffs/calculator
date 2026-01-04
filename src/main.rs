/*
 *  Functions:
 *  op_add(a: f32, b: f32) -> f32;
 *  op_sub(a: f32, b: f32) -> f32;
 *  op_mul(a: f32, b: f32) -> f32;
 *  op_div(a: f32, b: f32) -> f32;
 *  
 *  Input:
 *  Math expression
 *
 *  Logic:
 *  1. Input
 *  2. Token Parse
 *  3. BEDMAS operations
 *  4. Output answer
 */

use std::io::Write;

// enum TokenType {
//     ADDITION,
//     SUBTRACT,
//     MULTIPLY,
//     DIVISION,
//     EXPONENT,
//     BRACKET
// }

// const PRIO_ADD: i8  = 1;
// const PRIO_SUB: i8  = 1;
// const PRIO_MUL: i8  = 2;
// const PRIO_DIV: i8  = 2;
// const PRIO_EXP: i8  = 3;
// const PRIO_BRA: i8  = 4;
// const PRIO_MAX: i8  = 4;

// struct Token {
//     token_type: TokenType,
//     token_prio: i8
// }

fn main() {
    print!("Enter a math expression: ");
    std::io::stdout().flush().unwrap();

    // Input
    let mut user_input = String::new();
    std::io::stdin().read_line(&mut user_input).expect("Failed to get input");
    println!("Value: {}", user_input);

    // Token parsing
    // let _value: Vec<Token> = parse_tokens(user_input);

    // BEDMAS

    // Output
}

// fn parse_tokens(input: String) -> Vec<Token> {
//     return Vec::<Token>::new();
// }

