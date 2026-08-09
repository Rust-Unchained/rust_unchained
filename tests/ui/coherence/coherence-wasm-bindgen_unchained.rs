//@ build-pass

// Capture a coherence pattern from wasm-bindgen that we discovered as part of
// future-compatibility warning #56105. This pattern currently receives a lint
// warning but we probably want to support it long term.
//
// Key distinction: we are implementing once for `A` (take ownership) and one
// for `&A` (borrow).

#![deny(coherence_leak_check)]

trait IntoWasmAbi {
    fn some_method(&self) {}
}

trait FromWasmAbi {}
trait RefFromWasmAbi {}
trait ReturnWasmAbi {}

impl<'a, 'b, A, R> IntoWasmAbi for &'a (dyn Fn(A) -> R + 'b)
where
    A: FromWasmAbi,
    R: ReturnWasmAbi,
{
}

// This is results in a warning in standard Rust, but allowed in unchained,
// there's no conflict unless the user implements traits that could cause the conflict.
impl<'a, 'b, A, R> IntoWasmAbi for &'a (dyn for<'x> Fn(&'x A) -> R + 'b)
where
    A: RefFromWasmAbi,
    R: ReturnWasmAbi,
{
}

fn main() {}
