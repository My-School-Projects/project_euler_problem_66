fn main()
{
    println!("{}, {}", is_perfect_square(25), is_perfect_square(26));
}

///
/// Evaluates true if n is a perfect square.
///
fn is_perfect_square(n: i64) -> bool
{
    // If n is a perfect square, then its square root will be an integer.
    // Take the square root
    let root_n = (n as f64).sqrt();
    // Round root_n to the nearest integer value
    let rounded = root_n.round();
    // If rounded == root_n, then root_n is an integer.
    rounded == root_n
}
