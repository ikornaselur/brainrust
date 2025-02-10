use crate::instructions::Instruction;

// The minimum number of memory cells is 30,000 as per the conventions on esolangs.org, that's good
// enough for this interpreter
const MEMORY_SIZE: usize = 30_000;

pub struct VM {
    memory_pointer: usize,
    instr_pointer: usize,
    // We use u8 to represent 8 bit memory cells, as per the conventions on esolangs.org
    memory: Vec<u8>,
    loop_stack: Vec<usize>,
    program: Vec<Instruction>,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> VM {
        VM {
            memory_pointer: 0,
            instr_pointer: 0,
            memory: vec![0; MEMORY_SIZE],
            loop_stack: Vec::new(),
            program,
        }
    }

    pub fn run(&mut self) {
        let program_len = self.program.len();
        while self.instr_pointer < program_len {
            let instr = self.program[self.instr_pointer];
            self.run_instruction(&instr);
            self.instr_pointer += 1;
        }
    }

    fn run_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::MoveLeft => {
                if self.memory_pointer == 0 {
                    panic!("Pointer out of bounds");
                }
                self.memory_pointer -= 1;
            }
            Instruction::MoveRight => {
                if self.memory_pointer == MEMORY_SIZE - 1 {
                    panic!("Pointer out of bounds");
                }
                self.memory_pointer += 1;
            }
            Instruction::IncrementPtr => {
                // Wrapping add as per memory conventions on esolangs.org
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_add(1);
            }
            Instruction::DecrementPtr => {
                // Wrapping add as per memory conventions on esolangs.org
                self.memory[self.memory_pointer] = self.memory[self.memory_pointer].wrapping_sub(1);
            }
            Instruction::Output => {
                print!("{}", self.memory[self.memory_pointer] as char);
            }
            Instruction::Input => todo!(),
            Instruction::LoopStart => {
                let loop_level = self.loop_stack.len();
                self.loop_stack.push(self.instr_pointer);

                // If the memory cell is 0, we jump past to the matching loop end, by just going
                // through the instructions until the loop_stack becomes empty
                if self.memory[self.memory_pointer] == 0 {
                    // TODO: This could be optimised by storing references between loop start/end
                    while !self.loop_stack.is_empty() {
                        self.instr_pointer += 1;
                        // TODO: Handle out of bounds, in case of a faulty program
                        match &self.program[self.instr_pointer] {
                            Instruction::LoopStart => self.loop_stack.push(self.instr_pointer),
                            Instruction::LoopEnd => {
                                self.loop_stack.pop();
                                if self.loop_stack.len() == loop_level {
                                    // We're at the loop end for this level
                                    return;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Otherwise.. we just continue into the loop
            }
            Instruction::LoopEnd => {
                let loop_start = self.loop_stack.last().unwrap();

                if self.memory[self.memory_pointer] != 0 {
                    // If the memory cell is not 0, we jump back to the matching loop start
                    self.instr_pointer = *loop_start;
                } else {
                    // Otherwise, we just continue past the loop end
                    self.loop_stack.pop().unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_new() {
        let vm = VM::new(vec![]);
        assert_eq!(vm.memory_pointer, 0);
        assert_eq!(vm.instr_pointer, 0);
        assert_eq!(vm.memory.len(), MEMORY_SIZE);
    }

    #[test]
    fn test_vm_increment() {
        let mut vm = VM::new(vec![]);
        vm.run_instruction(&Instruction::IncrementPtr);
        assert_eq!(vm.memory[0], 1);
    }

    #[test]
    fn test_vm_decrement() {
        let mut vm = VM::new(vec![]);
        vm.memory[0] = 3;
        vm.run_instruction(&Instruction::DecrementPtr);
        assert_eq!(vm.memory[0], 2);
    }

    #[test]
    fn test_vm_move_left() {
        let mut vm = VM::new(vec![]);
        vm.memory_pointer = 1;
        vm.run_instruction(&Instruction::MoveLeft);
        assert_eq!(vm.memory_pointer, 0);
    }

    #[test]
    fn test_vm_move_right() {
        let mut vm = VM::new(vec![]);
        vm.memory_pointer = 0;
        vm.run_instruction(&Instruction::MoveRight);
        assert_eq!(vm.memory_pointer, 1);
    }

    #[test]
    fn test_vm_loop_start_simple_one_level_jump_past() {
        // Simple program that should just jump past the increment pointers
        let mut vm = VM::new(vec![
            Instruction::LoopStart,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::LoopEnd,
        ]);

        vm.run_instruction(&Instruction::LoopStart);

        assert_eq!(vm.instr_pointer, 4); // End of loop
        assert_eq!(vm.memory[0], 0);
    }

    #[test]
    fn test_vm_loop_start_inner_loop_jump_past() {
        // The state in this program will be to jump past the inner loop
        let mut vm = VM::new(vec![
            Instruction::LoopStart,
            Instruction::LoopStart,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::LoopEnd,
            Instruction::IncrementPtr,
            Instruction::LoopEnd,
        ]);
        vm.instr_pointer = 1; // Inner loop
        vm.loop_stack = vec![0];

        vm.run_instruction(&Instruction::LoopStart);

        assert_eq!(vm.instr_pointer, 5); // End of inner loop
    }

    #[test]
    fn test_vm_loop_end_simple_one_level_jump_back() {
        // Simple program that should just jump past the increment pointers
        let mut vm = VM::new(vec![
            Instruction::LoopStart,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::LoopEnd,
        ]);

        vm.loop_stack = vec![0];
        vm.instr_pointer = 4;
        vm.memory[0] = 1; // Should jump back on non-zero

        vm.run_instruction(&Instruction::LoopEnd);

        assert_eq!(vm.instr_pointer, 0);
    }

    #[test]
    fn test_vm_loop_end_inner_loop_jump_back() {
        // The state in this program will be to jump past the inner loop
        let mut vm = VM::new(vec![
            Instruction::LoopStart,
            Instruction::LoopStart,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::LoopEnd,
            Instruction::IncrementPtr,
            Instruction::LoopEnd,
        ]);
        vm.instr_pointer = 5; // Inner loop
        vm.loop_stack = vec![0, 1];
        vm.memory[0] = 1; // Should jump back on non-zero

        vm.run_instruction(&Instruction::LoopEnd);

        assert_eq!(vm.instr_pointer, 1); // Start of inner loop
    }

    #[test]
    fn test_vm_loop_end_continues_past_if_zero() {
        let mut vm = VM::new(vec![
            Instruction::LoopStart,
            Instruction::LoopStart,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::IncrementPtr,
            Instruction::LoopEnd,
            Instruction::IncrementPtr,
            Instruction::LoopEnd,
        ]);
        vm.instr_pointer = 5; // Inner loop
        vm.loop_stack = vec![0, 1];

        vm.run_instruction(&Instruction::LoopEnd);

        assert_eq!(vm.instr_pointer, 5); // End of inner loop
        assert_eq!(vm.loop_stack, vec![0]);
    }
}
