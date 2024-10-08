use crate::datatype::memory_vecf16::{Vecf16Input, Vecf16Output};
use crate::error::*;
use base::vector::*;
use std::num::NonZero;

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_add(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> Vecf16Output {
    check_matched_dims(lhs.dims(), rhs.dims());
    Vecf16Output::new(
        lhs.as_borrowed()
            .operator_add(rhs.as_borrowed())
            .as_borrowed(),
    )
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_sub(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> Vecf16Output {
    check_matched_dims(lhs.dims(), rhs.dims());
    Vecf16Output::new(
        lhs.as_borrowed()
            .operator_sub(rhs.as_borrowed())
            .as_borrowed(),
    )
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_mul(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> Vecf16Output {
    check_matched_dims(lhs.dims(), rhs.dims());
    Vecf16Output::new(
        lhs.as_borrowed()
            .operator_mul(rhs.as_borrowed())
            .as_borrowed(),
    )
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_lt(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> bool {
    check_matched_dims(lhs.dims(), rhs.dims());
    lhs.as_borrowed() < rhs.as_borrowed()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_lte(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> bool {
    check_matched_dims(lhs.dims(), rhs.dims());
    lhs.as_borrowed() <= rhs.as_borrowed()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_gt(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> bool {
    check_matched_dims(lhs.dims(), rhs.dims());
    lhs.as_borrowed() > rhs.as_borrowed()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_gte(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> bool {
    check_matched_dims(lhs.dims(), rhs.dims());
    lhs.as_borrowed() >= rhs.as_borrowed()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_eq(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> bool {
    check_matched_dims(lhs.dims(), rhs.dims());
    lhs.as_borrowed() == rhs.as_borrowed()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_neq(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> bool {
    check_matched_dims(lhs.dims(), rhs.dims());
    lhs.as_borrowed() != rhs.as_borrowed()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_dot(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> f32 {
    check_matched_dims(lhs.dims(), rhs.dims());
    VectBorrowed::operator_dot(lhs.as_borrowed(), rhs.as_borrowed()).to_f32()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_l2(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> f32 {
    check_matched_dims(lhs.dims(), rhs.dims());
    VectBorrowed::operator_l2(lhs.as_borrowed(), rhs.as_borrowed()).to_f32()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_operator_cos(lhs: Vecf16Input<'_>, rhs: Vecf16Input<'_>) -> f32 {
    check_matched_dims(lhs.dims(), rhs.dims());
    VectBorrowed::operator_cos(lhs.as_borrowed(), rhs.as_borrowed()).to_f32()
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_sphere_dot_in(
    lhs: Vecf16Input<'_>,
    rhs: pgrx::composite_type!("sphere_vecf16"),
) -> bool {
    let center: Vecf16Output = match rhs.get_by_index(NonZero::new(1).unwrap()) {
        Ok(Some(s)) => s,
        Ok(None) => pgrx::error!("Bad input: empty center at sphere"),
        Err(_) => unreachable!(),
    };
    check_matched_dims(lhs.dims(), center.dims());
    let radius: f32 = match rhs.get_by_index(NonZero::new(2).unwrap()) {
        Ok(Some(s)) => s,
        Ok(None) => pgrx::error!("Bad input: empty radius at sphere"),
        Err(_) => unreachable!(),
    };
    VectBorrowed::operator_dot(lhs.as_borrowed(), center.as_borrowed()).to_f32() < radius
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_sphere_l2_in(
    lhs: Vecf16Input<'_>,
    rhs: pgrx::composite_type!("sphere_vecf16"),
) -> bool {
    let center: Vecf16Output = match rhs.get_by_index(NonZero::new(1).unwrap()) {
        Ok(Some(s)) => s,
        Ok(None) => pgrx::error!("Bad input: empty center at sphere"),
        Err(_) => unreachable!(),
    };
    check_matched_dims(lhs.dims(), center.dims());
    let radius: f32 = match rhs.get_by_index(NonZero::new(2).unwrap()) {
        Ok(Some(s)) => s,
        Ok(None) => pgrx::error!("Bad input: empty radius at sphere"),
        Err(_) => unreachable!(),
    };
    VectBorrowed::operator_l2(lhs.as_borrowed(), center.as_borrowed()).to_f32() < radius
}

#[pgrx::pg_extern(immutable, strict, parallel_safe)]
fn _vectors_vecf16_sphere_cos_in(
    lhs: Vecf16Input<'_>,
    rhs: pgrx::composite_type!("sphere_vecf16"),
) -> bool {
    let center: Vecf16Output = match rhs.get_by_index(NonZero::new(1).unwrap()) {
        Ok(Some(s)) => s,
        Ok(None) => pgrx::error!("Bad input: empty center at sphere"),
        Err(_) => unreachable!(),
    };
    check_matched_dims(lhs.dims(), center.dims());
    let radius: f32 = match rhs.get_by_index(NonZero::new(2).unwrap()) {
        Ok(Some(s)) => s,
        Ok(None) => pgrx::error!("Bad input: empty radius at sphere"),
        Err(_) => unreachable!(),
    };
    VectBorrowed::operator_cos(lhs.as_borrowed(), center.as_borrowed()).to_f32() < radius
}
