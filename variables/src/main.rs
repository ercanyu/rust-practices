fn main() {
    constants();
    mutable_variables();
    shadowing();
    integer_types();
    floating_point_types();
    numeric_operations();
}

fn numeric_operations() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("sum: {sum}, difference: {difference}, product: {product}");
    println!("quotient: {quotient}, truncated: {truncated}, remainder: {remainder}");
}

fn floating_point_types() {
    let f64 = 2.0;
    let f32: f32 = 3.0;

    println!("f64: {f64}, f32: {f32}");
}

fn integer_types() {
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
}

fn shadowing() {
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("the value of y in the inner scope is: {y}");
    }

    println!("the value of y is: {y}");
}

fn mutable_variables() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
}

fn constants() {
    const MAX_POINTS: u32 = 100_000;
    println!("the value of const MAX_POINTS is: {MAX_POINTS}");
}
