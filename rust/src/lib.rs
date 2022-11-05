use primitive_types::U256;

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
const POP: u8 = 80;
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

pub fn evm(code: impl AsRef<[u8]>) -> Vec<U256> {
    // convert instructions
    let c = code.as_ref();
    println!("{:?}", c);

    // init result
    let mut stack = Vec::new();

    // init program counter
    let mut pc = 0;

    // process instructions
    while pc < c.len() {
        let instruction = c[pc];
        println!("instruction: {:?}", instruction);
        match instruction {
            PUSH1 => {
                pc += 1;
                stack.insert(0, U256::from(c[pc]));
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
            POP => {
                stack.remove(0);
            }
            STOP => {
                break;
            }
            ADD => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let (sum, _) = U256::overflowing_add(b, a);
                stack.insert(0, sum);
            }
            MUL => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let (product, _) = U256::overflowing_mul(b, a);
                stack.insert(0, product);
            }
            SUB => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let (diff, _) = U256::overflowing_sub(b, a);
                stack.insert(0, diff);
            }

            DIV => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                if a == U256::from(0) {
                    stack.insert(0, a);
                } else {
                    let quotient = b / a;
                    stack.insert(0, quotient);
                }
            }
            SDIV => {
                // signed integer division
                let mut a = stack.pop().unwrap_or_else(|| U256::from(0));
                let a_is_neg = U256::leading_zeros(&a) == 0;
                if a_is_neg {
                    (a, _) = U256::overflowing_add(!a, U256::from(1));
                }

                let mut b = stack.pop().unwrap_or_else(|| U256::from(0));
                let b_is_neg = U256::leading_zeros(&b) == 0;
                if b_is_neg {
                    (b, _) = U256::overflowing_add(!b, U256::from(1));
                }

                let mut quotient = b / a;

                if (a_is_neg && !b_is_neg) || (!a_is_neg && b_is_neg) {
                    (quotient, _) = U256::overflowing_add(!quotient, U256::from(1));
                }
                stack.insert(0, quotient);
            }
            MOD => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                if a == U256::from(0) {
                    stack.insert(0, a);
                } else {
                    let modulus = b % a;
                    stack.insert(0, modulus);
                }
            }
            SMOD => {
                let mut a = stack.pop().unwrap_or_else(|| U256::from(0));
                let a_is_neg = U256::leading_zeros(&a) == 0;
                if a_is_neg {
                    (a, _) = U256::overflowing_add(!a, U256::from(1));
                }

                let mut b = stack.pop().unwrap_or_else(|| U256::from(0));
                let b_is_neg = U256::leading_zeros(&b) == 0;
                if b_is_neg {
                    (b, _) = U256::overflowing_add(!b, U256::from(1));
                }
                if a == U256::from(0) {
                    stack.insert(0, a);
                    break;
                }
                let mut modulus = b % a;
                if a_is_neg {
                    (modulus, _) = U256::overflowing_add(!modulus, U256::from(1));
                }
                stack.insert(0, modulus);
            }
            LT => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let is_less_than = b < a;
                if is_less_than {
                    stack.insert(0, U256::from(0x1));
                } else {
                    stack.insert(0, U256::from(0x0));
                }
            }
            GT => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let is_greater_than = b > a;
                if is_greater_than {
                    stack.insert(0, U256::from(0x1));
                } else {
                    stack.insert(0, U256::from(0x0));
                }
            }
            SLT => {
                let mut a = stack.pop().unwrap_or_else(|| U256::from(0));
                let a_is_neg = U256::leading_zeros(&a) == 0;
                let mut b = stack.pop().unwrap_or_else(|| U256::from(0));
                let b_is_neg = U256::leading_zeros(&b) == 0;

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
                    stack.insert(0, U256::from(0x1));
                } else {
                    stack.insert(0, U256::from(0x0));
                }
            }
            SGT => {
                let mut a = stack.pop().unwrap_or_else(|| U256::from(0));
                let a_is_neg = U256::leading_zeros(&a) == 0;
                let mut b = stack.pop().unwrap_or_else(|| U256::from(0));
                let b_is_neg = U256::leading_zeros(&b) == 0;

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
                    stack.insert(0, U256::from(0x1));
                } else {
                    stack.insert(0, U256::from(0x0));
                }
            }
            EQ => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let is_equal = b == a;
                if is_equal {
                    stack.insert(0, U256::from(0x1));
                } else {
                    stack.insert(0, U256::from(0x0));
                }
            }
            ISZERO => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                if a == U256::from(0) {
                    stack.insert(0, U256::from(0x1));
                } else {
                    stack.insert(0, U256::from(0));
                }
            }
            AND => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let and = a & b;
                stack.insert(0, U256::from(and));
            }
            OR => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let result = a | b;
                stack.insert(0, U256::from(result));
            }
            XOR => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                let result = a ^ b;
                stack.insert(0, U256::from(result));
            }
            NOT => {
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                let result = !a;
                stack.insert(0, U256::from(result));
            }
            BYTE => {
                let value = stack.pop().unwrap_or_else(|| U256::from(0));
                let mut byte_offset = stack.pop().unwrap_or_else(|| U256::from(0));
                if byte_offset > U256::from(31) {
                    stack.insert(0, U256::from(0));
                } else {
                    byte_offset = U256::from(31) - byte_offset;
                    let byte = value.byte(U256::as_usize(&byte_offset));
                    stack.insert(0, U256::from(byte));
                }
            }
            _ => {
                println!("unsupported instruction!");
            }
        }

        // advance program counter
        pc += 1;
    }
    return stack;
}

fn push_n_bytes(n: usize, stack: &mut Vec<U256>, bytes: &[u8]) -> usize {
    let number = U256::from_big_endian(bytes);
    stack.insert(0, number);
    return n;
}
