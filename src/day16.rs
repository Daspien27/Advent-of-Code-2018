use regex::{Regex};
use std::fmt;
use std::collections::HashSet;


#[derive(Debug, Clone, PartialEq)]
struct Register (i64, i64, i64, i64);

trait IntoRegister {
    fn into_register(&self) -> Register;
}


impl IntoRegister for &[i64] {
    fn into_register(&self) -> Register {
        Register(self[0], self[1], self[2], self[3])
    }
}

impl fmt::Display for Register {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.0, self.1, self.2, self.3)
    }

}

impl From<&str> for Register {
    fn from(input: &str) -> Register {
        input
            .split(", ")
            .map(|s|{
                s.parse().unwrap()
            })
            .collect::<Vec<i64>>()
            .as_slice()
            .into_register()
    }
}

impl Register {
    fn reference(&self, idx : i64) -> &i64 {
        match idx {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Reference is out of register range.")
        }
    }

    fn reference_mut(&mut self, idx : i64) -> &mut i64 {
        match idx {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Reference is out of register range.")
        }
    }
}


#[derive(Debug, Clone)]
pub struct InstructionSet(i64, i64, i64, i64);
trait IntoInstruction {
    fn into_instruction(&self) -> InstructionSet;
}

impl IntoInstruction for &[i64] {
    fn into_instruction(&self) -> InstructionSet {
        InstructionSet(self[0], self[1], self[2], self[3])
    }
}

impl fmt::Display for InstructionSet {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.0, self.1, self.2, self.3)
    }

}

impl From<&str> for InstructionSet {
    fn from(input: &str) -> InstructionSet {
        input
            .split(" ")
            .map(|s|{
                s.parse().unwrap()
            })
            .collect::<Vec<i64>>()
            .as_slice()
            .into_instruction()
    }
}


pub struct TestCase
{
    before_register : Register,
    instruction : InstructionSet,
    after_register : Register,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Opcodes {
    addr,
    addi,
    mulr,
    muli,
    banr,
    bani,
    borr,
    bori,
    setr,
    seti,
    gtir,
    gtri,
    gtrr,
    eqir,
    eqri,
    eqrr,    
}

fn addr(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = register.reference(instruction.1) + register.reference(instruction.2);
}
fn addi(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = register.reference(instruction.1) + instruction.2;
}
fn mulr(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = register.reference(instruction.1) * register.reference(instruction.2);
}
fn muli(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = register.reference(instruction.1) * instruction.2;
}
fn banr(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = register.reference(instruction.1) & register.reference(instruction.2);
}
fn bani(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = register.reference(instruction.1) & instruction.2;
}
fn borr(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = register.reference(instruction.1) | register.reference(instruction.2);
}
fn bori(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = register.reference(instruction.1) | instruction.2;
}
fn setr(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = *register.reference(instruction.1);
}
fn seti(instruction: &InstructionSet, register : &mut Register) {
    *register.reference_mut(instruction.3) = instruction.1;
}
fn gtir(instruction: &InstructionSet, register : &mut Register) {
    if instruction.1 > *register.reference(instruction.2)
    {
        *register.reference_mut(instruction.3) = 1;
    }
    else
    {
        *register.reference_mut(instruction.3) = 0;
    }
}
fn gtri(instruction: &InstructionSet, register : &mut Register) {
    if *register.reference(instruction.1) > instruction.2
    {
        *register.reference_mut(instruction.3) = 1;
    }
    else
    {
        *register.reference_mut(instruction.3) = 0;
    }
}
fn gtrr(instruction: &InstructionSet, register : &mut Register) {
    if *register.reference(instruction.1) > *register.reference(instruction.2)
    {
        *register.reference_mut(instruction.3) = 1;
    }
    else
    {
        *register.reference_mut(instruction.3) = 0;
    }
}
fn eqir(instruction: &InstructionSet, register : &mut Register) {
    if instruction.1 == *register.reference(instruction.2)
    {
        *register.reference_mut(instruction.3) = 1;
    }
    else
    {
        *register.reference_mut(instruction.3) = 0;
    }
}
fn eqri(instruction: &InstructionSet, register : &mut Register) {
    if *register.reference(instruction.1) == instruction.2
    {
        *register.reference_mut(instruction.3) = 1;
    }
    else
    {
        *register.reference_mut(instruction.3) = 0;
    }
}
fn eqrr(instruction: &InstructionSet, register : &mut Register) {
    if *register.reference(instruction.1) == *register.reference(instruction.2)
    {
        *register.reference_mut(instruction.3) = 1;
    }
    else
    {
        *register.reference_mut(instruction.3) = 0;
    }
}

impl Opcodes {
    fn apply(&self, instruction: &InstructionSet, register : &mut Register) {
        match self {
            Opcodes::addr => addr(&instruction, register),
            Opcodes::addi => addi(&instruction, register),
            Opcodes::mulr => mulr(&instruction, register),
            Opcodes::muli => muli(&instruction, register),
            Opcodes::banr => banr(&instruction, register),
            Opcodes::bani => bani(&instruction, register),
            Opcodes::borr => borr(&instruction, register),
            Opcodes::bori => bori(&instruction, register),
            Opcodes::setr => setr(&instruction, register),
            Opcodes::seti => seti(&instruction, register),
            Opcodes::gtir => gtir(&instruction, register),
            Opcodes::gtri => gtri(&instruction, register),
            Opcodes::gtrr => gtrr(&instruction, register),
            Opcodes::eqir => eqir(&instruction, register),
            Opcodes::eqri => eqri(&instruction, register),
            Opcodes::eqrr => eqrr(&instruction, register),   
        }
    }
}

const POSSIBLE_INSTRUCTIONS : [Opcodes; 16] = [
    Opcodes::addr,
    Opcodes::addi,
    Opcodes::mulr,
    Opcodes::muli,
    Opcodes::banr,
    Opcodes::bani,
    Opcodes::borr,
    Opcodes::bori,
    Opcodes::setr,
    Opcodes::seti,
    Opcodes::gtir,
    Opcodes::gtri,
    Opcodes::gtrr,
    Opcodes::eqir,
    Opcodes::eqri,
    Opcodes::eqrr
];

impl fmt::Display for TestCase {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Before: {}\n{}\nAfter:  {} ", self.before_register, self.instruction, self.after_register)
    }

}

