use std::collections::HashMap;

use crate::bytecode::{CompiledFunction, Instruction, Opcode, Program};
use crate::codegen::CompiledModule;

pub fn link(modules: Vec<(String, CompiledModule)>) -> Program {
    let n = modules.len();

    // Assign base offsets. Index 0 = println, 1 = print.
    let mut fn_bases = vec![0usize; n];
    let mut str_bases = vec![0usize; n];
    let mut fn_cursor = 2usize;
    let mut str_cursor = 0usize;
    for i in 0..n {
        fn_bases[i] = fn_cursor;
        str_bases[i] = str_cursor;
        fn_cursor += modules[i].1.functions.len();
        str_cursor += modules[i].1.strings.len();
    }

    // Build the global export map: module_path -> { export_name -> absolute index }.
    let mut global_exports: HashMap<String, HashMap<String, usize>> = HashMap::new();
    for (i, (path, cm)) in modules.iter().enumerate() {
        let abs = cm.exports.iter()
            .map(|(name, &local)| (name.clone(), fn_bases[i] + local))
            .collect();
        global_exports.insert(path.clone(), abs);
    }

    // Patch relocations in every module.
    let mut modules = modules;
    for i in 0..n {
        let imports = modules[i].1.imports.clone();
        patch_code(&mut modules[i].1.main, fn_bases[i], str_bases[i], &imports, &global_exports);
        let fn_count = modules[i].1.functions.len();
        for j in 0..fn_count {
            patch_code(
                &mut modules[i].1.functions[j].code_block,
                fn_bases[i], str_bases[i], &imports, &global_exports,
            );
        }
    }

    // Main is the last module (topological order: deps before dependents).
    let main_code = modules.last().unwrap().1.main.clone();

    let placeholder = CompiledFunction { code_block: Vec::new() };
    let mut functions = vec![placeholder.clone(), placeholder]; // 0 = println, 1 = print
    for (_, cm) in &modules {
        functions.extend(cm.functions.iter().cloned());
    }
    let mut strings = Vec::new();
    for (_, cm) in &modules {
        strings.extend(cm.strings.iter().cloned());
    }

    Program { main: main_code, functions, strings }
}

fn patch_code(
    code: &mut Vec<Instruction>,
    fn_base: usize,
    str_base: usize,
    imports: &[(String, String)],
    global_exports: &HashMap<String, HashMap<String, usize>>,
) {
    for instr in code.iter_mut() {
        match instr.opcode {
            Opcode::LoadLocalFn => {
                instr.op1 = (fn_base + instr.op1 as usize) as u64;
                instr.opcode = Opcode::MovConst;
            }
            Opcode::LoadImport => {
                let idx = instr.op1 as usize;
                let (module_path, export_name) = &imports[idx];
                instr.op1 = global_exports
                    .get(module_path)
                    .and_then(|e| e.get(export_name))
                    .copied()
                    .unwrap_or_else(|| panic!("unresolved import {}::{}", module_path, export_name))
                    as u64;
                instr.opcode = Opcode::MovConst;
            }
            Opcode::LoadString => {
                instr.op1 = (str_base + instr.op1 as usize) as u64;
            }
            _ => {}
        }
    }
}
