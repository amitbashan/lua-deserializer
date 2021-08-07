# Lua Deserializer

Bare-bones Lua deserializer.

## Bytecode Format

This will deserialize a slightly modified Lua 5.1.5 bytecode format.

### Changes

* Instruction operation code is a byte instead of 6 bits
* Instruction argument B is a byte instead of 9 bits
* Instruction argument C is a byte instead of 9 bits
* Instruction argument Bx is 16 bits instead of 18 bits
