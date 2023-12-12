
# Huw's Stack Machine
A basic stack machine implemented in Rust.

## Instructions

*int = integer number*
*float = float number*

Stack contains a list of integers. We run an instruction to interact with the stack. So for example `push 1` will add 1 to stack list. `pop` will remove the top (value: 1) from the stack.

| Instruction         | Description                                                                                               |
| --------------------| --------------------------------------------------------------------------------------------------------- |
| addi                | Pop two off stack, add the two and push new value onto stack. Two numbers must be integer type.           |
| subi                | Pop two off stack, subtract the two and new value push onto stack. Two numbers must be integer type       |
| muli                | Pop two off stack, multiply the two and new value push onto stack. Two numbers must be integer type       |
| divi                | Pop two off stack, divide the two and push new value onto stack. Two numbers must be integer type         |
| addf                | Pop two off stack, add the two and push new value onto stack. Output will be float type.                  |
| subf                | Pop two off stack, subtract the two and new value push onto stack. Output will be float type.             |
| mulf                | Pop two off stack, multiply the two and new value push onto stack. Output will be float type.             |
| divf                | Pop two off stack, divide the two and push new value onto stack. Output will be float type.               |
| pushi `<INT>`       | Push int onto stack.                                                                                      |
| pushf `<FLOAT>`     | Push int onto stack.                                                                                      |
| pop                 | Pop int off  stack.                                                                                       |
| inputi              | Read user input as int.                                                                                   |
| inputf              | Read user input as float.                                                                                 |
| print               | Print top of stack.                                                                                       |
| printl              | Print top of stack.                                                                                       |
| printa              | Print top of stack as an ASCII character.                                                                 |
| label `<LABEL>`     | Used as locations for jumps.                                                                              |
| j `<LABEL>`         | Jump to label.                                                                                            |
| je `<LABEL>`        | Jump to label if equal.                                                                                   |
| jn `<LABEL>`        | Jump to label if not equal.                                                                               |
| jg `<LABEL>`        | Jump to label if greater than.                                                                            |
| jge `<LABEL>`       | Jump to label if greater than or equal to.                                                                |
| jl `<LABEL>`        | Jump to label if less than.                                                                               |
| jle `<LABEL>`       | Jump to label if less than or equal to.                                                                   |
| toi                 | Convert top of stack to integer.                                                                          |
| tof                 | Convert top of stack to float.                                                                            |
| dup                 | Duplicates top of stack.                                                                                  |

## Process of jump
1. Compare two top items on stack
2. Pop top of stack
3. Jump to label if comparison is true, otherwise, we carry onto next line.

## Example
```
pushInt 0
label main
printLine
pushInt 1
addi
push 10
jl main
```
The example above prints a list of numbers from 0 to 9.

## Usage
```
cargo build
cargo run FILENAME
```
