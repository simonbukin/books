fn main() {
    println!("Hello, world!");

    let number = 3;

    if number < 5 {
        println!("yooo it is less than 5");
    } else {
        println!("damn it's bigger")
    }

    let number = if 5 > 6 { 3 } else { 4 };
    println!("number is: {number}");

    let mut counter = 0;

    counter = loop {
        counter += 1;
        if counter > 5 {
            break counter * 100;
        }
    };

    println!("counter: {counter}");

    println!("five is : {}", the_number_five());

    cool_for_loop();

    let fib_one = fibonacci(100);
    println!("the fibonacci number is {fib_one}");
}

fn the_number_five() -> i32 {
    5
}

fn cool_for_loop() -> () {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for elem in a {
        println!("elem: {elem}")
    }
}

fn fibonacci(times: u128) -> u128 {
    let mut first = 0;
    let mut second = 1;

    if times == 1 {
        return first;
    }
    if times == 2 {
        return second;
    }

    let mut i = 2;
    while i < times {
        let temp = second;
        second = first + second;
        first = temp;
        i += 1;
    }
    second
}
