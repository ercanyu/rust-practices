fn main() {
    constants();
    mutable_variables();
    shadowing();
    integer_types();
    floating_point_types();
    numeric_operations();
    boolean_type();
    character_type();
    tuple_type();
    array_type();
}

fn array_type() {
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    println!("first element of array is: {first}");

    let months = [
        "january",
        "february",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];
    let third_month = months[2];
    println!("third month is: {third_month}");

    let array_with_type: [i32; 5] = [1, 2, 3, 4, 5];
    let array_with_type_and_initial_value = [3; 5];
    let first_element = array_with_type[0];
    let first_element_in_array_with_initial_value = array_with_type_and_initial_value[0];

    println!("first element in array_with_type is: {first_element}");
    println!("first element in array_with_type_and_initial_value is: {first_element_in_array_with_initial_value}");
}

fn tuple_type() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    println!("x in tuple: {x}, y in tuple: {y}, z in tuple: {z}");

    let five_hundred = tuple.0;
    let six_point_four = tuple.1;
    let one = tuple.2;

    println!("five_hundred: {five_hundred},six_point_four: {six_point_four}, one: {one}");

    let unit = ();
    let _ = unit;
    println!("a tuple without any values has a special name: unit");
}

fn character_type() {
    let char_z = 'z';
    let char_heart: char = '❤';
    let char_emoji_heart_eyed_cat = '😻';
    println!(
        "char_z: {char_z}, char_heart: {char_heart}, char_emoji_heart_eyed_cat: {char_emoji_heart_eyed_cat}"
    );
}

fn boolean_type() {
    let t = true;
    let f: bool = false;

    println!("t: {t}, f: {f}");
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
