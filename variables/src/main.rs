const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // constants
    println!("the value of const THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // mutable variable
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("the value of y in the inner scope is: {y}");
    }

    println!("the value of y is: {y}");

    // integer types
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    let integer_with_type_suffix = 42u32;

    println!("decimal: {decimal}, hex: {hex}, octal: {octal}");
    println!(
        "binary: {binary}, byte: {byte}, integer_with_type_suffix: {integer_with_type_suffix}"
    );

    // floating point types
    let f64 = 2.0;
    let f32: f32 = 3.0;

    println!("f64: {f64}, f32: {f32}");

    // numeric operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("sum: {sum}, difference: {difference}, product: {product}");
    println!("quotient: {quotient}, truncated: {truncated}, remainder: {remainder}");
}
