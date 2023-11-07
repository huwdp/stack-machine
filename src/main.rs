use std::env;
use std::fs;
use std::convert::TryInto;

#[derive(PartialEq, Eq)]
enum InstructionType {
    PUSHINT,    // Push value onto stack.
    PUSHFLOAT,  // Push value onto stack.
    POP,        // Pop value off top of stack.
    ADDI,       // Pop two off stack then add them and put onto stack.
    SUBI,       // Pop two off stack then substract them and put onto stack.
    MULI,       // Pop two off stack then multiply them and put onto stack.
    DIVI,       // Pop two off stack then divide them and put onto stack.
    ADDF,       // Pop two off stack then add them and put onto stack.
    SUBF,       // Pop two off stack then substract them and put onto stack.
    MULF,       // Pop two off stack then multiply them and put onto stack.
    DIVF,       // Pop two off stack then divide them and put onto stack.
    PRINT,      // Print value on top of stack.
    PRINTLINE,  // Print line of value on top of stack.
    PRINTASCII, // Print ASCII character on top of stack.
    INPUTINT,   // Read user input and push value on stack as int.
    INPUTFLOAT, // Read user input and push value on stack as int.
    LABEL,      // Label for jumps to change the instruction pointer to.
    J,          // Jump to label
    JE,         // Jump to label if equal
    JN,         // Jump to label if not equal
    JL,         // Jump to label if less than
    JG,         // Jump to label if greater than
    JLE,        // Jump to label if less than or equal
    JGE,        // Jump to label if greter than or equal
    TOINT,      // Pop off stack and convert to int32
    TOFLOAT,     // Pop off stack and convert to float32
    DUPLICATE,  // Pop off stack and push two copies onto stack
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ValueType {
    INT,
    FLOAT
}

#[derive(Clone, Copy)]
struct Value {
    int_value: i32,
    float_value: f32,
    value_type: ValueType,
}

struct StackMachine {
    instructions: Vec<Box<Instruction>>,
    stack: Vec<Value>
}

struct Instruction
{
    instruction_type: InstructionType,
    value: Option<Value>,
    label: String
}

impl StackMachine {
    fn get_pointer(&self, label: &String) -> usize {
        let mut p : usize = 0;
        for i in &self.instructions {
            if i.label == *label && i.instruction_type == InstructionType::LABEL
            {
                return p;
            }
            p += 1;
        }
        panic!("Label '{}' is not found", *label);
    }
}

static DIFFERENT_TYPES: &str = "Cannot compare different value types!";
static STACK_UNDERFLOW: &str = "Stack underflow";
static TOP_IS_NOT_INTEGER: &str = "Top of stack is not type of integer";
static TOP_IS_NOT_FLOAT: &str = "Top of stack is not type of float";

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
            Some(&"pushi") => {
                let value_input = sections.get(1).expect("Push argument missing");
                let value = value_input.parse::<i32>().unwrap();
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::PUSHINT, value: Some(Value { int_value: value, float_value: 0.0, value_type: ValueType::INT, }), label: String::from("") }));
            },
            Some(&"pushf") => {
                let value_input = sections.get(1).expect("Push argument missing");
                let value = value_input.parse::<f32>().unwrap();
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::PUSHFLOAT, value: Some(Value { int_value: 0, float_value: value, value_type: ValueType::FLOAT}),  label: String::from("") }));
            },
            Some(&"pop") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::POP, value: None, label: String::from("") }));
            },
            Some(&"addi") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::ADDI, value: None, label: String::from("") }));
            },
            Some(&"subi") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::SUBI, value: None, label: String::from("") }));
            },
            Some(&"muli") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::MULI, value: None, label: String::from("") }));
            },
            Some(&"divi") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::DIVI, value: None, label: String::from("") }));
            },
            Some(&"addf") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::ADDF, value: None, label: String::from("") }));
            },
            Some(&"subf") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::SUBF, value: None, label: String::from("") }));
            },
            Some(&"mulf") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::MULF, value: None, label: String::from("") }));
            },
            Some(&"divf") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::DIVF, value: None, label: String::from("") }));
            },
            Some(&"print") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::PRINT, value: None, label: String::from("") }));
            },
            Some(&"printl") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::PRINTLINE, value: None, label: String::from("") }));
            },
            Some(&"printa") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::PRINTASCII, value: None, label: String::from("") }));
            },
            Some(&"inputi") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::INPUTINT, value: None, label: String::from("") }));
            },
            Some(&"inputf") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::INPUTFLOAT, value: None, label: String::from("") }));
            },
            Some(&"label") => {
                let label = sections.get(1).expect("Lbl argument missing");
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::LABEL, value: None, label: label.to_string() }));
            },
            Some(&"j") => {
                let label = sections.get(1).expect("J argument missing");
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::J, value: None, label: label.to_string() }));
            },
            Some(&"je") => {
                let label = sections.get(1).expect("JE argument missing");
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::JE, value: None, label: label.to_string() }));
            },
            Some(&"jn") => {
                let label = sections.get(1).expect("JN argument missing");
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::JN, value: None, label: label.to_string() }));
            },
            Some(&"jl") => {
                let label = sections.get(1).expect("JL argument missing");
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::JL, value: None, label: label.to_string() }));
            },
            Some(&"jg") => {
                let label = sections.get(1).expect("JG argument missing");
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::JG, value: None, label: label.to_string() }));
            },
            Some(&"jle") => {
                let label = sections.get(1).expect("JLE argument missing");
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::JLE, value: None, label: label.to_string() }));
            },
            Some(&"jge") => {
                let label = sections.get(1).expect("JGE argument missing");
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::JGE, value: None, label: label.to_string() }));
            },
            Some(&"toi") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::TOINT, value: None, label: String::from("") }));
            },
            Some(&"dup") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::DUPLICATE, value: None, label: String::from("") }));
            },
            Some(&"tof") => {
                sm.instructions.push(Box::new(Instruction { instruction_type: InstructionType::TOFLOAT, value: None, label: String::from("") }));
            },
            Some(&_) => panic!("Instruction not implemented"),
            None => todo!()
        }
    }
    let mut pointer: usize = 0;
    let size: usize = sm.instructions.len();
    while pointer < size {
        let instruction: &Box<Instruction> = sm.instructions.get(pointer).unwrap();
        pointer += 1;
        match instruction.instruction_type {
            InstructionType::PUSHINT => {
                let data = instruction.value.as_ref().unwrap();
                sm.stack.push(Value { int_value: data.int_value, float_value: data.float_value, value_type: ValueType::INT });
            },
            InstructionType::PUSHFLOAT => {
                let data = instruction.value.as_ref().unwrap();
                sm.stack.push(Value { int_value: data.int_value, float_value: data.float_value, value_type: ValueType::FLOAT });
            },
            InstructionType::POP => {
                sm.stack.pop();
            },
            InstructionType::ADDI => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(second.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                assert!(first.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                sm.stack.push(Value { int_value: first.int_value + second.int_value, float_value: 0.0, value_type: ValueType::INT, });
            },
            InstructionType::SUBI => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(second.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                assert!(first.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                sm.stack.push(Value { int_value: first.int_value - second.int_value, float_value: 0.0, value_type: ValueType::INT, });
            },
            InstructionType::MULI => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(second.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                assert!(first.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                sm.stack.push(Value { int_value: first.int_value * second.int_value, float_value: 0.0, value_type: ValueType::INT, });
            },
            InstructionType::DIVI => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(second.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                assert!(first.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                sm.stack.push(Value { int_value: first.int_value / second.int_value, float_value: 0.0, value_type: ValueType::INT, });
            },
            InstructionType::ADDF => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(second.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                assert!(first.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                sm.stack.push(Value { int_value: 0, float_value: first.float_value + second.float_value, value_type: ValueType::FLOAT, });
            },
            InstructionType::SUBF => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(second.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                assert!(first.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                sm.stack.push(Value { int_value: 0, float_value: first.float_value - second.float_value, value_type: ValueType::FLOAT, });
            },
            InstructionType::MULF => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(second.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                assert!(first.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                sm.stack.push(Value { int_value: 0, float_value: first.float_value * second.float_value, value_type: ValueType::FLOAT, });
            },
            InstructionType::DIVF => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(second.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                assert!(first.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                sm.stack.push(Value { int_value: 0, float_value: first.float_value / second.float_value, value_type: ValueType::FLOAT, });
            },
            InstructionType::PRINT => {
                let top = sm.stack.pop().expect(STACK_UNDERFLOW);
                match top.value_type {
                    ValueType::INT => {
                        print!("{}", top.int_value);
                    },
                    ValueType::FLOAT => {
                        print!("{}", top.float_value);
                    }
                }
                sm.stack.push(top);
            },
            InstructionType::PRINTLINE => {
                let top = sm.stack.pop().expect(STACK_UNDERFLOW);
                match top.value_type {
                    ValueType::INT => {
                        println!("{}", top.int_value);
                    },
                    ValueType::FLOAT => {
                        println!("{}", top.float_value);
                    }
                }
                sm.stack.push(top);
            },
            InstructionType::PRINTASCII => {
                let top = sm.stack.pop().expect(STACK_UNDERFLOW);
                match top.value_type {
                    ValueType::INT => {
                        print!("{}", char::from_u32(top.int_value.try_into().unwrap()).unwrap());
                    },
                    ValueType::FLOAT => {
                        panic!("{}", TOP_IS_NOT_INTEGER);
                    }
                }
                sm.stack.push(top);
            },
            InstructionType::INPUTINT => {
                let mut line  = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let value : i32 = line.trim().parse().unwrap();
                sm.stack.push(Value { int_value: value, float_value: 0.0, value_type: ValueType::INT, });
            },
            InstructionType::INPUTFLOAT => {
                let mut line  = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let value : f32 = line.trim().parse().unwrap();
                sm.stack.push(Value { int_value: 0, float_value: value, value_type: ValueType::FLOAT, });
            }
            InstructionType::LABEL => { }
            InstructionType::J => {
                pointer = sm.get_pointer(&instruction.label);
            },
            InstructionType::JE => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                if second.value_type != first.value_type {
                    panic!("{}", &DIFFERENT_TYPES);
                }
                match first.value_type {
                    ValueType::INT => {
                        if first.int_value == second.int_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    },
                    ValueType::FLOAT => {
                        if first.float_value == second.float_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    }
                }
                sm.stack.push(first);
            },
            InstructionType::JN => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                if second.value_type != first.value_type {
                    panic!("{}", &DIFFERENT_TYPES);
                }
                match first.value_type {
                    ValueType::INT => {
                        if first.int_value != second.int_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    },
                    ValueType::FLOAT => {
                        if first.float_value != second.float_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    }
                }
                sm.stack.push(first);
            },
            InstructionType::JL => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                if second.value_type != first.value_type {
                    panic!("{}", &DIFFERENT_TYPES);
                }
                match first.value_type {
                    ValueType::INT => {
                        if first.int_value < second.int_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    },
                    ValueType::FLOAT => {
                        if first.float_value < second.float_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    }
                }
                sm.stack.push(first);
            },
            InstructionType::JG => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                if second.value_type != first.value_type {
                    panic!("{}", &DIFFERENT_TYPES);
                }
                match first.value_type {
                    ValueType::INT => {
                        if first.int_value > second.int_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    },
                    ValueType::FLOAT => {
                        if first.float_value > second.float_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    }
                }
                sm.stack.push(first);
            },
            InstructionType::JLE => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                if second.value_type != first.value_type {
                    panic!("{}", &DIFFERENT_TYPES);
                }
                match first.value_type {
                    ValueType::INT => {
                        if first.int_value <= second.int_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    },
                    ValueType::FLOAT => {
                        if first.float_value <= second.float_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    }
                }
                sm.stack.push(first);
            },
            InstructionType::JGE => {
                let second = sm.stack.pop().expect(STACK_UNDERFLOW);
                let first = sm.stack.pop().expect(STACK_UNDERFLOW);
                if second.value_type != first.value_type {
                    panic!("{}", &DIFFERENT_TYPES);
                }
                match first.value_type {
                    ValueType::INT => {
                        if first.int_value >= second.int_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    },
                    ValueType::FLOAT => {
                        if first.float_value >= second.float_value {
                            pointer = sm.get_pointer(&instruction.label);
                        }
                    }
                }
                sm.stack.push(first);
            },
            InstructionType::TOINT => {
                let top = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(top.value_type == ValueType::FLOAT, "{}", TOP_IS_NOT_FLOAT);
                let val: i32 = top.float_value as i32;
                sm.stack.push(Value { int_value: val, float_value: 0.0, value_type: ValueType::INT, });
            },
            InstructionType::TOFLOAT => {
                let top = sm.stack.pop().expect(STACK_UNDERFLOW);
                assert!(top.value_type == ValueType::INT, "{}", TOP_IS_NOT_INTEGER);
                let val: f32 = top.int_value as f32;
                sm.stack.push(Value { int_value: 0, float_value: val, value_type: ValueType::FLOAT, });
            }
            InstructionType::DUPLICATE => {
                let top = sm.stack.pop().expect(STACK_UNDERFLOW).clone();
                sm.stack.push(top);
                sm.stack.push(Value { int_value: top.int_value, float_value: top.float_value, value_type: ValueType::INT });
                sm.stack.push(Value { int_value: top.int_value, float_value: top.float_value, value_type: ValueType::INT });
            }
        }
    }
}
