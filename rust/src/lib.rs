use primitive_types::U256;

const STOP: u8 = 0;
const ADD: u8 = 1;
const MUL: u8 = 2;
const SUB: u8 = 3;
const DIV: u8 = 4;
const SDIV: u8 = 5;
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
                // inserts at index 0
                stack.insert(0, U256::from(c[pc]));
            }
            PUSH32 => {
                pc += 1;
                let slice = &c[pc..pc + 32];
                println!("slice is {:?}: ", slice);
                let number = U256::from_big_endian(slice);
                println!("number is {:?}: ", number);
                stack.insert(0, number);
            }
            POP => {
                // removes index 0
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
                let a = stack.pop().unwrap_or_else(|| U256::from(0));
                println!("sdiv a {:?}", a);
                let b = stack.pop().unwrap_or_else(|| U256::from(0));
                println!("sdiv b {:?}", b);
                if a == U256::from(0) {
                    stack.insert(0, a);
                } else {
                    // fmt.
                    let quotient = b / a;
                    stack.insert(0, quotient);
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
