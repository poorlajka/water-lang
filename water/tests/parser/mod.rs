pub mod assignment;
use std::fs;
use water::{ast, lexer, parser};

pub fn test_parser(program: &str, correct_ast: &ast::Module) {
    /*
        Read code
    */
    let code = fs::read_to_string(program).expect("failed to read test file");

    let lexer::LexingArtifacts { tokens, errors } = lexer::tokenize(&code);

    assert!(errors.is_empty());

    let parser::ParsingArtifacts { ast, errors } =
        parser::parse_module(&tokens, &"main".to_string());

    assert!(errors.is_empty());

    assert_eq!(ast, *correct_ast);
}
