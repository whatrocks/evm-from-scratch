use evm::evm;
use primitive_types::U256;
use serde::Deserialize;
use serde_json::json;

use evm::Tx;

#[derive(Debug, Deserialize)]
struct Evmtest {
    name: String,
    code: Code,
    tx: Option<Tx>,
    state: Option<serde_json::Value>,
    expect: Expect,
}

#[derive(Debug, Deserialize)]
struct Code {
    asm: String,
    bin: String,
}

#[derive(Debug, Deserialize)]
struct Expect {
    stack: Option<Vec<String>>,
    success: Option<bool>,
    #[serde(rename = "return")]
    ret: Option<String>,
}

// This parses an integer based off its prefix: 0x - base16, otherwise base10.
fn parseUIntRadix(number: &String) -> Option<U256> {
    let radix = if number.starts_with("0x") { 16 } else { 10 };
    if let Ok(result) = U256::from_str_radix(number, radix) {
        Some(result)
    } else {
        None
    }
}

fn main() {
    let text = std::fs::read_to_string("../evm.json").unwrap();
    let data: Vec<Evmtest> = serde_json::from_str(&text).unwrap();

    let total = data.len();

    for (index, test) in data.iter().enumerate() {
        println!("Test {} of {}: {}", index + 1, total, test.name);

        let code: Vec<u8> = hex::decode(&test.code.bin).unwrap();

        let mut my_tx = Tx {
            to: Some("123".to_string()),
            from: Some("456".to_string()),
        };

        if test.tx.is_some() {
            let transaction = test.tx.as_ref().unwrap();
            if transaction.to.is_some() {
                my_tx.to = transaction.to.clone();
            }
            if transaction.from.is_some() {
                my_tx.from = transaction.from.clone();
            }
        }

        let mut state = json!({});

        if test.state.is_some() {
            let test_state = test.state.as_ref().unwrap();
            state = test_state.clone();
        }

        let actual_stack = evm(&code, my_tx, state);

        let mut expected_stack: Vec<U256> = Vec::new();
        if let Some(ref stacks) = test.expect.stack {
            for stack in stacks {
                expected_stack.push(parseUIntRadix(stack).unwrap());
            }
        }

        let mut matching = actual_stack.len() == expected_stack.len();
        if matching {
            for i in 0..actual_stack.len() {
                if actual_stack[i] != expected_stack[i] {
                    matching = false;
                    break;
                }
            }
        }

        if !matching {
            println!("Instructions: \n{}\n", test.code.asm);

            println!("Expected: [");
            for v in expected_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Got: [");
            for v in actual_stack {
                println!("  {:#X},", v);
            }
            println!("]\n");

            println!("Progress: {}/{}\n\n", index, total);
            panic!("Stack mismatch");
        }
        println!("PASS");
    }
    println!("Congratulations!");
}
