// Allow the use of the n...m syntax
// n...m is an inclusive range (n through m)
// whereas n..m is n through m-1
#![feature(inclusive_range_syntax)]

fn main()
{
    let target_d_value = 50;

    let mut max_x = 0 as i64;
    let mut d_with_largest_x = 0;

    for d in 0...target_d_value {
        // We can only find x if d is not a perfect square
        if !is_perfect_square(d) {
            let x = find_minimal_x(d);

            if x > max_x {
                max_x = x;
                d_with_largest_x = d;
            }
        }
    }

    println!("The value of D ≤ {} in minimal solutions of x for which the largest value of x is obtained is: {}", target_d_value, d_with_largest_x);
}

///
/// Returns the smallest x for which x^2 - Dy^2 = 1,
/// where D is given and y is a positive integer.
///
/// This function will block indefinitely if D is a perfect square.
///
fn find_minimal_x(d: i64) -> i64
{
    // We are brute force searching for the y that will give us the smallest x.
    // Smaller y values will give us smaller x values, so we will start at 1
    // and iterate as long as it takes to find an x value that is an integer.
    let mut y = 1 as i64;

    loop {
        // x = sqrt(Dy^2 + 1)
        let x = ((d * y * y + 1) as f64).sqrt();
        // If x is an integer, then we're done, and we will return x.
        // Otherwise, we will increment y and continue searching.
        if is_an_integer(x) {
            return x as i64;
        } else {
            y = y + 1;
        }
    }
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
