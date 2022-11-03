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
const POP: u8 = 80;
const PUSH1: u8 = 96;
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
        match instruction {
            PUSH1 => {
                pc += 1;
                stack.insert(0, U256::from(c[pc]));
            }
            PUSH32 => {
                pc += 1;
                let slice = &c[pc..pc + 32];
                // println!("slice is {:?}: ", slice);
                let number = U256::from_big_endian(slice);
                // println!("number is {:?}: ", number);
                stack.insert(0, number);
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
            _ => {
                println!("unsupported instruction!");
            }
        }

        // advance program counter
        pc += 1;
    }
    return stack;
}
