use std::collections::VecDeque;

#[derive(Clone)]
pub struct IntCodePC {
    data: Vec<i128>,
    pc: usize, // Program Counter
    input: VecDeque<i128>,
    output: VecDeque<i128>,
    initial_data: Vec<i128>,
    relative_base: i128,
}

impl IntCodePC {
    pub fn new(program_input: &str) -> Self {
        let mut data: Vec<i128> = program_input
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        data.resize(65536, 0);

        Self {
            data: data.clone(),
            pc: 0,
            input: VecDeque::default(),
            output: VecDeque::default(),
            initial_data: data,
            relative_base: 0,
        }
    }

    pub fn run_program(&mut self) -> IntCodeProgramState {
        let mut next_instruction = IntCodeInstruction::new(self.data[self.pc]);

        while next_instruction.instruction_type != IntCodeInstructionType::Halt {
            match next_instruction.instruction_type {
                IntCodeInstructionType::Add => {
                    let operand_1 = self.get_operand(&next_instruction, 1);
                    let operand_2 = self.get_operand(&next_instruction, 2);
                    let destination = self.get_destination(&next_instruction, 3);
                    self.data[destination] = operand_1 + operand_2;
                    self.pc += 4;
                }
                IntCodeInstructionType::Mul => {
                    let operand_1 = self.get_operand(&next_instruction, 1);
                    let operand_2 = self.get_operand(&next_instruction, 2);
                    let destination = self.get_destination(&next_instruction, 3);
                    self.data[destination] = operand_1 * operand_2;
                    self.pc += 4;
                }
                IntCodeInstructionType::Input => {
                    let Some(input_data) = self.input.pop_front() else {
                        return IntCodeProgramState::NeedsInput;
                    };

                    let destination = self.get_destination(&next_instruction, 1);
                    self.data[destination] = input_data;
                    self.pc += 2;
                }
                IntCodeInstructionType::Output => {
                    let output_value = self.get_operand(&next_instruction, 1);
                    self.output.push_back(output_value);
                    self.pc += 2;
                }
                IntCodeInstructionType::JumpIfTrue => {
                    if self.get_operand(&next_instruction, 1) != 0 {
                        self.pc = usize::try_from(self.get_operand(&next_instruction, 2)).unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                IntCodeInstructionType::JumpIfFalse => {
                    if self.get_operand(&next_instruction, 1) == 0 {
                        self.pc = usize::try_from(self.get_operand(&next_instruction, 2)).unwrap();
                    } else {
                        self.pc += 3;
                    }
                }
                IntCodeInstructionType::LessThan => {
                    let operand_1 = self.get_operand(&next_instruction, 1);
                    let operand_2 = self.get_operand(&next_instruction, 2);
                    let destination = self.get_destination(&next_instruction, 3);
                    if operand_1 < operand_2 {
                        self.data[destination] = 1;
                    } else {
                        self.data[destination] = 0;
                    }
                    self.pc += 4;
                }
                IntCodeInstructionType::Equals => {
                    let operand_1 = self.get_operand(&next_instruction, 1);
                    let operand_2 = self.get_operand(&next_instruction, 2);
                    let destination = self.get_destination(&next_instruction, 3);
                    if operand_1 == operand_2 {
                        self.data[destination] = 1;
                    } else {
                        self.data[destination] = 0;
                    }
                    self.pc += 4;
                }
                IntCodeInstructionType::AdjustRelativeBase => {
                    self.relative_base += self.get_operand(&next_instruction, 1);
                    self.pc += 2;
                }
                IntCodeInstructionType::Halt => unreachable!(),
            }

            next_instruction = IntCodeInstruction::new(self.data[self.pc]);
        }

        IntCodeProgramState::Halted
    }

    fn get_operand(&self, instruction: &IntCodeInstruction, index: usize) -> i128 {
        match instruction.mode_of(index) {
            ParameterMode::Position => {
                let operand_location = self.data[self.pc + index];
                self.data[usize::try_from(operand_location).unwrap()]
            }
            ParameterMode::Direct => self.data[self.pc + index],
            ParameterMode::Relative => {
                let operand_value = self.data[self.pc + index];
                let relative_location = operand_value + self.relative_base;
                self.data[usize::try_from(relative_location).unwrap()]
            }
        }
    }

    fn get_destination(&self, instruction: &IntCodeInstruction, index: usize) -> usize {
        match instruction.mode_of(index) {
            ParameterMode::Relative => {
                let operand_value = self.data[self.pc + index];
                operand_value + self.relative_base
            }
            _ => self.data[self.pc + index],
        }
        .try_into()
        .unwrap()
    }

    pub fn set(&mut self, index: usize, value: i128) {
        self.data[index] = value;
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0;
    }

    pub fn reset_all(&mut self) {
        self.data = self.initial_data.clone();
        self.pc = 0;
        self.output = VecDeque::new();
        self.input = VecDeque::new();
        self.relative_base = 0;
    }

    pub fn set_input(&mut self, new_input: VecDeque<i128>) {
        self.input = new_input;
    }

    pub fn get_output(&self) -> VecDeque<i128> {
        self.output.clone()
    }

    pub fn take_output(&mut self) -> VecDeque<i128> {
        std::mem::take(&mut self.output)
    }

    pub fn get_data(&self, index: usize) -> Option<i128> {
        self.data.get(index).copied()
    }

    pub fn run_with_input(
        &mut self,
        input: VecDeque<i128>,
    ) -> (IntCodeProgramState, VecDeque<i128>) {
        self.input = input;
        let state = self.run_program();
        let output = self.take_output();
        (state, output)
    }
}

pub struct IntCodeInstruction {
    value: i128,
    instruction_type: IntCodeInstructionType,
}

impl IntCodeInstruction {
    pub fn new(value: i128) -> Self {
        Self {
            value,
            instruction_type: IntCodeInstructionType::try_from(value).unwrap(),
        }
    }

    pub fn mode_of(&self, operand_number: usize) -> ParameterMode {
        let mode = (self.value / 10i128.pow((1 + operand_number).try_into().unwrap())) % 10;
        ParameterMode::try_from(mode).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum IntCodeInstructionType {
    Add = 1,
    Mul = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    AdjustRelativeBase = 9,
    Halt = 99,
}

impl TryFrom<i128> for IntCodeInstructionType {
    type Error = ();

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        match value % 100 {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            3 => Ok(Self::Input),
            4 => Ok(Self::Output),
            5 => Ok(Self::JumpIfTrue),
            6 => Ok(Self::JumpIfFalse),
            7 => Ok(Self::LessThan),
            8 => Ok(Self::Equals),
            9 => Ok(Self::AdjustRelativeBase),
            99 => Ok(Self::Halt),
            _ => Err(()),
        }
    }
}

pub enum ParameterMode {
    Position = 0,
    Direct = 1,
    Relative = 2,
}

impl TryFrom<i128> for ParameterMode {
    type Error = ();

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        match value % 100 {
            0 => Ok(Self::Position),
            1 => Ok(Self::Direct),
            2 => Ok(Self::Relative),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum IntCodeProgramState {
    Halted,
    NeedsInput,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut pc = IntCodePC::new("1,4,2,0,99");
        assert_eq!(pc.run_program(), IntCodeProgramState::Halted);
        assert_eq!(pc.get_data(0), Some(101));
    }

    #[test]
    fn test_mul() {
        let mut pc = IntCodePC::new("2,4,2,0,99");
        assert_eq!(pc.run_program(), IntCodeProgramState::Halted);
        assert_eq!(pc.get_data(0), Some(198));
    }

    #[test]
    fn test_input_output() {
        let mut pc = IntCodePC::new("3,0,4,0,99");
        pc.set_input(VecDeque::from([1162]));
        pc.run_program();
        let output = pc.get_output();
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], 1162)
    }

    #[test]
    fn test_jumps() {
        // Should output 0 if input was 0, and 1 otherwise
        //Position mode
        let mut pc = IntCodePC::new("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
        pc.set_input(VecDeque::from([1162]));
        pc.run_program();
        let output = pc.get_output();
        assert_eq!(output[0], 1);

        pc.reset_all();
        pc.set_input(VecDeque::from([0]));
        pc.run_program();
        let output = pc.get_output();
        assert_eq!(output[0], 0);

        //Immediate mode
        let mut pc = IntCodePC::new("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
        pc.set_input(VecDeque::from([1162]));
        pc.run_program();
        let output = pc.get_output();
        assert_eq!(output[0], 1);

        pc.reset_all();
        pc.set_input(VecDeque::from([0]));
        pc.run_program();
        let output = pc.get_output();
        assert_eq!(output[0], 0);
    }

    #[test]
    fn test_relative() {
        // Output is a copy of the program
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut pc = IntCodePC::new(input);
        let data: Vec<i128> = input
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        pc.run_program();
        let output: Vec<i128> = pc.get_output().into();
        assert_eq!(data, output);

        // Output is its input
        let input = "109,1,203,11,209,8,204,1,99,10,0,42,0";
        let mut pc = IntCodePC::new(input);
        pc.set_input([2024].into());
        pc.run_program();
        let output = pc.get_output();
        println!("{output:?}");
        assert_eq!(output[0], 2024);
    }
}
