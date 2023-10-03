use std::ops::Add;

/// # The Σ sum of a function f from a to b.
/// ### a is the base number X=from
/// ### b is the top number
/// ### f is the function to be applied to each number
/// see https://en.wikipedia.org/wiki/Summation
pub fn sum<F>(a: usize, b: usize, f: fn(usize) -> F) -> F
    where
        F: Default + Add<Output = F> + Copy,
{
    (a..=b).map(f).fold(F::default(), Add::add)
}