use std::env;
use std::fs;
use std::convert::TryInto;

#[derive(PartialEq, Eq)]
enum InstructionType {
    PUSH,       // Push value onto stack.
    POP,        // Pop value off top of stack.
    ADD,        // Pop two off stack then add them and put onto stack.
    SUB,        // Pop two off stack then substract them and put onto stack.
    MUL,        // Pop two off stack then multiply them and put onto stack.
    DIV,        // Pop two off stack then divide them and put onto stack.
    PRINT,      // Print value on top of stack.
    PRINTLINE,  // Print line of value on top of stack.
    PRINTASCII, // Print ASCII character on top of stack.
    INPUT,      // Read user input and push value on stack.
    LABEL,      // Label for jumps to change the instruction pointer to.
    J,          // Jump to label
    JE,         // Jump to label if equal
    JN,         // Jump to label if not equal
    JL,         // Jump to label if less than
    JG,         // Jump to label if greater than
    JLE,        // Jump to label if less than or equal
    JGE         // Jump to label if greter than or equal
}

struct StackMachine {
    instructions: Vec<Box<Instruction>>,
    stack: Vec<i32>
}

struct Instruction
{
    _type: InstructionType,
    value: i32,
    label: String
}

impl StackMachine {
    fn get_pointer(&self, label: &String) -> usize {
        let mut p : usize = 0;
        for i in &self.instructions {
            if i.label == *label && i._type == InstructionType::LABEL
            {
                return p;
            }
            p += 1;
        }
        panic!("Label '{}' is not found", *label);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sm: StackMachine = StackMachine { instructions: Vec::new(), stack: Vec::new() };
    let file_path: &String = &args[1];
    let underflow_error: String = String::from("Stack underflow!");
    println!("Welcome to Huw's Stack Machine");
    let content = fs::read_to_string(file_path).expect("Cannot read file");
    let lines = content.split("\n");
    for line in lines {
        if line == "" {
            continue;
        }
        let sections: Vec<&str> = line.split(" ").collect();
        let instruction = sections.get(0);
        match instruction
        {
            Some(&"push") => {
                let value_input = sections.get(1).expect("Push argument missing");
                let value = value_input.parse::<i32>().unwrap();
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::PUSH, value: value, label: String::from("") }));
            },
            Some(&"pop") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::POP, value: 0, label: String::from("") }));
            },
            Some(&"add") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::ADD, value: 0, label: String::from("") }));
            },
            Some(&"sub") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::SUB, value: 0, label: String::from("") }));
            },
            Some(&"mul") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::MUL, value: 0, label: String::from("") }));
            },
            Some(&"div") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::DIV, value: 0, label: String::from("") }));
            },
            Some(&"print") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::PRINT, value: 0, label: String::from("") }));
            },
            Some(&"printLine") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::PRINTLINE, value: 0, label: String::from("") }));
            },
            Some(&"printAscii") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::PRINTASCII, value: 0, label: String::from("") }));
            },
            Some(&"input") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::INPUT, value: 0, label: String::from("") }));
            },
            Some(&"label") => {
                let label = sections.get(1).expect("Lbl argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::LABEL, value: 0, label: label.to_string() }));
            },
            Some(&"j") => {
                let label = sections.get(1).expect("J argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::J, value: 0, label: label.to_string() }));
            },
            Some(&"je") => {
                let label = sections.get(1).expect("JE argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::JE, value: 0, label: label.to_string() }));
            },
            Some(&"jn") => {
                let label = sections.get(1).expect("JN argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::JN, value: 0, label: label.to_string() }));
            },
            Some(&"jl") => {
                let label = sections.get(1).expect("JL argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::JL, value: 0, label: label.to_string() }));
            },
            Some(&"jg") => {
                let label = sections.get(1).expect("JG argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::JG, value: 0, label: label.to_string() }));
            },
            Some(&"jle") => {
                let label = sections.get(1).expect("JLE argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::JLE, value: 0, label: label.to_string() }));
            },
            Some(&"jge") => {
                let label = sections.get(1).expect("JGE argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::JGE, value: 0, label: label.to_string() }));
            },
            Some(&_) => todo!(),
            None => todo!()
        }
    }

    let mut pointer: usize = 0;
    let size: usize = sm.instructions.len();
    while pointer < size {
        let instruction: &Box<Instruction> = sm.instructions.get(pointer).unwrap();
        pointer += 1;
        match instruction._type {
            InstructionType::PUSH => {
                sm.stack.push(instruction.value);
            },
            InstructionType::POP => {
                sm.stack.pop();
            },
            InstructionType::ADD => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first+second);
            },
            InstructionType::SUB => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first-second);
            },
            InstructionType::MUL => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first*second);
            },
            InstructionType::DIV => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first/second);
            },
            InstructionType::PRINT => {
                let top = sm.stack.pop().expect(&underflow_error);
                print!("{}", top);
                sm.stack.push(top);
            },
            InstructionType::PRINTLINE => {
                let top = sm.stack.pop().expect(&underflow_error);
                println!("{}", top);
                sm.stack.push(top);
            },
            InstructionType::PRINTASCII => {
                let top = sm.stack.pop().expect(&underflow_error);
                print!("{}", char::from_u32(top.try_into().unwrap()).unwrap());
                sm.stack.push(top);
            },
            InstructionType::INPUT => {
                let mut line  = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let value : i32 = line.trim().parse().unwrap();
                sm.stack.push(value);
            }
            InstructionType::LABEL => {
            }
            InstructionType::J => {
                pointer = sm.get_pointer(&instruction.label);
            }
            InstructionType::JE => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first);
                if second == first {
                    pointer = sm.get_pointer(&instruction.label);
                    continue
                }
            }
            InstructionType::JN => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first);
                if first != second {
                    pointer = sm.get_pointer(&instruction.label);
                    continue
                }
            }
            InstructionType::JL => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first);
                if first < second {
                    pointer = sm.get_pointer(&instruction.label);
                    continue
                }
            }
            InstructionType::JG => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first);
                if first > second {
                    pointer = sm.get_pointer(&instruction.label);
                    continue
                }
            }
            InstructionType::JLE => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first);
                if first <= second {
                    pointer = sm.get_pointer(&instruction.label);
                    continue
                }
            }
            InstructionType::JGE => {
                let second = sm.stack.pop().expect(&underflow_error);
                let first = sm.stack.pop().expect(&underflow_error);
                sm.stack.push(first);
                if first >= second {
                    pointer = sm.get_pointer(&instruction.label);
                    continue
                }
            }
        }
    }
}
