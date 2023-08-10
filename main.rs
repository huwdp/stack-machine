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
    JL,     // Jump if less than
}

struct Instruction
{
    _type: InstructionType,
    value: i32,
    label: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut instructions: Vec<Box<Instruction>> = Vec::new();
    let mut stack: Vec<i32> = Vec::new();
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
                instructions.push(Box::new(Instruction { _type: InstructionType::PUSH, value: value, label: String::from("") }));
            },
            Some(&"pop") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::POP, value: 0, label: String::from("") }));
            },
            Some(&"add") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::ADD, value: 0, label: String::from("") }));
            },
            Some(&"sub") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::SUB, value: 0, label: String::from("") }));
            },
            Some(&"mul") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::MUL, value: 0, label: String::from("") }));
            },
            Some(&"div") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::DIV, value: 0, label: String::from("") }));
            },
            Some(&"prt") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::PRINT, value: 0, label: String::from("") }));
            },
            Some(&"ipt") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::INPUT, value: 0, label: String::from("") }));
            },
            Some(&"lbl") => {
                let label = sections.get(1).expect("Lbl argument missing");
                instructions.push(Box::new(Instruction { _type: InstructionType::LABEL, value: 0, label: label.to_string() }));
            },
            Some(&"jl") => {
                let label = sections.get(1).expect("JL argument missing");
                instructions.push(Box::new(Instruction { _type: InstructionType::JL, value: 0, label: label.to_string() }));
            }
            Some(&_) => todo!(),
            None => todo!()
        }
    }

    let mut pointer: usize = 0;
    let max_count: usize = instructions.len();
    while pointer < max_count {
        let instruction: &Box<Instruction> = instructions.get(pointer).unwrap();
        match instruction._type {
            InstructionType::PUSH => {
                stack.push(instruction.value);
                pointer += 1;
            },
            InstructionType::POP => {
                stack.pop();
            },
            InstructionType::ADD => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                stack.push(b+a);
                pointer += 1;
            },
            InstructionType::SUB => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                stack.push(b-a);
                pointer += 1;
            },
            InstructionType::MUL => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                stack.push(b*a);
                pointer += 1;
            },
            InstructionType::DIV => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                stack.push(b/a);
                pointer += 1;
            },
            InstructionType::PRINT => {
                let a = stack.pop().expect("Stack underflow!");
                println!("{}", a);
                stack.push(a);
                pointer += 1;
            },
            InstructionType::INPUT => {
                let mut line  = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let value : i32 = line.trim().parse().unwrap();
                stack.push(value);
                pointer += 1;
            }
            InstructionType::LABEL => {
                pointer += 1;
            }
            InstructionType::JL => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                if b < a {
                    // Find label and change pointer
                    let mut p : usize = 0;
                    for i in &instructions {
                        p += 1;
                        if i.label == instruction.label
                        {
                            break;
                        }
                    }
                    if p > max_count
                    {
                        todo!(); // No label found
                    }
                    stack.push(b);
                    pointer = p;
                }
                else {
                    stack.push(b);
                    pointer += 1;
                }
            }
        }
    }
}
