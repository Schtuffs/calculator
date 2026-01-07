/*
 *  Functions:
 *  op_add(a: f64, b: f64) -> f64;
 *  op_sub(a: f64, b: f64) -> f64;
 *  op_mul(a: f64, b: f64) -> f64;
 *  op_exp(a: f64, b: f64) -> f64;
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

use std::io::{self, Write};

mod token;
use token::{Token, TokenType, TokenPrio};

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
    let value = calculate(tokens);

    // Output
    println!("{formatted} = {value}");
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
                tokens.push(Token { value: (part.to_string()), ttype: (TokenType::Number), prio: (TokenPrio::NONE) });
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
            "+" => tokens.push(Token { value: (part.to_string()), ttype: (TokenType::Addition), prio: (TokenPrio::ADD) }),
            "-" => tokens.push(Token { value: (part.to_string()), ttype: (TokenType::Subtract), prio: (TokenPrio::SUB) }),
            "*" => tokens.push(Token { value: (part.to_string()), ttype: (TokenType::Multiply), prio: (TokenPrio::MUL) }),
            "/" => tokens.push(Token { value: (part.to_string()), ttype: (TokenType::Division), prio: (TokenPrio::DIV) }),
            "^" => tokens.push(Token { value: (part.to_string()), ttype: (TokenType::Exponent), prio: (TokenPrio::EXP) }),
            "(" => tokens.push(Token { value: (part.to_string()), ttype: (TokenType::ParenOpen), prio: (TokenPrio::PAR) }),
            ")" => tokens.push(Token { value: (part.to_string()), ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR) }),
            _ => println!("Unknown operator: {part}"),
        }
    }
    
    return tokens;
}

fn calculate(tokens: Vec<Token>) -> f64 {
    let mut tokens = tokens;

    // Loop through tokens based on prio
    for cur_prio in (0..=TokenPrio::MAX).rev() {
        for i in 0..tokens.len() {
            if i >= tokens.len() {
                break;
            }

            let token = &tokens[i];
            if cur_prio == token.prio {
                // Manage brackets
                if token.ttype == TokenType::ParenOpen {
                    calculate_brackets(&mut tokens, i);
                    continue;
                }

                // Manage other
                
                // Cannot have an operand as first or last token
                if i == 0 || i == tokens.len() - 1 {
                    continue;
                }

                // Parse values first
                let a: f64 = tokens[i - 1].value.parse::<f64>().expect("ERROR: Could not parse f64");
                let b: f64 = tokens[i + 1].value.parse::<f64>().expect("ERROR: Could not parse f64");

                // Match operation
                match token.ttype {
                    TokenType::Addition => swap_expression(&mut tokens, i - 1, i + 1, op_add(a, b)),
                    TokenType::Subtract => swap_expression(&mut tokens, i - 1, i + 1, op_sub(a, b)),
                    TokenType::Multiply => swap_expression(&mut tokens, i - 1, i + 1, op_mul(a, b)),
                    TokenType::Division => swap_expression(&mut tokens, i - 1, i + 1, op_div(a, b)),
                    TokenType::Exponent => swap_expression(&mut tokens, i - 1, i + 1, op_exp(a, b)),
                    _ => println!("Unknown token type: {:?}", token.ttype),
                }
            }
        }
    }

    match tokens[0].value.parse::<f64>() {
        Ok(number) => number,
        Err(_) => {
            println!("ERROR: Could not parse f64");
            0.
        }
    }
}

fn calculate_brackets(tokens: &mut Vec<Token>, begin: usize) {
    // Find end of bracket position
    let mut end = tokens.len();
    for i in begin + 1..tokens.len() {
        let token = &tokens[i];
        if token.ttype == TokenType::ParenOpen {
            // Recursive call to handle sub parentheses
            calculate_brackets(tokens, i);
        }
        else if token.ttype == TokenType::ParenClose {
            end = i;
            break;
        }
    }
    
    // Calculate items between these indexes
    let sub_vec: Vec<Token> = tokens[begin+1..end].to_vec();
    let sub_val = calculate(sub_vec);

    // Calculate items between these indexes
    swap_expression(tokens, begin, end, sub_val);
}

fn swap_expression(tokens: &mut Vec<Token>, begin: usize, end: usize, value: f64) {
    // Remove items between begin and end in tokens and swap with final value
    tokens.drain(begin..=end);
    tokens.insert(begin, Token { value: (value.to_string()), ttype: (TokenType::Number), prio: (TokenPrio::NONE) });
}

fn op_add(a: f64, b: f64) -> f64 {
    return a + b;
}

fn op_sub(a: f64, b: f64) -> f64 {
    return a - b;
}

fn op_mul(a: f64, b: f64) -> f64 {
    return a * b;
}

fn op_exp(a: f64, b: f64) -> f64 {
    a.powf(b)
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
    
    mod exp {
        use super::*;

        #[test]
        fn exp_2_and_2_equal_4() {
            let expected = 4.0;
    
            let a = 2.0;
            let b = 2.0;
            let actual = op_exp(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn exp_4_and_1p5_equal_8() {
            let expected = 8.0;
    
            let a = 4.0;
            let b = 1.5;
            let actual = op_exp(a, b);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn exp_27_and_0p33_equal_3() {
            let expected = 3.0;
    
            let a = 27.0;
            let b = 1.0 / 3.0;
            let actual = op_exp(a, b);
    
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

    mod format_tokens {
        use super::*;

        #[test]
        fn format_none_required() {
            let expected = "1";
    
            let input = String::from("1");
            let actual = format_tokens(&input);
    
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn format_initial_negative() {
            let expected = "-1";
    
            let input = String::from(" -   1");
            let actual = format_tokens(&input);
    
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn format_3_args() {
            let expected = "-1 + 2";
    
            let input = String::from("-   1 +2");
            let actual = format_tokens(&input);
    
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn format_paren() {
            let expected = "( 1 )";
    
            let input = String::from("     (1)   ");
            let actual = format_tokens(&input);
    
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn format_paren_negative() {
            let expected = "( -1 )";
    
            let input = String::from("     (-1)   ");
            let actual = format_tokens(&input);
    
            assert_eq!(expected, actual);
        }

        #[test]
        fn format_many() {
            let expected = "-1 + 2 * ( -3 - ( -5 ) ) ^ ( 4 )";

            let input = String::from("       -1   +  2*(-   3 -  (-5))^(     4)  ");
            let actual = format_tokens(&input);

            assert_eq!(expected, actual);
        }
    }

    mod parse_tokens {
        use super::*;

        #[test]
        fn parse_1_token() {
            let expected = vec![
                Token { value: (String::from("1")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
            ];
            
            let input = String::from("1");
            let actual = parse_tokens(&input);
    
            assert_eq!(expected.len(), actual.len());
            for i in 0..expected.len() {
                assert_eq!(&expected[i], &actual[i]);
            }
        }
        
        #[test]
        fn parse_3_tokens() {
            let expected = vec![
                Token { value: (String::from("109")),   ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("*")),     ttype: (TokenType::Multiply),   prio: (TokenPrio::MUL)  },
                Token { value: (String::from("15")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
            ];
            
            let input = String::from("109 * 15");
            let actual = parse_tokens(&input);
    
            assert_eq!(expected.len(), actual.len());
            for i in 0..expected.len() {
                assert_eq!(&expected[i], &actual[i]);
            }
        }
        
        #[test]
        fn parse_parens() {
            let expected = vec![
                Token { value: (String::from("1")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("+")),     ttype: (TokenType::Addition),   prio: (TokenPrio::ADD)  },
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("3")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
            ];
            
            let input = String::from("1 + ( 3 )");
            let actual = parse_tokens(&input);
    
            assert_eq!(expected.len(), actual.len());
            for i in 0..expected.len() {
                assert_eq!(&expected[i], &actual[i]);
            }
        }
        
        #[test]
        fn parse_exponents() {
            let expected = vec![
                Token { value: (String::from("2")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("^")),     ttype: (TokenType::Exponent),   prio: (TokenPrio::EXP)  },
                Token { value: (String::from("4")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
            ];
            
            let input = String::from("2 ^ 4");
            let actual = parse_tokens(&input);
    
            assert_eq!(expected.len(), actual.len());
            for i in 0..expected.len() {
                assert_eq!(&expected[i], &actual[i]);
            }
        }
        
        #[test]
        fn parse_negatives() {
            let expected = vec![
                Token { value: (String::from("-1")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("-")),     ttype: (TokenType::Subtract),   prio: (TokenPrio::SUB)  },
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("-1")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
            ];
            
            let input = String::from("-1 - ( -1 )");
            let actual = parse_tokens(&input);
    
            assert_eq!(expected.len(), actual.len());
            for i in 0..expected.len() {
                assert_eq!(&expected[i], &actual[i]);
            }
        }

        #[test]
        fn parse_many() {
            let expected = vec![
                Token { value: (String::from("-1")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("+")),     ttype: (TokenType::Addition),   prio: (TokenPrio::ADD)  },
                Token { value: (String::from("2")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("*")),     ttype: (TokenType::Multiply),   prio: (TokenPrio::MUL)  },
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("-3")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("-")),     ttype: (TokenType::Subtract),   prio: (TokenPrio::SUB)  },
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("-5")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
                Token { value: (String::from("^")),     ttype: (TokenType::Exponent),   prio: (TokenPrio::EXP)  },
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("4")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
            ];
                
            let input = String::from("-1 + 2 * ( -3 - ( -5 ) ) ^ ( 4 )");
            let actual = parse_tokens(&input);
            
            assert_eq!(expected.len(), actual.len());
            for i in 0..expected.len() {
                assert_eq!(&expected[i], &actual[i]);
            }
        }
    }
    
    mod calculate {
        use super::*;
        
        #[test]
        fn calc_1_equal_1() {
            let expected: f64 = 1.;

            let input = vec![
                Token { value: (String::from("1")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                ];
            let actual = calculate(input);
            
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn calc_15_plus_19_equal_34() {
            let expected: f64 = 34.;

            let input = vec![
                Token { value: (String::from("15")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("+")),     ttype: (TokenType::Addition),   prio: (TokenPrio::ADD)  },
                Token { value: (String::from("19")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
            ];
            let actual = calculate(input);
    
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn calc_20_minus_12_equal_8() {
            let expected: f64 = 8.;

            let input = vec![
                Token { value: (String::from("20")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("-")),     ttype: (TokenType::Subtract),   prio: (TokenPrio::SUB)  },
                Token { value: (String::from("12")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
            ];
            let actual = calculate(input);
            
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn calc_3_times_4_equal_12() {
            let expected: f64 = 12.;

            let input = vec![
                Token { value: (String::from("3")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("*")),     ttype: (TokenType::Multiply),   prio: (TokenPrio::MUL)  },
                Token { value: (String::from("4")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
            ];
            let actual = calculate(input);
            
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn calc_6_divide_2_equal_3() {
            let expected: f64 = 3.;

            let input = vec![
                Token { value: (String::from("6")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("/")),     ttype: (TokenType::Division),   prio: (TokenPrio::DIV)  },
                Token { value: (String::from("2")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
            ];
            let actual = calculate(input);
            
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn calc_2_exp_3_equal_8() {
            let expected: f64 = 8.;

            let input = vec![
                Token { value: (String::from("2")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("^")),     ttype: (TokenType::Exponent),   prio: (TokenPrio::EXP)  },
                Token { value: (String::from("3")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
            ];
            let actual = calculate(input);
            
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn calc_opar_negative_3_plus_5_equal_2() {
            let expected: f64 = 2.;

            let input = vec![
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("-3")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("+")),     ttype: (TokenType::Addition),   prio: (TokenPrio::ADD)  },
                Token { value: (String::from("5")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
            ];
            let actual = calculate(input);
                
            assert_eq!(expected, actual);
        }
        
        #[test]
        fn calc_many_equal_31() {
            let expected: f64 = 31.;

            let input = vec![
                Token { value: (String::from("-1")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("+")),     ttype: (TokenType::Addition),   prio: (TokenPrio::ADD)  },
                Token { value: (String::from("2")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("*")),     ttype: (TokenType::Multiply),   prio: (TokenPrio::MUL)  },
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("-3")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from("-")),     ttype: (TokenType::Subtract),   prio: (TokenPrio::SUB)  },
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("-5")),    ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
                Token { value: (String::from("^")),     ttype: (TokenType::Exponent),   prio: (TokenPrio::EXP)  },
                Token { value: (String::from("(")),     ttype: (TokenType::ParenOpen),  prio: (TokenPrio::PAR)  },
                Token { value: (String::from("4")),     ttype: (TokenType::Number),     prio: (TokenPrio::NONE) },
                Token { value: (String::from(")")),     ttype: (TokenType::ParenClose), prio: (TokenPrio::PAR)  },
            ];
            let actual = calculate(input);
            
            assert_eq!(expected, actual);
        }
    }
}

