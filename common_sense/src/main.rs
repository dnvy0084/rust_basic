const BASE_NUM: u32 = 10; // constant expression 

fn test_shadow() -> u32 {
    let a = 1;

    let a = a + a;

    let a = a * a;

    return a;
}

fn test_spaces() -> usize {
    let spaces = "   ";
    let spaces= spaces.len();

    return spaces;
}

/**
 * error occur.
 * */
fn test_mut_spaces() -> usize {
    let mut spaces = "   ";
    // spaces = spaces.len();

    // return spaces;
    return spaces.len();
}

fn test_float() -> f32 {
    let num = 1.5;
    let num: f32 = num + num;

    return num * num;
}

fn test_bool() -> bool {
    let b: bool = false;

    return !b;
}

fn test_char() -> char {
    let c = 'c';

    return c;
}

fn test_tup() -> (i32, f64, u8) {
    let tup = (500, 6.4, 1);

    return tup;
}

fn test_arr() -> [&'static str; 12] {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dev"];

    return months;
}

/**
 * static borrowed value
 * */
fn test() -> &'static str {
    let t = "test";

    return t;
}

fn log(a: i32, b: i32) {
    let base = 10;
    let c: i32 = {
        a + b + base
    };

    println!("base({}) + {} + {} = {}", base, a, b, c); 
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn isEven(n: i32) {
    if n % 2 == 0 {
        println!("this number({}) is even.", n);
    } else {
        println!("this number({}) is odd.", n);
    }
}

fn isEven2(n: i32) {
    match n % 2 == 0 {
        true => println!("{} is EVEN", n),
        false => println!("{} is ODD", n),
    }
}

fn match4(n: i32) {
    if n < 4 {
        println!("{} is small then 4", n);
        return;
    }

    match n % 4 {
        0 => println!("{} is divisible by 4", n),
        1 => println!("{} is divisible by 3", n),
        2 => println!("{} is divisible by 2", n),
        _ => println!("{} is not divisible", n),
    }
}

fn test_let_in_if() {
    let b = true;

    let number = if b {
        5
    } else {
        6
    };

    println!("test_let_in_if result is {}", number);
}

fn test_loop_months() {
    println!("loop test ================");

    let months: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dev"];
    let mut i = 0;

    let result = loop {
        println!("{} is {}", i + 1, months[i]);

        i = i + 1;
        if i >= 12 {
            break i;
        }
    };

    println!("loop result is {}", result);
}

fn test_while() {
    println!("while test ================");

    let months: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dev"];
    let mut i = 0;

    while i < 12 {
        println!("index is {}, and month is {}", i + 1, months[i]);
        i += 1;
    };
}

fn test_for() {
    println!("for test ================");

    let months: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dev"];

    for label in months.iter().rev() {
        println!("month is {}", label);
    };
}

fn test_for_with_rev() {
    println!("for test with rev ================");

    let months: [&str; 12] = ["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dev"];

    for index in 2..6 {
        println!("label {}", months[index]);
    };
}

fn f_to_c(f: f32) -> f32 {
    (f - 32.0) / 1.8
}

fn c_to_f(c: f32) -> f32 {
    c * 1.8 + 32.0
}

// fn fib(n: i32, acc: i32 = 0) -> i32 {
//     if n <= 0 {
//         return acc;
//     }

//     // 0, 1, 2, 3, 4, 5, 6, 7...
//     //    1, 3, 6, 10, 14
// }

fn fib(n: i32) -> i32 {
    let mut count = n;
    let mut a = 0;
    let mut b = 1;
    let mut temp = 0;

    while count > 0 {
        temp = a + b;
        a = b;
        b = temp;
        count -= 1;
    }

    a

    // if n <= 0 {
    //     return a;
    // }

    // fib(b, a + b, n - 1)
}

fn main() {
    println!("global constans value -> {}", BASE_NUM);
    println!("test_shadow() -> {}", test_shadow());
    println!("test_spaces() -> {}", test_spaces());
    println!("test_mut_spaces() -> {}", test_mut_spaces());
    println!("test_float() -> {}", test_float() == 9.0);
    println!("test_bool() -> {}", !test_bool());
    println!("test_char() -> {}", test_char());

    let (x, _, z) = test_tup();

    println!("test_tup() -> {}, {}, {}", x, test_tup().1, z);

    let mut arr = test_arr();

    arr[0] = "JAN";

    let mut jan = arr[0];

    jan = "january";

    println!("test_arr() -> {}, {}", arr[0], jan);

    log(1, 2);
    println!("add {} + {} = {}", 10, 10, add(10, 10)); 
    isEven(3);
    isEven(4);
    isEven2(5);
    isEven2(6);

    println!("------------------");
    match4(0);
    match4(1);
    match4(2);
    match4(3);
    match4(4);
    match4(5);
    println!("------------------");

    test_let_in_if();
    test_loop_months();
    test_while();
    test_for();
    test_for_with_rev();

    println!("f({}) to c({})", 32.0, f_to_c(32.0));
    println!("c({}) to f({})", 0, c_to_f(0.0));
    println!("{}th fibonacci number is {}", 10, fib(12));
}