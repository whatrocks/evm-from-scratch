use std::str::FromStr;

use primitive_types::U256;
use sha3::{Digest, Keccak256};

// instructions
const STOP: u8 = 0;
const ADD: u8 = 1;
const MUL: u8 = 2;
const SUB: u8 = 3;
const DIV: u8 = 4;
const SDIV: u8 = 5;
const MOD: u8 = 6;
const SMOD: u8 = 7;
const LT: u8 = 16;
const GT: u8 = 17;
const SLT: u8 = 18;
const SGT: u8 = 19;
const EQ: u8 = 20;
const ISZERO: u8 = 21;
const AND: u8 = 22;
const OR: u8 = 23;
const XOR: u8 = 24;
const NOT: u8 = 25;
const BYTE: u8 = 26;
const SHA3: u8 = 32;
const ADDRESS: u8 = 48;
const POP: u8 = 80;
const MLOAD: u8 = 81;
const MSTORE: u8 = 82;
const MSTORE8: u8 = 83;
const JUMP: u8 = 86;
const JUMPI: u8 = 87;
const PC: u8 = 88;
const MSIZE: u8 = 89;
const GAS: u8 = 90;
const JUMPDEST: u8 = 91;
const PUSH1: u8 = 96;
const PUSH2: u8 = 97;
const PUSH3: u8 = 98;
const PUSH4: u8 = 99;
const PUSH5: u8 = 100;
const PUSH6: u8 = 101;
const PUSH7: u8 = 102;
const PUSH8: u8 = 103;
const PUSH9: u8 = 104;
const PUSH10: u8 = 105;
const PUSH11: u8 = 106;
const PUSH12: u8 = 107;
const PUSH13: u8 = 108;
const PUSH14: u8 = 109;
const PUSH15: u8 = 110;
const PUSH16: u8 = 111;
const PUSH17: u8 = 112;
const PUSH18: u8 = 113;
const PUSH19: u8 = 114;
const PUSH20: u8 = 115;
const PUSH21: u8 = 116;
const PUSH22: u8 = 117;
const PUSH23: u8 = 118;
const PUSH24: u8 = 119;
const PUSH25: u8 = 120;
const PUSH26: u8 = 121;
const PUSH27: u8 = 122;
const PUSH28: u8 = 123;
const PUSH29: u8 = 124;
const PUSH30: u8 = 125;
const PUSH31: u8 = 126;
const PUSH32: u8 = 127;
const DUP1: u8 = 128;
const DUP2: u8 = 129;
const DUP3: u8 = 130;
const DUP4: u8 = 131;
const DUP5: u8 = 132;
const DUP6: u8 = 133;
const DUP7: u8 = 134;
const DUP8: u8 = 135;
const DUP9: u8 = 136;
const DUP10: u8 = 137;
const DUP11: u8 = 138;
const DUP12: u8 = 139;
const DUP13: u8 = 140;
const DUP14: u8 = 141;
const DUP15: u8 = 142;
const DUP16: u8 = 143;
const SWAP1: u8 = 144;
const SWAP3: u8 = 146;

struct Memory {
    storage: Vec<u8>,
    size: usize,
}
impl Memory {
    fn new() -> Memory {
        Memory {
            storage: vec![0; 1024 * 1024],
            size: 0,
        }
    }
    fn get_size(&mut self) -> U256 {
        return U256::from(self.size);
    }
    fn store(&mut self, offset: usize, value: U256) {
        for i in 0..32 {
            let byte = value.byte(31 - i);
            let idx = offset + i;
            if idx > self.size {
                self.size = ((idx + 31) / 32) * 32;
            }
            self.storage[idx] = byte;
        }
    }
    fn store_byte(&mut self, offset: usize, byte: u8) {
        if offset > self.size {
            self.size = ((offset + 31) / 32) * 32;
        }
        self.storage[offset] = byte;
    }
    fn load(&mut self, offset: usize) -> U256 {
        if offset > self.size {
            self.size = ((offset + 31) / 32) * 32;
        }
        let upper_idx = offset + 32;
        if upper_idx > self.size {
            self.size = ((upper_idx + 31) / 32) * 32;
        }
        let bytes = &self.storage[offset..offset + 32];
        return U256::from_big_endian(bytes);
    }
}

