use crate::parser::lang_ast::{Expression, Statement, BinaryOp, Module, Assignment, Function};

pub fn print_ast(module: &Module) {

    println!("\nModule: {}", module.name);
    for statement in &module.statements {
        print_statement(&statement, "", true);
    }
}

fn print_expression(expr: &Expression, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    print!("{prefix}{connector}");

    match expr {
        Expression::Integer(i) => {
            println!("Integer({})", i);
        }

        Expression::Float(f) => {
            println!("Float({})", f);
        }

        Expression::String(s) => {
            println!("String(\"{}\")", s);
        }

        Expression::Variable(v) => {
            println!("Variable({})", v);
        }

        Expression::Binary { lhs, op, rhs } => {
            println!("Binary({:?})", op);

            let new_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });

            print_expression(lhs, &new_prefix, false);
            print_expression(rhs, &new_prefix, true);
        }

        Expression::Block { statements, final_expr } => {
            println!("Block");

            let new_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });

            for (i, stmt) in statements.iter().enumerate() {
                let last_stmt = i == statements.len() - 1 && final_expr.is_none();
                print_statement(stmt, &new_prefix, last_stmt);
            }

            if let Some(expr) = final_expr {
                print_expression(expr, &new_prefix, true);
            }
        }

        Expression::Conditional { condition, then_branch, else_branch } => {
            println!("If");

            let new_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });

            print_expression(condition, &new_prefix, false);
            print_expression(then_branch, &new_prefix, else_branch.is_none());

            if let Some(else_expr) = &**else_branch {
                print_expression(else_expr, &new_prefix, true);
            }
        }
        Expression::FunctionCall(_) => {

        }
        Expression::Unary{..} => {

        }
    }
}

fn print_statement(stmt: &Statement, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    print!("{prefix}{connector}");

    match stmt {
        Statement::Expression(expr) => {
            println!("Expression");
            let new_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });
            print_expression(expr, &new_prefix, true);
        }
        Statement::Assignment( Assignment {lhs, rhs}) => {
            println!("Assignment");

            let new_prefix = format!("{prefix}{}", if is_last { "    " } else { "│   " });

            print_expression(&Expression::Variable(lhs.to_string()), &new_prefix, false);
            print_expression(rhs, &new_prefix, true);

        }
        Statement::Function(_) => {

        }
    }
}