#[derive(Clone)]
pub struct IntCodePC {
    data: Vec<usize>,
    pc: usize, // Program Counter
}

impl IntCodePC {
    pub fn new(input: &str) -> Self {
        let data = input
            .trim()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        Self { data, pc: 0 }
    }

    pub fn run_program(&mut self) -> usize {
        let mut next_instruction = IntCodeInstruction::try_from(self.data[self.pc]).unwrap();

        while next_instruction != IntCodeInstruction::Halt {
            match next_instruction {
                IntCodeInstruction::Add => {
                    let operand_1 = self.get_operand(self.pc + 1);
                    let operand_2 = self.get_operand(self.pc + 2);
                    let destination = self.data[self.pc + 3];
                    self.data[destination] = operand_1 + operand_2;
                    self.pc += 4;
                }
                IntCodeInstruction::Mul => {
                    let operand_1 = self.get_operand(self.pc + 1);
                    let operand_2 = self.get_operand(self.pc + 2);
                    let destination = self.data[self.pc + 3];
                    self.data[destination] = operand_1 * operand_2;
                    self.pc += 4;
                }
                IntCodeInstruction::Halt => break,
            }

            next_instruction = IntCodeInstruction::try_from(self.data[self.pc]).unwrap();
        }

        self.data[0]
    }

    fn get_operand(&self, index: usize) -> usize {
        let operand_location = self.data[index];
        self.data[operand_location]
    }

    pub fn set(&mut self, index: usize, value: usize) {
        self.data[index] = value;
    }

    pub fn reset_pc(&mut self) {
        self.pc = 0;
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum IntCodeInstruction {
    Add = 1,
    Mul = 2,
    Halt = 99,
}

impl TryFrom<usize> for IntCodeInstruction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Add),
            2 => Ok(Self::Mul),
            99 => Ok(Self::Halt),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut pc = IntCodePC::new("1,4,2,0,99");
        assert_eq!(pc.run_program(), 101);
    }

    #[test]
    fn test_mul() {
        let mut pc = IntCodePC::new("2,4,2,0,99");
        assert_eq!(pc.run_program(), 198);
    }
}
