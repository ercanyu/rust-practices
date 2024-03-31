fn main() {
    println!("hello, it's a me, the main function!");

    another_function();
    function_with_parameters(5);
    print_labeled_measurement('h', 10);

    expressions();
    functions_with_return_values();
}

fn functions_with_return_values() {
    let x = five();
    println!("The value returned from five() is: {x}");

    let y = plus_one(12);
    println!("The value returned from plus_one(12) is: {y}");

    let z = plus_one_with_return_keyword(12);
    println!("The value returned from plus_one_with_return_keyword(12) is: {z}");
}

fn plus_one_with_return_keyword(x: i32) -> i32 {
    return x + 1;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn five() -> i32 {
    5
}

fn expressions() {
    /*
    expressions do not include ending semicolons,
    if you add a semicolon to the end of an expression,
    you turn it into a statement, and it will then not return a value
    */
    let expression = {
        let x = 5;
        x + 1
    };

    println!("The value of the expression is: {expression}");
}

fn another_function() {
    println!("just another function.");
}

fn function_with_parameters(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(label: char, measurement: i32) {
    println!("labeled measurement => {measurement}{label}");
}
