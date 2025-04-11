//@ build-pass

// Make sure that an invalid inherent impl doesn't totally clobber all of the
// other inherent impls, which lead to mysterious method/assoc-item probing errors.

// Allowed in Rust Unchained, no conflicts here.
impl () {}

struct W;
impl W {
    const CONST: u32 = 0;
}

fn main() {
    let _ = W::CONST;
}
