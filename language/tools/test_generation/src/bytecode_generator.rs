// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{abstract_state::AbstractState, common, summaries};
use rand::{rngs::StdRng, FromEntropy, Rng, SeedableRng};
use vm::file_format::{
    AddressPoolIndex, ByteArrayPoolIndex, Bytecode, FunctionSignature, SignatureToken,
    StringPoolIndex,
};

/// This type represents bytecode instructions that take a `u8`
type U8ToBytecode = fn(u8) -> Bytecode;

/// This type represents bytecode instructions that take a `u64`
type U64ToBytecode = fn(u64) -> Bytecode;

/// This type represents bytecode instructions that take a `StringPoolIndex`
type StringPoolIndexToBytecode = fn(StringPoolIndex) -> Bytecode;

/// This type represents bytecode instructions that take a `AddressPoolIndex`
type AddressPoolIndexToBytecode = fn(AddressPoolIndex) -> Bytecode;

/// This type represents bytecode instructions that take a `ByteArrayPoolIndex`
type ByteArrayPoolIndexToBytecode = fn(ByteArrayPoolIndex) -> Bytecode;

/// There are six types of bytecode instructions
#[derive(Debug, Clone)]
enum BytecodeType {
    /// Instructions that do not take an argument
    NoArg(Bytecode),

    /// Instructions that take a `u8`
    U8(U8ToBytecode),

    /// Instructions that take a `u64`
    U64(U64ToBytecode),

    /// Instructions that take a `StringPoolIndex`
    StringPoolIndex(StringPoolIndexToBytecode),

    /// Instructions that take an `AddressPoolIndex`
    AddressPoolIndex(AddressPoolIndexToBytecode),

    /// Instructions that take a `ByteArrayPoolIndex`
    ByteArrayPoolIndex(ByteArrayPoolIndexToBytecode),
}

/// Abstraction for change to the stack size
#[derive(Debug, Copy, Clone, PartialEq)]
enum StackEffect {
    /// Represents an increase in stack size
    Add,

    /// Represents a decrease in stack size
    Sub,

    /// Represents no change in stack size
    Nop,
}

/// Generates a sequence of bytecode instructions.
/// This generator has:
/// - `instructions`: A list of bytecode instructions to use for generation
/// - `rng`: A random number generator for uniform random choice of next instruction
#[derive(Debug, Clone)]
pub struct BytecodeGenerator {
    instructions: Vec<(StackEffect, BytecodeType)>,
    rng: StdRng,
}

