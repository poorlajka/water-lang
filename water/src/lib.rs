pub mod ast;
pub mod lexer;
pub mod parser;
pub mod diagnostics;
pub mod codegen;
pub mod bytecode;
pub mod vm;

use std::fs;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;

use crate::diagnostics::{Diagnostic, Label, Severity, emitter};
use crate::bytecode::Program;

struct CapturingWriter(Rc<RefCell<Vec<u8>>>);

impl Write for CapturingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn compile_source(source: &str) -> Program {
    let mut diagnostics = Vec::new();

    /*
        Tokenize
    */
    let lexer::LexingArtifacts { tokens, errors } = lexer::tokenize(source);

    for (_, span) in &errors {
        diagnostics.push(Diagnostic {
            severity: Severity::Error,
            message: "lexing error".into(),
            labels: vec![Label { span: span.clone(), message: None }]
        });
    }

    /*
        Parse
    */
    let parser::ParsingArtifacts { ast, errors } =
        parser::parse_module(&tokens, &"main".to_string());

    for error in errors {
        diagnostics.push(Diagnostic {
            severity: Severity::Error,
            message: error.message,
            labels: vec![Label {
                span: error.span.unwrap_or(0..0),
                message: None,
            }],
        });
    }

    ast::display::print_ast(&ast);
    emitter::emit_diagnostics(source, &diagnostics);

    /*
        Compile
    */
    codegen::compile_module(&ast)
}

pub fn run_capturing(source: &str) -> String {
    let buf = Rc::new(RefCell::new(Vec::new()));
    let writer = CapturingWriter(buf.clone());
    let program = compile_source(source);
    vm::exec_with(&program, Box::new(writer));
    String::from_utf8(Rc::try_unwrap(buf).unwrap().into_inner()).unwrap()
}

pub fn run_program(program_path: &str) {

    /*
        Read source
    */
    let source = match fs::read_to_string(program_path) {
        Ok(source) => source,
        Err(e) => {
            eprintln!("Failed to read program: {}", e);
            return;
        }
    };

    /*
        Compile
    */
    let program = compile_source(&source);

    /*
        Execute
    */
    vm::exec(&program);
}
