use crate::instructions::Instruction;

// The minimum number of memory cells is 30,000 as per the conventions on esolangs.org, that's good
// enough for this interpreter
const MEMORY_SIZE: usize = 30000;

pub struct VM {
    pointer: usize,
    // We use u8 to represent 8 bit memory cells, as per the conventions on esolangs.org
    memory: Vec<u8>,
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

impl VM {
    pub fn new() -> VM {
        VM {
            pointer: 0,
            memory: vec![0; MEMORY_SIZE],
        }
    }

    pub fn run_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::MoveLeft => {
                if self.pointer == 0 {
                    panic!("Pointer out of bounds");
                }
                self.pointer -= 1;
            }
            Instruction::MoveRight => {
                if self.pointer == MEMORY_SIZE - 1 {
                    panic!("Pointer out of bounds");
                }
                self.pointer += 1;
            }
            Instruction::IncrementPtr => {
                // Wrapping add as per memory conventions on esolangs.org
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(1);
            }
            Instruction::DecrementPtr => {
                // Wrapping add as per memory conventions on esolangs.org
                self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(1);
            }
            Instruction::Output => {
                print!("{}", self.memory[self.pointer] as char);
            }
            Instruction::Input => todo!(),
            Instruction::LoopStart => todo!(),
            Instruction::LoopEnd => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_new() {
        let vm = VM::new();
        assert_eq!(vm.pointer, 0);
        assert_eq!(vm.memory.len(), MEMORY_SIZE);
    }

    #[test]
    fn test_vm_increment() {
        let mut vm = VM::new();
        vm.run_instruction(&Instruction::IncrementPtr);
        assert_eq!(vm.memory[0], 1);
    }

    #[test]
    fn test_vm_decrement() {
        let mut vm = VM::new();
        vm.memory[0] = 3;
        vm.run_instruction(&Instruction::DecrementPtr);
        assert_eq!(vm.memory[0], 2);
    }

    #[test]
    fn test_vm_move_left() {
        let mut vm = VM::new();
        vm.pointer = 1;
        vm.run_instruction(&Instruction::MoveLeft);
        assert_eq!(vm.pointer, 0);
    }

    #[test]
    fn test_vm_move_right() {
        let mut vm = VM::new();
        vm.pointer = 0;
        vm.run_instruction(&Instruction::MoveRight);
        assert_eq!(vm.pointer, 1);
    }
}