pub fn evm(code: impl AsRef<[u8]>) -> Vec<U256> {
    // convert instructions
    let c = code.as_ref();
    println!("{:?}", c);

    // init result
    let mut stack = Vec::new();

    // init program counter
    let mut pc = 0;

    // init memory
    let mut memory = Memory::new();

    // process instructions
    while pc < c.len() {
        let instruction = c[pc];
        println!("instruction: {:?}, stack: {:?}", instruction, stack);
        match instruction {
            PUSH1 => {
                pc += 1;
                stack.push(U256::from(c[pc]));
                println!("stack: {:?}", stack);
            }
            PUSH2 => {
                pc += push_n_bytes(2, &mut stack, &c[pc + 1..pc + 3]);
            }
            PUSH3 => {
                pc += push_n_bytes(3, &mut stack, &c[pc + 1..pc + 4]);
            }
            PUSH4 => {
                pc += push_n_bytes(4, &mut stack, &c[pc + 1..pc + 5]);
            }
            PUSH5 => {
                pc += push_n_bytes(5, &mut stack, &c[pc + 1..pc + 6]);
            }
            PUSH6 => {
                pc += push_n_bytes(6, &mut stack, &c[pc + 1..pc + 7]);
            }
            PUSH7 => {
                pc += push_n_bytes(7, &mut stack, &c[pc + 1..pc + 8]);
            }
            PUSH8 => {
                pc += push_n_bytes(8, &mut stack, &c[pc + 1..pc + 9]);
            }
            PUSH9 => {
                pc += push_n_bytes(9, &mut stack, &c[pc + 1..pc + 10]);
            }
            PUSH10 => {
                pc += push_n_bytes(10, &mut stack, &c[pc + 1..pc + 11]);
            }
            PUSH11 => {
                pc += push_n_bytes(11, &mut stack, &c[pc + 1..pc + 12]);
            }
            PUSH12 => {
                pc += push_n_bytes(12, &mut stack, &c[pc + 1..pc + 13]);
            }
            PUSH13 => {
                pc += push_n_bytes(13, &mut stack, &c[pc + 1..pc + 14]);
            }
            PUSH14 => {
                pc += push_n_bytes(14, &mut stack, &c[pc + 1..pc + 15]);
            }
            PUSH15 => {
                pc += push_n_bytes(15, &mut stack, &c[pc + 1..pc + 16]);
            }
            PUSH16 => {
                pc += push_n_bytes(16, &mut stack, &c[pc + 1..pc + 17]);
            }
            PUSH17 => {
                pc += push_n_bytes(17, &mut stack, &c[pc + 1..pc + 18]);
            }
            PUSH18 => {
                pc += push_n_bytes(18, &mut stack, &c[pc + 1..pc + 19]);
            }
            PUSH19 => {
                pc += push_n_bytes(19, &mut stack, &c[pc + 1..pc + 20]);
            }
            PUSH20 => {
                pc += push_n_bytes(20, &mut stack, &c[pc + 1..pc + 21]);
            }
            PUSH21 => {
                pc += push_n_bytes(21, &mut stack, &c[pc + 1..pc + 22]);
            }
            PUSH22 => {
                pc += push_n_bytes(22, &mut stack, &c[pc + 1..pc + 23]);
            }
            PUSH23 => {
                pc += push_n_bytes(23, &mut stack, &c[pc + 1..pc + 24]);
            }
            PUSH24 => {
                pc += push_n_bytes(24, &mut stack, &c[pc + 1..pc + 25]);
            }
            PUSH25 => {
                pc += push_n_bytes(25, &mut stack, &c[pc + 1..pc + 26]);
            }
            PUSH26 => {
                pc += push_n_bytes(26, &mut stack, &c[pc + 1..pc + 27]);
            }
            PUSH27 => {
                pc += push_n_bytes(27, &mut stack, &c[pc + 1..pc + 28]);
            }
            PUSH28 => {
                pc += push_n_bytes(28, &mut stack, &c[pc + 1..pc + 29]);
            }
            PUSH29 => {
                pc += push_n_bytes(29, &mut stack, &c[pc + 1..pc + 30]);
            }
            PUSH30 => {
                pc += push_n_bytes(30, &mut stack, &c[pc + 1..pc + 31]);
            }
            PUSH31 => {
                pc += push_n_bytes(31, &mut stack, &c[pc + 1..pc + 32]);
            }
            PUSH32 => {
                pc += push_n_bytes(32, &mut stack, &c[pc + 1..pc + 33]);
            }
            DUP1 | DUP2 | DUP3 | DUP4 | DUP5 | DUP6 | DUP7 | DUP8 | DUP9 | DUP10 | DUP11
            | DUP12 | DUP13 | DUP14 | DUP15 | DUP16 => {
                stack.push(stack[0]);
            }
            SWAP1 => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                stack.push(b);
                stack.push(a);
            }
            SWAP3 => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let idx = stack.len() - 3;
                let a = stack[idx];
                let c = a;
                let a = b;
                let b = c;
                stack[idx] = a;
                stack.push(b);
            }
            POP => {
                stack.pop();
            }
            JUMP => {
                let counter = stack.pop().unwrap_or_else(|| U256::from(0));
                pc = counter.as_usize();
            }
            JUMPI => {
                let counter = stack.pop().unwrap_or_else(|| U256::from(0));
                println!("c {:?}", counter);
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                println!("b {:?}", b);
                if b != U256::from(0) {
                    pc = counter.as_usize();
                }
            }
            JUMPDEST => {}
            STOP => {
                break;
            }
            ADD => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let (sum, _) = U256::overflowing_add(b, a);
                stack.push(sum);
            }
            MUL => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let (product, _) = U256::overflowing_mul(b, a);
                stack.push(product);
            }
            SUB => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let (diff, _) = U256::overflowing_sub(b, a);
                stack.push(diff);
            }

            DIV => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                if a == U256::from(0) {
                    stack.push(a);
                } else {
                    let quotient = b / a;
                    stack.push(quotient);
                }
            }
            SDIV => {
                // signed integer division
                let mut b = stack.pop().unwrap_or_else(|| U256::from(0));
                let b_is_neg = U256::leading_zeros(&b) == 0;
                if b_is_neg {
                    (b, _) = U256::overflowing_add(!b, U256::from(1));
                }

                let mut a = stack.pop().unwrap_or_else(|| U256::from(0));
                let a_is_neg = U256::leading_zeros(&a) == 0;
                if a_is_neg {
                    (a, _) = U256::overflowing_add(!a, U256::from(1));
                }

                let mut quotient = b / a;

                if (a_is_neg && !b_is_neg) || (!a_is_neg && b_is_neg) {
                    (quotient, _) = U256::overflowing_add(!quotient, U256::from(1));
                }
                stack.push(quotient);
            }
            MOD => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                if a == U256::from(0) {
                    stack.push(a);
                } else {
                    let modulus = b % a;
                    stack.push(modulus);
                }
            }
            SMOD => {
                let mut b = stack.pop().unwrap_or_else(|| U256::from(0));
                let b_is_neg = U256::leading_zeros(&b) == 0;
                if b_is_neg {
                    (b, _) = U256::overflowing_add(!b, U256::from(1));
                }

                let mut a = stack.pop().unwrap_or_else(|| U256::from(0));
                let a_is_neg = U256::leading_zeros(&a) == 0;
                if a_is_neg {
                    (a, _) = U256::overflowing_add(!a, U256::from(1));
                }
                if a == U256::from(0) {
                    stack.push(a);
                    break;
                }
                let mut modulus = b % a;
                if a_is_neg {
                    (modulus, _) = U256::overflowing_add(!modulus, U256::from(1));
                }
                stack.push(modulus);
            }
            LT => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let is_less_than = b < a;
                if is_less_than {
                    stack.push(U256::from(0x1));
                } else {
                    stack.push(U256::from(0x0));
                }
            }
            GT => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let is_greater_than = b > a;
                if is_greater_than {
                    stack.push(U256::from(0x1));
                } else {
                    stack.push(U256::from(0x0));
                }
            }
            SLT => {
                let mut b = stack.pop().unwrap_or_else(|| U256::from(0));
                let b_is_neg = U256::leading_zeros(&b) == 0;
                let mut a = stack.pop().unwrap_or_else(|| U256::from(0));
                let a_is_neg = U256::leading_zeros(&a) == 0;

                let mut is_less_than = b < a;
                if a_is_neg && b_is_neg {
                    (a, _) = U256::overflowing_add(!a, U256::from(1));
                    (b, _) = U256::overflowing_add(!b, U256::from(1));
                    is_less_than = b < a;
                }
                if a_is_neg && !b_is_neg {
                    is_less_than = false;
                }
                if !a_is_neg && b_is_neg {
                    is_less_than = true;
                }
                if is_less_than {
                    stack.push(U256::from(0x1));
                } else {
                    stack.push(U256::from(0x0));
                }
            }
            SGT => {
                let mut b = stack.pop().unwrap_or_else(|| U256::from(0));
                let b_is_neg = U256::leading_zeros(&b) == 0;
                let mut a = stack.pop().unwrap_or_else(|| U256::from(0));
                let a_is_neg = U256::leading_zeros(&a) == 0;

                let mut is_greater_than = b > a;
                if a_is_neg && b_is_neg {
                    (a, _) = U256::overflowing_add(!a, U256::from(1));
                    (b, _) = U256::overflowing_add(!b, U256::from(1));
                    is_greater_than = b > a;
                }
                if a_is_neg && !b_is_neg {
                    is_greater_than = true;
                }
                if !a_is_neg && b_is_neg {
                    is_greater_than = false;
                }
                if is_greater_than {
                    stack.push(U256::from(0x1));
                } else {
                    stack.push(U256::from(0x0));
                }
            }
            EQ => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let is_equal = b == a;
                if is_equal {
                    stack.push(U256::from(0x1));
                } else {
                    stack.push(U256::from(0x0));
                }
            }
            ISZERO => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                if a == U256::from(0) {
                    stack.push(U256::from(0x1));
                } else {
                    stack.push(U256::from(0));
                }
            }
            AND => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let and = a & b;
                stack.push(U256::from(and));
            }
            OR => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let result = a | b;
                stack.push(U256::from(result));
            }
            XOR => {
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let result = a ^ b;
                stack.push(U256::from(result));
            }
            NOT => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let result = !a;
                stack.push(U256::from(result));
            }
            BYTE => {
                let mut byte_offset = stack.pop().unwrap_or_else(|| U256::from(0));
                let value = stack.pop().unwrap_or_else(|| U256::from(0));
                if byte_offset > U256::from(31) {
                    stack.push(U256::from(0));
                } else {
                    byte_offset = U256::from(31) - byte_offset;
                    let byte = value.byte(U256::as_usize(&byte_offset));
                    stack.push(U256::from(byte));
                }
            }
            PC => {
                stack.push(U256::from(pc));
            }
            GAS => {
                stack.push(U256::max_value());
            }
            MSTORE => {
                let offset = stack.pop().unwrap_or_else(|| U256::from(0));
                let value = stack.pop().unwrap_or_else(|| U256::from(0));
                memory.store(offset.as_usize(), value);
            }
            MSTORE8 => {
                let offset = stack.pop().unwrap_or_else(|| U256::from(0));
                let value = stack.pop().unwrap_or_else(|| U256::from(0));
                let byte = value.byte(0);
                memory.store_byte(offset.as_usize(), byte);
            }
            MSIZE => {
                let size = memory.get_size();
                stack.push(size);
            }
            MLOAD => {
                let offset = stack.pop().unwrap_or_else(|| U256::from(0));
                let value = memory.load(offset.as_usize());
                stack.push(value);
            }
            SHA3 => {
                let offset = stack.pop().unwrap_or_else(|| U256::from(0));
                let num_bytes = stack.pop().unwrap_or_else(|| U256::from(0)).as_usize();
                let value = memory.load(offset.as_usize());
                let mut bytes: Vec<u8> = Vec::new();
                for i in 0..num_bytes {
                    let byte = value.byte(31 - i);
                    bytes.push(byte);
                }
                let mut hasher = Keccak256::new();
                hasher.update(bytes);
                let result = hasher.finalize();
                let foo = hex::encode(result);
                let hash = U256::from_str(&foo);
                stack.push(hash.unwrap());
            }
            ADDRESS => {}
            _ => {
                println!("unsupported instruction!");
            }
        }

        // advance program counter
        pc += 1;
    }
    return stack.into_iter().rev().collect();
}

fn push_n_bytes(n: usize, stack: &mut Vec<U256>, bytes: &[u8]) -> usize {
    let number = U256::from_big_endian(bytes);
    stack.push(number);
    return n;
}
