pub mod ast;
pub mod lexer;
pub mod parser;
pub mod diagnostics;
pub mod codegen;
pub mod bytecode;
pub mod vm;
pub mod linker;

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;

use crate::ast::{Module, Statement};
use crate::codegen::CompiledModule;
use crate::bytecode::Program;

struct CapturingWriter(Rc<RefCell<Vec<u8>>>);

impl Write for CapturingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.borrow_mut().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

fn compile_source(source: &str, base_dir: Option<&Path>) -> Program {
    let asts = parse_all_modules(source, base_dir);

    let bytecode_modules: Vec<(String, CompiledModule)> = asts.into_iter()
        .map(|(path, ast)| (path, codegen::compile_module_from_ast(&ast)))
        .collect();

    linker::link(bytecode_modules)
}

fn parse_all_modules(source: &str, base_dir: Option<&Path>) -> Vec<(String, Module)> {
    let mut ordered_asts = Vec::new();
    let mut visited = HashSet::new();
    visited.insert("__main__".to_string());

    let main_ast = parse_module(source, "__main__");
    parse_module_deps(&main_ast, base_dir, &mut ordered_asts, &mut visited);
    ordered_asts.push(("__main__".to_string(), main_ast));
    ordered_asts
}

fn parse_module_deps(
    ast: &Module,
    base_dir: Option<&Path>,
    ordered_asts: &mut Vec<(String, Module)>,
    visited: &mut HashSet<String>,
) {
    for stmt in &ast.statements {
        let imp_path = match stmt {
            Statement::ImportFrom { path: p, .. } => Some(p.as_str()),
            Statement::ImportModule { path: p, .. } => Some(p.as_str()),
            _ => None,
        };
        if let Some(imp_path) = imp_path {
            if !visited.insert(imp_path.to_string()) { continue; }

            let file_path = module_file_path(imp_path, base_dir);
            let source = fs::read_to_string(&file_path)
                .unwrap_or_else(|_| panic!("cannot load module '{}'", imp_path));

            let dep_ast = parse_module(&source, imp_path);
            let dep_base = file_path.parent().map(|p| p.to_path_buf());
            parse_module_deps(&dep_ast, dep_base.as_deref(), ordered_asts, visited);
            ordered_asts.push((imp_path.to_string(), dep_ast));
        }
    }
}

fn parse_module(source: &str, name: &str) -> Module {
    let lexer::LexingArtifacts { tokens, .. } = lexer::tokenize(source);
    parser::parse_module(&tokens, &name.to_string()).ast
}

fn module_file_path(path: &str, base_dir: Option<&Path>) -> PathBuf {
    let mut p = if let Some(dir) = base_dir { dir.join(path) } else { PathBuf::from(path) };
    p.set_extension("water");
    p
}

pub fn run_capturing(source: &str) -> String {
    run_capturing_with_dir(source, None)
}

pub fn run_capturing_with_dir(source: &str, base_dir: Option<&Path>) -> String {
    let buf = Rc::new(RefCell::new(Vec::new()));
    let writer = CapturingWriter(buf.clone());
    let program = compile_source(source, base_dir);
    vm::exec_with(&program, Box::new(writer));
    String::from_utf8(Rc::try_unwrap(buf).unwrap().into_inner()).unwrap()
}

pub fn run_program(program_path: &str) {
    let source = match fs::read_to_string(program_path) {
        Ok(source) => source,
        Err(e) => { eprintln!("Failed to read program: {}", e); return; }
    };

    let base_dir = Path::new(program_path).parent();
    let program = compile_source(&source, base_dir);
    vm::exec(&program);
}
