fn main() {
    println!("Hello, world!");
}

// Tried to understand "procedural macros", here's the points:
// 1. Procedural macros are not like runtime functions.
// 2. They’re compile-time code transformers.
//
// 3. They:
//
// -Run before the compiler’s semantic phases,
//
// -Only see and emit syntax trees, not types,
//
// -Must generate valid Rust source,
//
// -Can easily create type errors or syntax errors if misused.
