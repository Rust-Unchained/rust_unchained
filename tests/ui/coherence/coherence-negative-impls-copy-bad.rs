//@ build-pass

#![feature(negative_impls)]
#![crate_type = "lib"]

// FIXME: This is allowed in Rust Unchained, 
// but it should be tackled in the future.
// 
// Negative impls are problematic on combinations of fundamental types + fundamental traits. 

impl !Copy for str {}

impl !Copy for fn() {}

impl !Copy for () {}

