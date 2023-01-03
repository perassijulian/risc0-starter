#![no_main]
#![no_std]  // std support is experimental, but you can remove this to try it

risc0_zkvm_guest::entry!(main);
use risc0_zkvm_guest::env;

pub fn main() {
    // TODO: Implement your guest code here
    let a: u64 = env::read();
    let b: u64 = env::read();

    if a == 0 || b == 0 {
        panic!("Trivial solution!");
    }

    let result = a.checked_mul(b).expect("Multiplication overflow!");

    env::commit(&result)
}
