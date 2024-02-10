
//! # Generic Data Structures
//! The NDArray module allows for multidimensional array creation.
//! The module allows common operations like adding, subtracting and multiplying
//! multi rank values. The underlying representation is a contigous array using
//! strides to move to the next dimension.
pub mod ops;
pub mod ndarray;