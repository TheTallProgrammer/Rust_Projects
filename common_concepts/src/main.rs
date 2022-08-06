// Functions
// -------------------------------------------------------------
fn printFunc(place: &str){
    println!("{place} is a cool place!");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. 
fn plus_one(x: i32) -> i32 {
    x + 1
}
// -------------------------------------------------------------

// Main Function
fn main() {
    // Immutable
    let x: i32 = 5;
    println!("The value of x is {x}");

    // Mutable
    let mut y: i32 = 10;
    println!("The value of y is {y}");

    // Shadow
    let mut b = 10;
    b = 20;
    println!("The value of b is {b}");

    // Tuple
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tup;
    println!("The value of y is {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array {:?}", a);

    // Function Calls
    let state = "Texas";
    printFunc(&state);
    print_labeled_measurement(5, 'h');
    let sum: i32 = plus_one(5);

    // Expressions
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    // If statements
    let number = 3;
    if number != 0 {
        println!("number was something other than zero");
    }

    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // Nested Loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
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
    println!("End count = {count}");

    // For loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
