fn main() {
    let mut x = 5;
    println!("x is {x}");
    x = 6;
    println!("x is {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

    let tup = (1, 4.2, true, 'A');

    let (number, floating, boolean, character) = tup;

    let a: [u32; 5] = [1, 2, 3, 4, 5];

    function_test();
}

fn function_test() {
    println!("whoa a different function");
}

// 4 scalar (single unit) types : booleans, characters, integers and floating points
// chars are single quotes, strings are double quotes
// compound types combine other types together
// tuples are fixed length collections of variables that can be different types
// arrays are fixed length, but must be one type (vectors are the same but can change size)
