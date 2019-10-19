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

fn main() {
    println!("global constans value -> {}", BASE_NUM);
    println!("test_shadow() -> {}", test_shadow());
    println!("test_spaces() -> {}", test_spaces());
    println!("test_mut_spaces() -> {}", test_mut_spaces());
    println!("test_float() -> {}", test_float() == 9.0);
    println!("test_bool() -> {}", !test_bool());
    println!("test_char() -> {}", test_char());
}
