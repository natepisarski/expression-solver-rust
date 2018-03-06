use std::fmt::{Display};
use std::vec::Vec;

/// Defines the different binary operations that could appear in an expression
#[derive(Copy, Clone, Debug)]
pub enum Operations {

    /// 1+2
    Add,

    /// 1-2
    Subtract,

    /// 1*2
    Multiply,

    /// 1/2
    Divide,

    ///1^2
    Power
}

/// Represents something that can appear in a valid expression.
#[derive(Copy, Clone, Debug)]
pub enum ExpressionAtom {

    /// e.g: 1, 5
    Number(u32),

    /// e.g: *, /
    Operation(Operations),

    /// i.e (
    LeftParenthesis,

    /// i.e )
    RightParenthesis
}

impl Display for ExpressionAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            &ExpressionAtom::Number(num) => write!(f, "Number({})", num),
            &ExpressionAtom::Operation(op) => write!(f, "Operation({})", op),
            &ExpressionAtom::LeftParenthesis => write!(f, "LPAREN"),
            &ExpressionAtom::RightParenthesis => write!(f, "RPAREN")
        };
    }
}

impl Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return match self {
            &Operations::Add => write!(f, "ADD"),
            &Operations::Subtract => write!(f, "SUBTRACT"),
            &Operations::Multiply => write!(f, "MULTIPLY"),
            &Operations::Divide => write!(f, "DIVIDE"),
            &Operations::Power => write!(f, "POWER")
        }
    }
}

/*
impl Display for Vec<ExpressionAtom> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut amalgation: String = "".to_string();
        for atom in self {
            amalgation = amalgation + atom.to_string()
        }
    }
}
*/
/// What order operations are calculated in
pub const ORDER_OF_OPERATIONS: &[Operations] =
    &[Operations::Power, Operations::Multiply, Operations::Divide, Operations::Add, Operations::Subtract];

/// Given a character representing a mathematical operation, turn it into the Operations enum.
pub fn turn_into_operation(operation_character: char) -> Option<Operations> {
    match operation_character {
        '+' => Some(Operations::Add),
        '-' => Some(Operations::Subtract),
        '*' => Some(Operations::Multiply),
        '/' => Some(Operations::Divide),
        '^' => Some(Operations::Power),
        _ => None
    }
}

/// Given an operation, turns it into its character equivalent
pub fn turn_into_character(operations: Operations) -> char {
    match operations {
        Operations::Add => '+',
        Operations::Subtract => '-',
        Operations::Multiply => '*',
        Operations::Divide => '/',
        Operations::Power => '^'
    }
}

/// Calculates two numbers with the Operations enum
pub fn calculate(lval: u32, rval: u32, operation: Operations) -> u32 {
    match operation {
        Operations::Add => lval + rval,
        Operations::Subtract => lval - rval,
        Operations::Multiply => lval * rval,
        Operations::Divide => lval / rval,
        Operations::Power => lval ^ rval
    }
}

/// The data structure which handles the computation.
///     Feeding it [1, +, 2] will make the result be '3'
///     Feeding it [3 / 4] will make the result '.75'
/// This data structure cannot handle order of operations
pub struct ExpressionStack {
    pub operation: Option<Operations>,
    pub left_value: Option<u32>,
    pub right_value: Option<u32>
}

/// The stack can accept multiple types. This lets us overload 'accept'
trait MultiValuedStack {
    type AcceptionType;
    fn accept(&mut self, item: Self::AcceptionType);
}

/// Allows the stack to calculate the value
impl ExpressionStack {
    pub fn calculate(&self) -> u32 {
        if let Some(left) = self.left_value {
            if let Some(right) = self.right_value {
                if let Some(operation) = self.operation {
                    calculate(left, right, operation);
                }
            }
        }
        panic!("Cannot calculate value without a lval, rval, and operation");
    }
}
/*
/// Allows the stack to take a number for the left and right position
impl MultiValuedStack for ExpressionStack {
    type AcceptionType = u32;

    fn accept(&mut self, number: u32) {
        if self.left_value.is_some() {
            self.right_value = number;
        } else {
            self.left_value = number
        }
    }
}

/// Allows the stack to take an operation
impl MultiValuedStack for ExpressionStack {
    type AcceptionType = Operations;

    fn accept(&mut self, operation: Operations) {
        self.operation = operation;
    }
}
*/
/// Represents something that can be passed through the stack. This also doesn't account for
/// PEMDAS or parenthesis
pub struct OperationTokenTree {
    tokens: Vec<ExpressionAtom>
}

impl OperationTokenTree {
    pub fn evaluate_tokens(expression: &str) -> Vec<ExpressionAtom> {

        // This will read as-is and do no processing
        /*
        EXAMPLES:
            * 1 + 2 = N(1), O(Add), N(2)
            * 1 + (1 + 2) = N(1), O(Add), LP, N(1), O(Add), N(2), RP
            * 1 + (1 + (1 + 2)) = N(1), O(Add), LP, N(1), O(Add), LP, N(1), O(Add), N(2), RP, RP
        */

        // The finished tokens
        let mut tokens: Vec<ExpressionAtom> = vec![];

        // Initial tokens. So, '12 + 4' will be N(1), N(2), O(Add), N(4)'. The second pass makes the 12
        let mut initial_tokenization: Vec<ExpressionAtom> = vec![];

        let expression_characters = expression.chars();
        for character in expression_characters {
            if character.is_numeric() {
                initial_tokenization.push(ExpressionAtom::Number(character.to_digit(10).unwrap()));
            }
            if let Some(op) = turn_into_operation(character) {
                initial_tokenization.push(ExpressionAtom::Operation(op));
            }
            if character.eq(&'(') {
                initial_tokenization.push(ExpressionAtom::LeftParenthesis);
            }
            if character.eq(&')') {
                initial_tokenization.push(ExpressionAtom::RightParenthesis);
            }
        }

        // Condenses the initial tokenization into the proper form
        let mut building_number: Option<ExpressionAtom> = None;
        for token in initial_tokenization {
            match token {
                ExpressionAtom::RightParenthesis | ExpressionAtom::LeftParenthesis | ExpressionAtom::Operation(_)
                    => {
                    if let Some(number) = building_number {
                        tokens.push(number);
                        building_number = None;
                    }
                        tokens.push(token);
                },
                _ => {
                    if let Some(number) = building_number {
                        if let ExpressionAtom::Number(old_number) = number {
                            if let ExpressionAtom::Number(new_number) = token {
                                building_number =
                                    Some(ExpressionAtom::Number((old_number.to_string() + &new_number.to_string()).parse::<u32>().unwrap()));
                            }
                        }
                    } else {
                        building_number = Some(token);
                    }
                }
            }
        }

        if let Some(number) = building_number {
            tokens.push(number);
        }
        return tokens;
    }
}

#[cfg(test)]
mod tests {
    use self::super::*;
    use OperationTokenTree;
    use std::vec::*;
    #[test]
    fn test_tokenizer() {
        let t_tree = OperationTokenTree{tokens: vec![]};

        let token_stream: Vec<ExpressionAtom> = OperationTokenTree::evaluate_tokens(
            "1+(1+(2+4+5666))"
        );
        println!("{:?}", token_stream);
    }
}
