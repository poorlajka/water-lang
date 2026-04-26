use crate::ast::{Expression, Module, Pattern, Statement};

pub fn print_ast(module: &Module) {
    println!("\nModule: {}", module.name);
    for statement in &module.statements {
        print_statement(statement, "", true);
    }
}

fn print_pattern(pat: &Pattern, prefix: &str, _is_last: bool) {
    if let Pattern::Identifier(lhs) = pat {
        print_expression(
            &Expression::Identifier(lhs.to_string()),
            prefix,
            false,
        );
    }
}

fn print_expression(expr: &Expression, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    print!("{prefix}{connector}");

    match expr {
        Expression::Assignment { lhs, rhs } => {
            println!("Assignment");

            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });

            print_pattern(&lhs.kind, prefix, is_last);
            print_expression(&rhs.kind, &new_prefix, true);
        }
        Expression::Integer(i) => {
            println!("Integer({})", i);
        }
        Expression::Array(a) => {
            println!("Array");
            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });
            for i in a {
                print_expression(&i.kind, &new_prefix, is_last);
            }
        }
        Expression::Tuple(a) => {
            println!("Tuple");
            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });
            for i in a {
                print_expression(&i.kind, &new_prefix, is_last);
            }
        }
        Expression::Index { target, index } => {
            println!("Index");
            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });
            print_expression(&target.kind, &new_prefix, false);
            print_expression(&index.kind, &new_prefix, true);
        }

        Expression::Float(f) => {
            println!("Float({})", f);
        }

        Expression::String(s) => {
            println!("String(\"{}\")", s);
        }

        Expression::Identifier(v) => {
            println!("Identifier({})", v);
        }

        Expression::Binary { lhs, op, rhs } => {
            println!("Binary({:?})", op);

            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });

            print_expression(&lhs.kind, &new_prefix, false);
            print_expression(&rhs.kind, &new_prefix, true);
        }

        Expression::Block {
            statements,
            final_expr,
        } => {
            println!("Block");

            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });

            for (i, stmt) in statements.iter().enumerate() {
                let last_stmt =
                    i == statements.len() - 1 && final_expr.is_none();
                print_statement(stmt, &new_prefix, last_stmt);
            }

            if let Some(expr) = final_expr {
                print_expression(&expr.kind, &new_prefix, true);
            }
        }

        Expression::Conditional {
            condition,
            then_branch,
            else_branch,
        } => {
            println!("If");

            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });

            print_expression(&condition.kind, &new_prefix, false);
            print_expression(
                &then_branch.kind,
                &new_prefix,
                else_branch.is_none(),
            );

            if let Some(else_expr) = else_branch {
                print_expression(&else_expr.kind, &new_prefix, true);
            }
        }
        Expression::FunctionCall { callee, arguments} => {
            println!("Functioncall");

            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });

            print_expression(&callee.kind, &new_prefix, is_last);

            for arg in arguments{
                print_expression(&arg.kind, &new_prefix, is_last);
            }

        }
        Expression::Function { signature, body } => {
            println!("Function");

            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });

            for arg in &signature.args {
                print_pattern(&arg.kind, &new_prefix, is_last);
            }

            print_expression(&body.kind, &new_prefix, is_last);
        }
        Expression::Unary { .. } => {}
        _ => {}
    }
}

fn print_statement(stmt: &Statement, prefix: &str, is_last: bool) {
    let connector = if is_last { "└── " } else { "├── " };
    print!("{prefix}{connector}");

    match stmt {
        Statement::Expression(expr) => {
            println!("Expression");
            let new_prefix =
                format!("{prefix}{}", if is_last { "    " } else { "│   " });
            print_expression(&expr.kind, &new_prefix, true);
        }
        _ => {
            println!("Other");
        }
    }
}
