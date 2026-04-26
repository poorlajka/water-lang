pub mod ast;
pub mod lexer;
pub mod parser;
pub mod diagnostics;
pub mod codegen;
pub mod bytecode;
pub mod vm;

use std::fs;

use crate::diagnostics::{Diagnostic, Severity, Label, emitter};

pub fn run_program(program_path: &str) {

    /*
        Read code
    */
    let code = match fs::read_to_string(program_path) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read program: {}", e);
            return;
        }
    };

    let mut diagnostics = Vec::new();

    /*
        Tokenize code
    */
    let lexer::LexingArtifacts { tokens, errors } = lexer::tokenize(&code);

    /* 
    for t in &tokens {
        println!("{:?}", t);
    }
    */

    for (error, span) in &errors {
        diagnostics.push(
            Diagnostic {
                severity: Severity::Error,
                message: "lexing error".into(),
                labels: vec![Label{ span: span.clone(), message: None}]
            }
        );
    }

    /*
        Parse tokens

    */
    let parser::ParsingArtifacts { ast, errors } =
        parser::parse_module(&tokens, &"main".to_string());

    for error in errors {
        diagnostics.push(
            Diagnostic {
                severity: Severity::Error,
                message: error.message,
                labels: vec![
                    Label {
                        span: error.span.unwrap_or(0..0),
                        message: None,
                    },
                ],
            }
        );
    }

    ast::display::print_ast(&ast);

    emitter::emit_diagnostics(&code, &diagnostics);

    /*
            Compile AST to bytecode

    */
    let bytecode = codegen::compile_module(&ast);

    /*
        Execute bytecode
    */
    vm::exec(&bytecode.main, &bytecode.functions);
}