use crate::lexer::{Token, Commands};
use crate::error::ErrorCtx;
use std::fmt::{Debug, Formatter};
use dyn_clone::{DynClone, clone_trait_object};
use std::any::Any;
use bfvm::ir::{ToIRInstruction, Instruction};

pub trait AST: DynClone + ToIRInstruction {
    fn debug_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result;

    fn as_any(&self) -> &dyn Any;

    fn equals(&self, other: &dyn AST) -> bool;
}

impl PartialEq for dyn AST {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.equals(other)
    }
}

impl Debug for dyn AST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_fmt(f)
    }
}

clone_trait_object!(AST);

/// Root node for the AST
#[derive(Debug, PartialEq, Clone)]
pub struct Module {
    pub body: Vec<Box<dyn AST + 'static>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandAST {
    pub command_type: Commands
}

impl AST for CommandAST {
    fn debug_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn AST) -> bool {
        other
            .as_any()
            .downcast_ref()
            .map_or(false, |o| self == o)
    }
}

impl ToIRInstruction for CommandAST {
    fn to_ir_instruction(&self) -> Instruction {
        match self.command_type {
            Commands::IncDp => Instruction::DpInc,
            Commands::DecDp => Instruction::DpDec,
            Commands::IncVal => Instruction::ValInc,
            Commands::DecVal => Instruction::ValDec,
            Commands::Output => Instruction::Output,
            Commands::Input => Instruction::Input,
            _ => panic!("Fatal internal error occurred") // should never *ever* happen
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopAST {
    pub body: Vec<Box<dyn AST>>
}

impl AST for LoopAST {
    fn debug_fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt(f)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn equals(&self, other: &dyn AST) -> bool {
        other
            .as_any()
            .downcast_ref()
            .map_or(false, |o| self == o)
    }
}

impl ToIRInstruction for LoopAST {
    fn to_ir_instruction(&self) -> Instruction {
        let mut loop_body: Vec<Instruction> = Vec::new();

        for command in &self.body {
            loop_body.push(command.to_ir_instruction())
        }

        Instruction::LoopBody(loop_body)
    }
}

pub struct Parser {
    lexical_tokens: Vec<Token>,
    current_pos: usize,
    error_ctx: ErrorCtx,
    module: Module
}

impl Parser {
    pub fn new(lexical_tokens: &Vec<Token>, error_ctx: ErrorCtx) -> Self {
        Self {
            lexical_tokens: lexical_tokens.to_vec(),
            current_pos: 0,
            error_ctx,
            module: Module {
                body: Vec::new()
            }
        }
    }

    pub fn parse(&mut self) {
        while self.current_pos < self.lexical_tokens.len() {
            let token = self.lexical_tokens[self.current_pos].clone();

            if token.command == Commands::LoopEnter {
                // eat LoopEnter
                self.advance(true);
                let loop_ast = self.parse_loop();
                self.module.body.push(Box::new(loop_ast))
            }
            else {
                self.module.body.push(Box::new(CommandAST {
                    command_type: token.command
                }))
            }

            self.advance(false);
        }
    }

    /// Recursively make loops
    fn parse_loop(&mut self) -> LoopAST {
        let mut loop_ast = LoopAST {
            body: Vec::new()
        };

        while self.current_pos < self.lexical_tokens.len() {
            let token = self.lexical_tokens[self.current_pos].clone();
            if token.command == Commands::LoopEnter {
                // Eat LoopEnter
                self.advance(true);
                loop_ast.body.push(Box::new(self.parse_loop()))
            }
            else if token.command == Commands::LoopEnd {
                // Eat LoopEnd
                return loop_ast
            }
            else {
                loop_ast.body.push(Box::new(CommandAST {
                    command_type: token.command
                }))
            }
            self.advance(false);
        }

        self.error_ctx.raise("Expected a ']'", self.current_pos);
        std::process::exit(1);
    }

    fn advance(&mut self, err_on_eof: bool) {
        self.current_pos += 1;

        if self.current_pos > self.lexical_tokens.len() && err_on_eof {
            self.error_ctx.raise("Unexpected EOF", self.current_pos - 1);
        }
    }

    pub fn get_module(&self) -> Module {
        self.module.clone()
    }
}

#[cfg(test)]
mod test_parser {
    use super::*;
    use crate::lexer::lex;

    #[test]
    fn parser_test_1() {
        let lex_tokens = lex("><+-.,[]");
        let errctx = ErrorCtx::new("><+-.,[]");
        let mut parser = Parser::new(&lex_tokens, errctx);
        parser.parse();

        let result = parser.get_module();
        let expected = Module {
            body: vec![
                Box::new(CommandAST {
                    command_type: Commands::IncDp,
                }),
                Box::new(CommandAST {
                    command_type: Commands::DecDp,
                }),
                Box::new(CommandAST {
                    command_type: Commands::IncVal,
                }),
                Box::new(CommandAST {
                    command_type: Commands::DecVal,
                }),
                Box::new(CommandAST {
                    command_type: Commands::Output,
                }),
                Box::new(CommandAST {
                    command_type: Commands::Input,
                }),
                Box::new(LoopAST {
                    body: vec![],
                }),
            ],
        };
        assert_eq!(result, expected);
    }
}