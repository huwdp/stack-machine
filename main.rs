use std::env;
use std::fs;

#[derive(PartialEq, Eq)]
enum InstructionType {
    PUSH,
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    PRINT,
    INPUT,  // User input
    LABEL,
    EXIT,
    J,      // Jump
    JE,     // Jump if equal
    JN,     // Jump if not equal
    JL,     // Jump if less than
    JG,     // Jump if greater than
    JLE,    // Jump if less than or equal
    JGE     // Hump if greter than or equal
}

struct StackMachine {
    instructions: Vec<Box<Instruction>>,
    stack: Vec<i32>
}

impl StackMachine {
    fn get_pointer(&self, label: &String) -> usize {
        let mut p : usize = 0;
        for i in &self.instructions {
            p += 1;
            if i.label == *label && i._type == InstructionType::LABEL
            {
                return p;
            }
        }
        panic!("Label '{}' is not found", *label);
    }
}

struct Instruction
{
    _type: InstructionType,
    value: i32,
    label: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sm: StackMachine = StackMachine { instructions: Vec::new(), stack: Vec::new() };
    let file_path: &String = &args[1];
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
            Some(&"prt") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::PRINT, value: 0, label: String::from("") }));
            },
            Some(&"ipt") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::INPUT, value: 0, label: String::from("") }));
            },
            Some(&"lbl") => {
                let label = sections.get(1).expect("Lbl argument missing");
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::LABEL, value: 0, label: label.to_string() }));
            },
            Some(&"exit") => {
                sm.instructions.push(Box::new(Instruction { _type: InstructionType::EXIT, value: 0, label: String::from("") }));
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
    let max_count: usize = sm.instructions.len();
    while pointer < max_count {
        let instruction: &Box<Instruction> = sm.instructions.get(pointer).unwrap();
        match instruction._type {
            InstructionType::PUSH => {
                sm.stack.push(instruction.value);
                pointer += 1;
            },
            InstructionType::POP => {
                sm.stack.pop();
            },
            InstructionType::ADD => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b+a);
                pointer += 1;
            },
            InstructionType::SUB => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b-a);
                pointer += 1;
            },
            InstructionType::MUL => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b*a);
                pointer += 1;
            },
            InstructionType::DIV => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b/a);
                pointer += 1;
            },
            InstructionType::PRINT => {
                let a = sm.stack.pop().expect("Stack underflow!");
                println!("{}", a);
                sm.stack.push(a);
                pointer += 1;
            },
            InstructionType::INPUT => {
                let mut line  = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let value : i32 = line.trim().parse().unwrap();
                sm.stack.push(value);
                pointer += 1;
            }
            InstructionType::LABEL => {
                pointer += 1;
                print!("Label: {}\n", instruction.label)
            }
            InstructionType::EXIT => {
                pointer = max_count;
            }
            InstructionType::J => {
                pointer = sm.get_pointer(&instruction.label);
            }
            InstructionType::JE => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b);
                if b == a {
                    pointer = sm.get_pointer(&instruction.label);
                }
                else {
                    pointer += 1;
                }
            }
            InstructionType::JN => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b);
                if b != a {
                    pointer = sm.get_pointer(&instruction.label);
                }
                else {
                    pointer += 1;
                }
            }
            InstructionType::JL => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b);
                if a < b {
                    pointer = sm.get_pointer(&instruction.label);
                }
                else {
                    pointer += 1;
                }
            }
            InstructionType::JG => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b);
                if a > b {
                    pointer = sm.get_pointer(&instruction.label);
                }
                else {
                    pointer += 1;
                }
            }
            InstructionType::JLE => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b);
                if a <= b {
                    pointer = sm.get_pointer(&instruction.label);
                }
                else {
                    pointer += 1;
                }
            }
            InstructionType::JGE => {
                let a = sm.stack.pop().expect("Stack underflow!");
                let b = sm.stack.pop().expect("Stack underflow!");
                sm.stack.push(b);
                if a >= b {
                    pointer = sm.get_pointer(&instruction.label);
                }
                else {
                    pointer += 1;
                }
            }
        }
    }
}
