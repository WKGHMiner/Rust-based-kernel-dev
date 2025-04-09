#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(user::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate user;

const SIZE: usize = 10;
const P: u32 = 3;
const STEP: usize = 100000;
const MOD: u32 = 10007;

#[unsafe(no_mangle)]
fn main() -> i32 {
    debug!("Task 2.");
    info!("This application do some regular arithmatic problems.");
    info!("It should work fine.");

    let mut pow = [0u32; SIZE];
    let mut index: usize = 0;
    pow[index] = 1;
    for i in 1..=STEP {
        let last = pow[index];
        index = (index + 1) % SIZE;
        pow[index] = last * P % MOD;
        if i % 10000 == 0 {
            println!("{}^{}={}(MOD {})", P, i, pow[index], MOD);
        }
    }
    println!("Test power OK!");
    
    0
}