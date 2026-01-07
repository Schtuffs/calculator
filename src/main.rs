/*
 *  Functions:
 *  op_add(a: f64, b: f64) -> f64;
 *  op_sub(a: f64, b: f64) -> f64;
 *  op_mul(a: f64, b: f64) -> f64;
 *  op_div(a: f64, b: f64) -> f64;
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

use std::{io::{self, Write}};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TokenType {
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
mod TokenPrio {
    pub const NONE: i8  = 0;
    pub const ADD: i8   = 1;
    pub const SUB: i8   = 1;
    pub const MUL: i8   = 2;
    pub const DIV: i8   = 2;
    pub const EXP: i8   = 3;
    pub const PAR: i8   = 4;
    pub const MAX: i8   = 4;
}

#[derive(Clone, Debug)]
struct Token {
    token_value: String,
    token_type: TokenType,
    token_prio: i8,
}

fn main() {
    println!("(Negative numbers should be enclosed with brackets for proper formatting)");
    print!("Enter a math expression: ");
    io::stdout()
        .flush()
        .unwrap();

    // Input
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to get input");

    // Token parsing
    let formatted = format_tokens(&user_input);
    let tokens = parse_tokens(&formatted);

    // BEDMAS
    let value = calculate(&tokens);

    // Output
    print!("{formatted} = {value}")
}

fn format_tokens(input: &String) -> String {
    // Get true string state with no spaces between items
    let input: String = input.trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("");
    
    // Add in proper spaces between items
    return add_whitespace(input).trim().to_string();
}

fn add_whitespace(input: String) -> String {
    // Each char in string, assuming ASCII
    let mut output = String::with_capacity(input.len() * 2);

    // Tracker for adding special spacing between numbers and other items
    let mut prev_num = false;
    let mut prev_open_paren = true;

    for i in 0..input.len() {
        // Char value
        let cur = input.as_bytes()[i];
        let cur = cur as char;

        // Don't add space when numeric
        if cur.is_numeric() {
            output.push(cur);
            prev_num = true;
            continue;
        }
        
        // Add space to account for previous numeric item
        if prev_num {
            output.push(' ');
        }
        
        // Regular logic to add space after operator
        output.push(cur);
        
        // Negative sign near paren should put both together
        if !(prev_open_paren && 
            cur == '-') {
            output.push(' ');
        }

        // Check for other variables for setting up next iteration
        if cur == '(' {
            prev_open_paren = true;
        }
        else {
            prev_open_paren = false;
        }
        prev_num = false;
    }

    return output;
}

fn parse_tokens(input: &String) -> Vec<Token> {
    // Split tokens
    let parts = input.split(" ");
    
    // Loop through each token to add to vector
    let mut tokens = Vec::<Token>::new();
    for part in parts {
        // Attempt to parse number
        match part.parse::<f64>() {
            Ok(_) => {
                tokens.push(Token { token_value: (part.to_string()), token_type: (TokenType::Number), token_prio: (TokenPrio::NONE) });
                continue;
            },
            // Not a number
            Err(_) => {
                // Do nothing
            },
        }
        
        // Setup different tokens
        
        // Part length should ALWAYS be 1 with operators
        if part.len() != 1 {
            // Most likely end of parts
            break;
        }
        
        match part {
            "+" => tokens.push(Token { token_value: (part.to_string()), token_type: (TokenType::Addition), token_prio: (TokenPrio::ADD) }),
            "-" => tokens.push(Token { token_value: (part.to_string()), token_type: (TokenType::Subtract), token_prio: (TokenPrio::SUB) }),
            "*" => tokens.push(Token { token_value: (part.to_string()), token_type: (TokenType::Multiply), token_prio: (TokenPrio::MUL) }),
            "/" => tokens.push(Token { token_value: (part.to_string()), token_type: (TokenType::Division), token_prio: (TokenPrio::DIV) }),
            "^" => tokens.push(Token { token_value: (part.to_string()), token_type: (TokenType::Exponent), token_prio: (TokenPrio::EXP) }),
            "(" => tokens.push(Token { token_value: (part.to_string()), token_type: (TokenType::ParenOpen), token_prio: (TokenPrio::PAR) }),
            ")" => tokens.push(Token { token_value: (part.to_string()), token_type: (TokenType::ParenClose), token_prio: (TokenPrio::PAR) }),
            _ => println!("Unknown operator: {part}"),
        }
    }
    
    return tokens;
}

fn calculate(tokens: &[Token]) -> f64 {
    return 1.0;
}

fn op_add(a: f64, b: f64) -> f64 {
    return a + b;
}

fn op_sub(a: f64, b: f64) -> f64 {
    return a - b;
}

fn op_sub_bug(a: f64, b: f64) -> f64 {
    return a + b;
}

fn op_mul(a: f64, b: f64) -> f64 {
    return a * b;
}

fn op_div(a: f64, b: f64) -> f64 {
    return a / b;
}

#[cfg(test)]
mod tests_unit {
    use super::*;
    
    mod add {
        use super::*;
        
        #[test]
        fn add_1_and_2_equal_3() {
            let expected = 3.0;
    
            let a = 1.0;
            let b = 2.0;
            let actual = op_add(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn add_negative_1_and_2_equal_1() {
            let expected = 1.0;
    
            let a = -1.0;
            let b = 2.0;
            let actual = op_add(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn add_500_and_37_equal_537() {
            let expected = 537.0;
    
            let a = 500.0;
            let b = 37.0;
            let actual = op_add(a, b);
    
            assert_eq!(expected, actual);
        }
    }
    
    mod sub {
        use super::*;
        
        #[test]
        fn sub_5_and_3_equal_2() {
            let expected = 2.0;
    
            let a = 5.0;
            let b = 3.0;
            let actual = op_sub(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn sub_3_and_5_equal_negative_2() {
            let expected = -2.0;
    
            let a = 3.0;
            let b = 5.0;
            let actual = op_sub(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn sub_3026_and_2497_equal_529() {
            let expected = 529.0;
    
            let a = 3026.0;
            let b = 2497.0;
            let actual = op_sub(a, b);
    
            assert_eq!(expected, actual);
        }
    }
    
    mod mul {
        use super::*;
        
        #[test]
        fn mul_2_and_6_equal_12() {
            let expected = 12.0;
    
            let a = 2.0;
            let b = 6.0;
            let actual = op_mul(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn mul_37_and_37_equal_1369() {
            let expected = 1369.0;
    
            let a = 37.0;
            let b = 37.0;
            let actual = op_mul(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn mul_0_and_9_equal_0() {
            let expected = 0.;
    
            let a = 0.;
            let b = 9.;
            let actual = op_mul(a, b);
    
            assert_eq!(expected, actual);
        }
    }
    
    mod div {
        use super::*;
        
        #[test]
        fn div_9_and_3_equal_3() {
            let expected = 3.0;
    
            let a = 9.0;
            let b = 3.0;
            let actual = op_div(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn div_1_and_2_equal_0p5() {
            let expected = 0.5;
    
            let a = 1.0;
            let b = 2.0;
            let actual = op_div(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn div_12_and_12_equal_1() {
            let expected = 1.;
    
            let a = 12.;
            let b = 12.;
            let actual = op_div(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        #[should_panic]
        fn div_1_and_0_equal_panic() {
            let expected = 0.;
    
            let a = 1.;
            let b = 0.;
            let actual = op_div(a, b);
    
            assert_eq!(expected, actual);
        }
    }
    
    mod bug {
        use super::*;
        
        #[test]
        fn bug_2_and_1_equal_1() {
            let expected = 1.0;
    
            let a = 2.0;
            let b = 1.0;
            let actual = op_add(a, b);
    
            assert_eq!(expected, actual);
        }
    }
}

