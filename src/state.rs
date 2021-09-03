// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
use seed_hooks::*;
use seed_hooks::atom::Atom;

#[atom]
pub fn global_count() -> Atom<i32> {
    0
}