#[aoc_generator(day16, part1)]
pub fn input_generator_part1(input: &str) -> Vec<TestCase>
{
    lazy_static! {
        static ref TESTCASE_REGEX : Regex = Regex::new(
r"(?m)Before: \[(?P<before>\d+, \d+, \d+, \d+)\]
(?P<instruction>\d+ \d+ \d+ \d+)
After:  \[(?P<after>\d+, \d+, \d+, \d+)\]").unwrap();
    }

    TESTCASE_REGEX.captures_iter(input)
                  .map(|cap|{
                        TestCase { before_register: Register::from(cap.name("before").unwrap().as_str()), instruction: InstructionSet::from(cap.name("instruction").unwrap().as_str()), after_register: Register::from(cap.name("after").unwrap().as_str())}
                   })
                   .collect()
}

#[aoc_generator(day16, part2)]
pub fn input_generator_part2(input: &str) -> (Vec<TestCase>, Vec<InstructionSet>)
{
    lazy_static! {
        static ref TESTCASE_REGEX : Regex = Regex::new(
r"(?m)Before: \[(?P<before>\d+, \d+, \d+, \d+)\]
(?P<instruction>\d+ \d+ \d+ \d+)
After:  \[(?P<after>\d+, \d+, \d+, \d+)\]").unwrap();

        static ref INSTRUCTION_REGEX : Regex = Regex::new(r"(?P<instruction>\d+ \d+ \d+ \d+)").unwrap();
    }

    let split_input : Vec<_> = input.split("\n\n\n").collect();
    assert_eq!(split_input.len(), 2);
    
    let test_cases = TESTCASE_REGEX.captures_iter(split_input[0])
                  .map(|cap|{
                        TestCase { before_register: Register::from(cap.name("before").unwrap().as_str()), instruction: InstructionSet::from(cap.name("instruction").unwrap().as_str()), after_register: Register::from(cap.name("after").unwrap().as_str())}
                   })
                   .collect();

    let instructions = INSTRUCTION_REGEX.captures_iter(split_input[1])
                        .map(|cap|{
                             InstructionSet::from(cap.name("instruction").unwrap().as_str())
                        })
                        .collect();


    (test_cases, instructions)
}


#[aoc(day16, part1)]
pub fn solve_part1(input: &Vec<TestCase>) -> u64 {

    input
        .iter()
        .map(|test_case|{
            let possible_opcodes : Vec<Opcodes> = POSSIBLE_INSTRUCTIONS
                .iter()
                .filter(|opcode|{
                    let mut register_test = test_case.before_register.clone();
                    opcode.apply(&test_case.instruction, &mut register_test);
                    test_case.after_register == register_test
                })
                .map(|&op| op)
                .collect();

            (test_case.instruction.0, possible_opcodes)
    })
    .fold(0u64, |acc, (_instruction_code, possible_opcodes)|{
        if possible_opcodes.len() >= 3 {
            acc + 1
        }
        else {
            acc
        }
    })
}

#[aoc(day16, part2)]
pub fn solve_part2((test_cases, program): &(Vec<TestCase>, Vec<InstructionSet>)) -> i64 {
    
    let mut instructions_opcode_map = vec![POSSIBLE_INSTRUCTIONS.iter().cloned().collect::<HashSet<Opcodes>>(); 16];

    test_cases
        .iter()
        .for_each(|test_case|{
            let possible_opcodes : HashSet<Opcodes> = POSSIBLE_INSTRUCTIONS
                .iter()
                .filter(|opcode|{
                    let mut register_test = test_case.before_register.clone();
                    opcode.apply(&test_case.instruction, &mut register_test);
                    test_case.after_register == register_test
                })
                .map(|&op| op)
                .collect();

            let intersection : HashSet<Opcodes> = instructions_opcode_map[test_case.instruction.0 as usize].intersection(&possible_opcodes).copied().collect();
            
            instructions_opcode_map[test_case.instruction.0 as usize] = intersection;
    });

    let mut instructions_opcode : Vec<Option<Opcodes>> = vec![None; 16];
    
    while instructions_opcode.iter().any(|x| x.is_none()) {
        let single_instructions : Vec<_>= instructions_opcode_map
                                        .iter()
                                        .enumerate()
                                        .filter(|(_, x)| x.len() == 1 )
                                        .map(|(i, x)|{
                                            (i, x.iter().take(1).next().unwrap().clone())
                                        })
                                        .collect();
        
        single_instructions
            .iter()
            .for_each(|(idx, instruction)|{
                instructions_opcode[*idx].get_or_insert(*instruction);

                instructions_opcode_map
                        .iter_mut()
                        .for_each(|x|{
                            x.remove(instruction);
                        });
            });                                            
    }

    let mut program_register = Register(0, 0, 0, 0);
    program
        .iter()
        .for_each(|instruction|{
            instructions_opcode[instruction.0 as usize].unwrap().apply(instruction, &mut program_register);
        });

    program_register.0
}

#[cfg(test)]
mod tests {

}