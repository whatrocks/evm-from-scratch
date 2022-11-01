use primitive_types::U256;

const STOP: u8 = 0;
const ADD: u8 = 1;
const POP: u8 = 80;
const PUSH1: u8 = 96;

pub fn evm(code: impl AsRef<[u8]>) -> Vec<U256> {    
    let mut v = Vec::new();
    let c = code.as_ref();
    println!("{:?}", c);
    let mut i = 0;
    while i < c.len() {
        if c[i] == PUSH1 {
            i += 1;
            v.insert(0, U256::from(c[i]));   
        }
        if c[i] == POP {
            v.remove(0);
        }
        if c[i] == STOP {
            break;
        }
        if c[i] == ADD {
            let a = v.pop().unwrap_or_else(|| U256::from(0));
            let b = v.pop().unwrap_or_else(|| U256::from(0));
            v.insert(0, a+b);
        }
        i += 1;
    }
    return v;
}
