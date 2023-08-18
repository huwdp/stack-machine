
# Huw's Stack Machine
A basic stack machine implemented in Rust.

## Instructions

*int = integer number*

Stack contains a list of integers. We run an instruction to interact with the stack. So for example `push 1` will add 1 to stack list. `pop` will remove the top (value: 1) from the stack.

| Instruction      | Description                                                                                               |
| ---------------- | --------------------------------------------------------------------------------------------------------- |
| add              | Pop two off stack, add the two and push new value onto stack.                                             |
| sub              | Pop two off stack, subtract the two and new value push onto stack.                                        |
| mul              | Pop two off stack, multiply the two and new value push onto stack.                                        |
| div              | Pop two off stack, divide the two and push new value onto stack.                                          |
| push `<INT>`     | Push int onto stack.                                                                                      |
| pop              | Pop int off  stack.                                                                                       |
| input            | Read user input                                                                                           |
| print            | Print top of stack.                                                                                       |
| printLine        | Print top of stack.                                                                                       |
| printAscii       | Print top of stack as an ASCII character.                                                                 |
| label `<LABEL>`  | Used as locations for jumps.                                                                              |
| j `<LABEL>`      | Jump to label.                                                                                            |
| je `<LABEL>`     | Jump to label if equal.                                                                                   |
| jn `<LABEL>`     | Jump to label if not equal.                                                                               |
| jg `<LABEL>`     | Jump to label if greater than.                                                                            |
| jge `<LABEL>`    | Jump to label if greater than or equal to.                                                                |
| jl `<LABEL>`     | Jump to label if less than.                                                                               |
| jle `<LABEL>`    | Jump to label if less than or equal to.                                                                   |

## Process of jump
1. Compare two top items on stack
2. Pop top of stack
3. Jump to label if comparison is true, otherwise, we carry onto next line.

## Example
```
push 0
label main
printLine
push 1
add
push 10
jl main
```
The example above prints a list of numbers from 0 to 9.

## Usage
```
rustc main.rs
./main FILENAME
```
