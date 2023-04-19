use crate::halo2_proofs::halo2curves::bn256::{Fq, Fq12, Fq2};
use crate::{
    bigint::CRTInteger,
    fields::{fp, fp12, fp2, FieldExtPoint},
};

pub mod final_exp;
pub mod pairing;

pub type FpChip<F> = fp::FpConfig<F, Fq>;
pub type FpPoint<'v, F> = CRTInteger<'v, F>;
pub type FqPoint<'v, F> = FieldExtPoint<FpPoint<'v, F>>;
pub type Fp2Chip<'a, F> = fp2::Fp2Chip<'a, F, FpChip<F>, Fq2>;
pub type Fp12Chip<'a, F> = fp12::Fp12Chip<'a, F, FpChip<F>, Fq12, 9>;

#[cfg(test)]
pub(crate) mod tests;
