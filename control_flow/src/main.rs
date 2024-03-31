fn main() {
    println!("Hello, world!");

    if_else_expressions();
    using_if_in_a_let_statement();
    returning_value_from_loops();
    loop_labels();
    conditional_loops_with_while();
    loop_through_a_collection_with_for();
}

fn loop_through_a_collection_with_for() {
    let array = [10, 20, 30, 40, 50];

    for element in array {
        println!("the value is: {}", element);
    }

    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("let's goooooo!");
}

fn conditional_loops_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("let's goooooo!");
}

fn loop_labels() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);

            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("end count = {count}");
}

fn returning_value_from_loops() {
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn using_if_in_a_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn if_else_expressions() {
    let number = 72;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was different than 0");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2, 3 or 4");
    }
}
