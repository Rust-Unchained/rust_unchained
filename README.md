# Rust Unchained

This is a fork of the official [Rust](https://github.com/rust-lang) compiler.

This README only lists the differences from the official one. For more information, see the official [README](https://github.com/rust-lang/rust).

# Introduction

The goal of this fork is to increase the productivity of application developers. 
We attempt to achieve this by disabling the orphan rules (many of which do not make sense when applied to binary crates).

There are ways to work around the orphan rules, but those are just workarounds, they waste development time.

# Differences from the official compiler

## Orphan rules are disabled
Which means you can implement foreign traits for foreign types.

Example:
```rust
use foreign::ForeignType;
use serde::Serialize;

impl Serialize for ForeignType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        todo!()
    }
}

impl i32 {
    fn display_percentage(self) -> String {
        format!("{self}%")
    }
}
```

Note that the compiler still checks for conflicts, and will not compile if there are any.

Expect conflicts to be much less frequent though, as the default compiler misleadingly 
issues the "conflicting implementations" error when you're violating the orphan rules
(despite no actual conflicts existing at present).

There is one exception, which may seem like an orphan rule but isn't:
- You cannot implement cross-crate traits on foreign types (such as `Send`, `Sync`, `Drop`, etc.)

If you're wondering how this was achieved, it was simpler than it may look. 
The official compiler is already capable of detecting conflicting impls,
the orphan rules are checks added merely to enforce a design philosophy (note how `std` violates the rules in a lot of places).
If you check Unchained's commits, you'll notice that most of them don't add any code, just removes it.

## Inherent impls on foreign types are allowed
Which means you can write extension methods without having to use extension traits.

Example:
```rust
impl<T: Display> Vec<T> {
    fn join_str(&self, sep: &str) -> String {
	    self.iter()
		    .map(T::to_string)
		    .intersperse(sep.to_string())
		    .collect()
    }
}
```

## `impl<T: Copy> Copy for Range<T> {}`
See https://github.com/rust-lang/rust/issues/18045

# Installation

Right now, the only way to install this custom toolchain is to build it from source.

The author is working on a more convenient way to install it, but there's no ETA yet.
Speaking of which, if you are familiar with the setup, the author would very much appreciate your help.

Building from source is the same as building the official compiler, just clone this repository instead of the official one.
See [INSTALL.md](INSTALL.md).

# Known issues

Implementing operator traits (Add, Sub, Mul, Div, etc.) on fundamental types (i8, i32, usize, etc.) is allowed, 
but the compiler is not capable of detecting your implementations when using the operator symbols (+, -, *, etc.). 

Example:
```rust
// This compiles
impl Add<u16> for i32 {
    type Output = i32;
    
    fn add(self, rhs: u16) -> Self {
        self + rhs as i32
    }
}

// The problem comes when using the `+` operator symbol:
fn main() {
	let a: i32 = 5;
	let b: u16 = 10;

	let c = a + b; // Error, using the symbol doesn't work

	let c = a.add(b); // Ok, the compiler is capable of detecting the implementation when using the method.
}
```

## PS - 1
Beware of false positives when using external analysis tools, such as Rust Analyzer and RustRover, 
they are programmed to detect violations of the orphan rules and do not take into account the custom compiler. You may need to disable certain inspections in those tools.

If you think you saw a false positive, run `cargo check` to be sure.

Clippy works just fine since it's a wrapper around `cargo check`.

## PS - 2
Do not upload crates made with this compiler to crates.io, those crates won't compile on the official compiler -
only users with the Unchained toolchain will be able to use them.
