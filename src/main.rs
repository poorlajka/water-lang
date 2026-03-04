mod lexer;
mod parser;

use std::fs;

fn main() {
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
    let lexer::LexingArtifacts { tokens, errors } = lexer::tokenize(&code);

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
    let parser::ParsingArtifacts { ast, errors } =
        parser::parse_module(&tokens, &"main".to_string());

    if !errors.is_empty() {
        for error in errors {
            println!("{:?}", error);
        }
    }

    parser::display::print_ast(&ast);

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
