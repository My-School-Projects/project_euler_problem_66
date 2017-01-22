fn main()
{
    println!("{}, {}", is_perfect_square(25), is_perfect_square(26));
}

///
/// Evaluates true if n is a perfect square.
///
fn is_perfect_square(n: i64) -> bool
{
    // Take the square root of n
    let root_n = (n as f64).sqrt();
    // If the square root of n is an integer, then n is a perfect square.
    is_an_integer(root_n)
}

///
/// Evaluates true if n is an integer.
///
fn is_an_integer(n: f64) -> bool
{
    // Round n to the nearest integer value
    let rounded = n.round();
    // If n and its rounded counterpart are equal, then n is an integer.
    n == rounded
}
