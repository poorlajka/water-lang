use crate::parser::test_parser;
use water::ast::{self, Expression, Node, Pattern, Statement};

#[test]
fn test_assignment_integer() {
    let correct_ast = ast::Module {
        name: "main".into(),
        statements: vec![Statement::Expression(Node::new(
            0,
            0..0,
            Expression::Assignment {
                lhs: Node::new(
                    0,
                    0..0,
                    Pattern::Identifier("int_variable".into()),
                ),
                rhs: Box::new(Node::new(0, 0..0, Expression::Integer(5))),
            },
        ))],
    };

    test_parser("tests/programs/assignment_int.wtr", &correct_ast);
}

#[test]
fn test_assignment_float() {
    let correct_ast = ast::Module {
        name: "main".into(),
        statements: vec![Statement::Expression(Node::new(
            0,
            0..0,
            Expression::Assignment {
                lhs: Node::new(
                    0,
                    0..0,
                    Pattern::Identifier("float_variable".into()),
                ),
                rhs: Box::new(Node::new(0, 0..0, Expression::Float(3.15))),
            },
        ))],
    };

    test_parser("tests/programs/assignment_float.wtr", &correct_ast);
}
