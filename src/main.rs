
mod lexer;
mod parser;

use crate::lexer::{lang_lexer, lang_token};
use crate::parser::lang_parser;
use crate::parser::print_ast::print_ast;

use std::fs;

fn main () {

    /*
        Read code 
    */
    let code = match fs::read_to_string("program.txt") {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Failed to read program: {}", e);
            return;
        }
    };

    /*
        Tokenize code
    */
    let lang_lexer::LexingArtifacts {
        tokens,
        errors,
    } = lang_lexer::tokenize(&code);


    if !errors.is_empty() {
        for error in &errors {
            println!("{:?}", error);
        }
        return;
    }

    for token in &tokens {
        println!("{:?}", token);
    }

    /*
        Parse tokens
    
    */
    let lang_parser::ParsingArtifacts {
        ast,
        errors,
    } = lang_parser::parse_module(&tokens, &"main".to_string());

    if !errors.is_empty() {
        for error in errors {
            println!("{:?}", error);
        }
    }

    print_ast(&ast);

    /*
    }
        */


    /*  
        Compile AST to bytecode
    
    let lang_compiler::CompilerArtifacts {
        bytecode,
        errors,
    } = lang_compiler::compile(&ast);
*/
    /*  
        Execute bytecode
    */
    //let exit_status = lang_vm::exec(&bytecode);
}
