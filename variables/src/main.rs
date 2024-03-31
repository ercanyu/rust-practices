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

    println!("the value of y is: {y}")
}
