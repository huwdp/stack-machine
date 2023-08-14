
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
| push `<INTEGER>` | Push int onto stack.                                                                                      |
| pop              | Pop int off  stack.                                                                                       |
| print            | Print top of stack.                                                                                       |
| printLine        | Print top of stack.                                                                                       |
| printAscii       | Print top of stack as an ASCII character.                                                                 |
| label `<LABEL>`  | Used as locations for jumps.                                                                              |
| j `<LABEL>`      | Jump to label.                                                                                            |
| je `<LABEL>`     | Pop two off stack. If the two are equal then jump to label.                                               |
| jn `<LABEL>`     | Pop two off stack. If the two are not equal then jump to label.                                           |
| jg `<LABEL>`     | Pop two off stack. If the first int is greater than the second int, then jump to label.                   |
| jge `<LABEL>`    | Pop two off stack. If the first int is greater than or equal to the second int, then jump to label.       |
| jl `<LABEL>`     | Pop two off stack. If the first int is less than the second int, then jump to label.                      |
| jle `<LABEL>`    | Pop two off stack. If the first int is less than or equal to the second int, then jump to label.          |

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
