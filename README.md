# Qasm
#### The polite assembler

## Why?

Felt inspired, wanted to try making one!

##### No, I meant why is it "The polite assembler"

Qasm aims to generate warnings helpful warnings and errors, kinda like Rust's, making it friendlier and more polite than other assemblers.

## Requirements

### Compiling

 - Compiling Qasm from source requires Rust (`rustc 1.35.0 nightly build`).
 - After that its best to install it as a system binary through cargo `cargo install qasm`

## Usage

It's always a good idea to read the manual page (`man qasm`) or the help page (`qasm -h`) to get a better understanding of the assembler usage.

## Example

```nasm
global _start

section .data
L0: db "Hello world", 10

section .text

_start:
    mov rax, 1
    mov rdi, 1
    mov rsi, L0
    mov rdx, 14
    syscall

    mov rax, 60
    mov rdi, 0
    ret
```
