# Chip8-asm

Currently this is a basic disassembler for Chip8 ROMs.
Point it at a Chip8 ROM file and it will print the assembly for that problem to STDOUT

## Usage Guide

To run Chip8-asm against a file:

`; cargo run <Chip8 ROM path>`

Output: 
```
CLS
LD V3 $0
LD V4 $1
LD V5 $238
SE V5 $238
JP $784
LD V3 $0
LD V4 $2
LD V5 $238
LD V6 $238
SE V5 V6
JP $784
LD V3 $0
LD V4 $3
LD V5 $238
SNE V5 $253
JP $784
LD V3 $0
LD V4 $4
LD V5 $238
ADD V5 $1
SE V5 $239
JP $784
LD V3 $0
LD V4 $5
...
```

Currently if you want to store the output to a file, you can just redirect STDOUT to a file as shown:

`; cargo run <Chip8 ROM path> > <desired file>`

## Todo
- [ ] Disassembly 
  - [x] Basic Disassembly 
  - [ ] Tests
- [ ] Assembly
  - [x] Basic Assembly 
  - [ ] Tests
