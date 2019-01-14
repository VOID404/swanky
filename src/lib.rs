#![deny(clippy::all)]
#![allow(
    clippy::cast_lossless,
    clippy::new_without_default,
    clippy::new_without_default_derive,
    clippy::block_in_if_condition_stmt,
    clippy::map_entry,
    clippy::needless_range_loop
)]

#![allow(non_snake_case)]

mod aes;
pub mod garble;
pub mod wire;
pub mod fancy;
pub mod dummy;
pub mod informer;
pub mod circuit;
pub mod util;
