fn main() {
    println!("hello, it's a me, the main function!");

    another_function();
    function_with_parameters(5);
    print_labeled_measurement('h', 10);

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
