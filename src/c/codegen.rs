use crate::ir::{IR, Instruction};

fn internal_codegen(instruction: &Instruction, code: &mut String) {
    match instruction {
        Instruction::DpInc => *code += "++ptr;",
        Instruction::DpDec => *code += "--ptr;",
        Instruction::ValInc => *code += "++array[ptr];",
        Instruction::ValDec => *code += "--array[ptr];",
        Instruction::Output => *code += "putchar(array[ptr]);",
        Instruction::Input => *code += "array[ptr] = getchar();",
        Instruction::LoopBody(body) => {
            *code += "while (array[ptr]) {";
            for ins in body {
                internal_codegen(ins, code);
            }
            *code += "};";
        },
        Instruction::Module(_) => panic!("Invalid IR: multiple Module found")
    }
}

pub fn codegen(ir: &IR) -> String {
    let mut result: String = String::from("\
    // This code was generated using the BFVM brainfuck compiler toolchain\n\
    #include <stdio.h>\n\
    void main()\
    {\
        char array[30000] = {0};\
        int ptr = 0;\
    ");

    if let Instruction::Module(module) = &ir.module {
        for ins in module {
            internal_codegen(ins, &mut result);
        }
    } else {
        panic!("No module at the root of IR");
    }

    result += "}";
    result
}