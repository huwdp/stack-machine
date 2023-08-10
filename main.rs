use std::env;
use std::fs;
use std::io;

enum InstructionType {
    PUSH,
    ADD,
    SUB,
    MUL,
    DIV,
    PRINT,
    INPUT,  // User input
}

struct Instruction
{
    _type: InstructionType,
    value: i32,
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
                instructions.push(Box::new(Instruction { _type: InstructionType::PUSH, value: value }));
            },
            Some(&"add") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::ADD, value: 0 }));
            },
            Some(&"sub") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::SUB, value: 0 }));
            },
            Some(&"mul") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::MUL, value: 0 }));
            },
            Some(&"div") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::DIV, value: 0 }));
            },
            Some(&"prt") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::PRINT, value: 0 }));
            },
            Some(&"ipt") => {
                instructions.push(Box::new(Instruction { _type: InstructionType::INPUT, value: 0 }));
            },
            Some(&_) => todo!(),
            None => todo!()
        }
    }

    for instruction in instructions {
        match instruction._type {
            InstructionType::PUSH => {
                stack.push(instruction.value);
            },
            InstructionType::ADD => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                stack.push(b+a);
            },
            InstructionType::SUB => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                stack.push(b-a);
            },
            InstructionType::MUL => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                stack.push(b*a);
            },
            InstructionType::DIV => {
                let a = stack.pop().expect("Stack underflow!");
                let b = stack.pop().expect("Stack underflow!");
                stack.push(b/a);
            },
            InstructionType::PRINT => {
                let a = stack.pop();
                println!("{}", a.expect("Stack underflow!"));
            },
            InstructionType::INPUT => {
                let mut line  = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                println!("{}", line);
                let value : i32 = line.trim().parse().unwrap();
                stack.push(value);
            }
        }
    }
}
