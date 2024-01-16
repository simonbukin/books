fn main() {
    println!("Hello, world!");
    fn_wow(5);
    expression_test(5, 10);
    println!("unsigned binteger!!!! {}", with_return_type())
}

fn fn_wow(x: u32) {
    println!("wow! it's a {x}!")
}

fn expression_test(x: u32, y: u32) {
    let y = {
        let z = x + 2;
        z
    };

    println!("huh {y}");
}

fn with_return_type() -> i32 {
    96
}

// statements perform an action and do no return a value
// expressions evaluate to a resulting value
// including a semicolon turns an expression into a statement... weird
