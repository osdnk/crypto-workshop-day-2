
#![feature(generic_const_exprs)]
use ring_arith::cyclotomic_ring::*;

fn main() {
    const N: usize = 128;
    const MOD_Q: u64 = 4546383823830515713; // Example modulus
    // const MOD_Q: u64 = 257; // Example modulus
    let mut operand1 = CyclotomicRing::<MOD_Q, N>::random();
    let mut operand2 = CyclotomicRing::<MOD_Q, N>::random();

    let expected_result = naive_multiply(&mut operand1, &mut operand2);

    let mut result = ntt_multiplication::<MOD_Q, N>(&mut operand1, &mut operand2);

    assert_eq!(result.data, expected_result.data);
    println!("Hello, world!");
}
