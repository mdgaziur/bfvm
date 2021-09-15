use crate::parser::Module;
use bfvm::ir::IR;

pub fn codegen(module: &Module) -> IR {
    let mut ir = IR::new();

    for node in &module.body {
        ir.insert_instruction(node.to_ir_instruction());
    }

    ir
}