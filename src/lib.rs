#![feature(rustc_private)]
// #![feature(integer_atomics)]
#![feature(box_patterns)]
#![feature(core_intrinsics)]
// #![feature(generator_trait)]
#![feature(associated_type_defaults)]
#![feature(exclusive_range_pattern)]
#![feature(trait_alias)]
#![feature(specialization)]
#[warn(unreachable_patterns)]
#[allow(
    dead_code,
    unused_variables,
    non_snake_case,
    unused_parens,
    unused_assignments,
    unused_unsafe,
    unused_imports
)]
pub mod ordmap;

#[macro_use]
pub mod sbtree;

pub mod asbtree;
