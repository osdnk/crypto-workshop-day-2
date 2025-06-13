use crate::hexl::bindings::{eltwise_mult_mod, ntt_forward_in_place, ntt_inverse_in_place};
use rand::Rng;


#[derive(Clone, Debug)]
pub struct CyclotomicRing<const MOD_Q: u64, const N: usize>
{
    pub data: [u64; N],
}

impl<const MOD_Q: u64, const N: usize> CyclotomicRing<MOD_Q, N>
{
    pub fn new() -> Self {
        Self { data: [0u64; N] }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        let mut data = [0u64; N];
        for i in 0..N {
            data[i] = rng.random_range(0..MOD_Q);
        }
        Self { data }
    }


    pub fn to_ntt_representation(&mut self) {
        unsafe { ntt_forward_in_place(self.data.as_mut_ptr(), self.data.len(), MOD_Q) };
    }


    pub fn to_coeff_representation(&mut self) {
        unsafe { ntt_inverse_in_place(self.data.as_mut_ptr(), self.data.len(), MOD_Q) };
    }
    
}

pub fn ntt_multiplication<const MOD_Q: u64, const N: usize>(
    operand1: &mut CyclotomicRing<MOD_Q, N>,
    operand2: &mut CyclotomicRing<MOD_Q, N>,
) -> CyclotomicRing<MOD_Q, N>
{
    operand1.to_ntt_representation();
    operand2.to_ntt_representation();

    let mut result = CyclotomicRing::<MOD_Q, N>::new();

    unsafe {
        eltwise_mult_mod(
            result.data.as_mut_ptr(),
            operand1.data.as_ptr(),
            operand2.data.as_ptr(),
            result.data.len() as u64,
            MOD_Q,
        ) 
    };

    operand1.to_coeff_representation();
    operand2.to_coeff_representation();
    result.to_coeff_representation();    
    result
}

pub fn naive_multiply<const MOD_Q: u64, const N: usize>(
    operand1: &mut CyclotomicRing<MOD_Q, N>,
    operand2: &mut CyclotomicRing<MOD_Q, N>,
) -> CyclotomicRing<MOD_Q, N>
{
    let mut result = CyclotomicRing::<MOD_Q, N>::new();
    for i in 0..N {
        for j in 0..N {
            if i + j < N {
                result.data[i + j] =
                    (result.data[i + j] + 
                        ((operand1.data[i] as u128 * operand2.data[j] as u128) % MOD_Q as u128) as u64) % MOD_Q;
            } else {
                result.data[i + j - N] = 
                    (result.data[i + j - N] + MOD_Q -
                        ((operand1.data[i] as u128 * operand2.data[j] as u128) % MOD_Q as u128) as u64 ) % MOD_Q;
            }
        }
    }
    result
}