impl BytecodeGenerator {
    /// The `BytecodeGenerator` is instantiated with a seed to use with
    /// its random number generator.
    pub fn new(seed: Option<[u8; 32]>) -> Self {
        let instructions: Vec<(StackEffect, BytecodeType)> = vec![
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Pop)),
            (StackEffect::Add, BytecodeType::U64(Bytecode::LdConst)),
            (
                StackEffect::Add,
                BytecodeType::StringPoolIndex(Bytecode::LdStr),
            ),
            (
                StackEffect::Add,
                BytecodeType::AddressPoolIndex(Bytecode::LdAddr),
            ),
            (StackEffect::Add, BytecodeType::NoArg(Bytecode::LdTrue)),
            (StackEffect::Add, BytecodeType::NoArg(Bytecode::LdFalse)),
            (
                StackEffect::Add,
                BytecodeType::ByteArrayPoolIndex(Bytecode::LdByteArray),
            ),
            (StackEffect::Add, BytecodeType::U8(Bytecode::CopyLoc)),
            (StackEffect::Add, BytecodeType::U8(Bytecode::MoveLoc)),
            (StackEffect::Sub, BytecodeType::U8(Bytecode::StLoc)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Add)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Sub)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Mul)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Div)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Mod)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::BitAnd)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::BitOr)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Xor)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Or)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::And)),
            (StackEffect::Nop, BytecodeType::NoArg(Bytecode::Not)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Eq)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Neq)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Lt)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Gt)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Le)),
            (StackEffect::Sub, BytecodeType::NoArg(Bytecode::Ge)),
            (
                StackEffect::Add,
                BytecodeType::NoArg(Bytecode::GetTxnGasUnitPrice),
            ),
            (
                StackEffect::Add,
                BytecodeType::NoArg(Bytecode::GetTxnMaxGasUnits),
            ),
            (
                StackEffect::Add,
                BytecodeType::NoArg(Bytecode::GetGasRemaining),
            ),
            (
                StackEffect::Add,
                BytecodeType::NoArg(Bytecode::GetTxnSequenceNumber),
            ),
        ];
        let generator = match seed {
            Some(seed) => StdRng::from_seed(seed),
            None => StdRng::from_entropy(),
        };
        Self {
            instructions,
            rng: generator,
        }
    }

    /// Given an `AbstractState`, `state`, and a the number of locals the function has,
    /// this function returns a list of instructions whose preconditions are satisfied for
    /// the state.
    fn candidate_instructions(
        &mut self,
        state: AbstractState,
        locals_len: usize,
    ) -> Vec<(StackEffect, Bytecode)> {
        let mut matches: Vec<(StackEffect, Bytecode)> = Vec::new();
        let instructions = &self.instructions;
        for (stack_effect, instruction) in instructions.iter() {
            let instruction: Bytecode = match instruction {
                BytecodeType::NoArg(instruction) => instruction.clone(),
                BytecodeType::U8(instruction) => {
                    // Generate a random index into the locals
                    let local_index: u8 = self.rng.gen_range(0, locals_len as u8);
                    instruction(local_index)
                }
                BytecodeType::U64(instruction) => {
                    // Generate a random u64 constant to load
                    let value = self.rng.gen_range(0, u64::max_value());
                    instruction(value)
                }
                BytecodeType::StringPoolIndex(instruction) => {
                    // TODO: Determine correct index
                    instruction(StringPoolIndex::new(0))
                }
                BytecodeType::AddressPoolIndex(instruction) => {
                    // TODO: Determine correct index
                    instruction(AddressPoolIndex::new(0))
                }
                BytecodeType::ByteArrayPoolIndex(instruction) => {
                    // TODO: Determine correct index
                    instruction(ByteArrayPoolIndex::new(0))
                }
            };
            let summary = summaries::instruction_summary(instruction.clone());
            let unsatisfied_preconditions = summary
                .preconditions
                .iter()
                .any(|precondition| !precondition(&state));
            if !unsatisfied_preconditions {
                matches.push((*stack_effect, instruction));
            }
        }
        matches
    }

    /// Select an instruction from the list of candidates based on the current state's
    /// stack size and the expected number of function return parameters.
    fn select_candidate(
        &mut self,
        return_len: usize,
        state: &AbstractState,
        candidates: &[(StackEffect, Bytecode)],
    ) -> Bytecode {
        let stack_len = state.stack_len();
        let prob_add = if stack_len > return_len {
            common::MUTATION_TOLERANCE / (stack_len as f32)
        } else {
            1.0
        };
        debug!("Pr[add] = {:?}", prob_add);
        let next_instruction_index;
        if self.rng.gen_range(0.0, 1.0) <= prob_add {
            let add_candidates: Vec<(StackEffect, Bytecode)> = candidates
                .iter()
                .filter(|(stack_effect, _)| {
                    *stack_effect == StackEffect::Add || *stack_effect == StackEffect::Nop
                })
                .cloned()
                .collect();
            debug!("Add candidates: [{:?}]", add_candidates);
            // Add candidates should not be empty unless the list of bytecode instructions is
            // changed
            if add_candidates.is_empty() {
                panic!("Could not find valid candidate");
            }
            next_instruction_index = self.rng.gen_range(0, add_candidates.len());
            add_candidates[next_instruction_index].1.clone()
        } else {
            let sub_candidates: Vec<(StackEffect, Bytecode)> = candidates
                .iter()
                .filter(|(stack_effect, _)| {
                    *stack_effect == StackEffect::Sub || *stack_effect == StackEffect::Nop
                })
                .cloned()
                .collect();
            debug!("Sub candidates: [{:?}]", sub_candidates);
            // Sub candidates should not be empty unless the list of bytecode instructions is
            // changed
            if sub_candidates.is_empty() {
                panic!("Could not find valid candidate");
            }
            next_instruction_index = self.rng.gen_range(0, sub_candidates.len());
            sub_candidates[next_instruction_index].1.clone()
        }
    }

    /// Transition an abstract state, `state` to the next state by applying all of the effects
    /// of a particular bytecode instruction, `instruction`.
    fn abstract_step(&self, state: AbstractState, instruction: Bytecode) -> AbstractState {
        summaries::instruction_summary(instruction)
            .effects
            .iter()
            .fold(state, |acc, effect| effect(&acc))
    }

    /// Return a sequence of bytecode instructions given a set of `locals` and a target return
    /// `signature`. The sequence should contain at least `target_min` and at most `target_max`
    /// instructions.
    pub fn generate(
        &mut self,
        locals: &[SignatureToken],
        signature: &FunctionSignature,
        target_min: usize,
        target_max: usize,
    ) -> Vec<Bytecode> {
        let mut bytecode: Vec<Bytecode> = Vec::new();
        let mut state: AbstractState = AbstractState::new(&Vec::new());
        loop {
            debug!("Bytecode: [{:?}]", bytecode);
            debug!("AbstractState: [{:?}]", state);
            let candidates = self.candidate_instructions(state.clone(), locals.len());
            debug!("Candidates: [{:?}]", candidates);
            if candidates.is_empty() {
                warn!("No candidates found for state: [{:?}]", state);
                break;
            }
            let next_instruction = self.select_candidate(0, &state, &candidates);
            debug!("Next instr: {:?}", next_instruction);
            state = self.abstract_step(state, next_instruction.clone());
            debug!("New state: {:?}", state);
            bytecode.push(next_instruction);
            debug!("**********************");
            if bytecode.len() >= target_min && state.is_final() || bytecode.len() >= target_max {
                info!("Instructions generated: {}", bytecode.len());
                break;
            }
        }
        for return_type in signature.return_types.iter() {
            match return_type {
                SignatureToken::String => bytecode.push(Bytecode::LdStr(StringPoolIndex::new(0))),
                SignatureToken::Address => {
                    bytecode.push(Bytecode::LdAddr(AddressPoolIndex::new(0)))
                }
                SignatureToken::U64 => bytecode.push(Bytecode::LdConst(0)),
                SignatureToken::Bool => bytecode.push(Bytecode::LdFalse),
                SignatureToken::ByteArray => {
                    bytecode.push(Bytecode::LdByteArray(ByteArrayPoolIndex::new(0)))
                }
                _ => panic!("Unsupported return type: {:#?}", return_type),
            }
        }
        bytecode.push(Bytecode::Ret);
        bytecode
    }
}
