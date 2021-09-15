#[derive(Debug)]
pub enum Instruction {
    DpInc,
    DpDec,
    ValInc,
    ValDec,
    Output,
    Input,
    LoopBody(Vec<Instruction>),

    /// The actual module
    ///
    /// Even if it contains the same datatype as LoopBody,
    /// it's treated differently. Each IR can only have one Module at the
    /// beginning. This variant contains the actual instructions
    Module(Vec<Instruction>),
}

/// #Intermediate representation for BF code
///
/// This struct will be used by the assembler(for vm) and the C codegen to generate codes
#[derive(Debug)]
pub struct IR {
    pub module: Instruction
}

/// This trait allows any arbitrary instruction to get converted to BFVM IR instruction
/// Helper trait for the user
pub trait ToIRInstruction {
    /// Converts instruction to BFVM IR instruction
    fn to_ir_instruction(&self) -> Instruction;
}

impl IR {
    /// Creates an IR with empty module and returns it
    pub fn new() -> Self {
        Self {
            module: Instruction::Module(Vec::new())
        }
    }

    pub fn insert_instruction(&mut self, instruction: Instruction) {
        if let Instruction::Module(module) = &mut self.module {
            module.push(instruction);
        } else {
            panic!("Fatal Error: No module in root");
        }
    }
}