fn move_data() {
    let a = String::from("hello!");
    let b = a.clone(); // deep copy

    println!("{}, {}", a, b);
}

fn move_to_fun() {
    let str = String::from("test");

    move_to_fun_(str);

    // println!("{}", str); error occur access moved value;
}

fn move_to_fun_(str: String) {
    println!("{}", str);
}

fn str_len_with_tuple(str: String) -> (String, usize) {
    let len = str.len();

    return (str, len);
}

fn test_str_len() {
    let hello = String::from("hello!");
    let (_str, _len) = str_len_with_tuple(hello);

    println!("{}({})", _str, _len);
}

fn test_str_len_with_ref() {
    let hello = String::from("hello!");

    println!("{}, ({})", &hello, str_len(&hello));
}

fn str_len(str: &String) -> usize {
    str.len()
}

fn test_mut_str() {
    let mut s = String::from("hello");

    append_world(&mut s);

    println!("{}", s);
}

fn append_world(s: &mut String) {
    s.push_str(" world!");
}

fn test_dangle_ref() {
    println!("get_dangle_ref => {}", get_dangle_ref());
}

fn get_dangle_ref() -> String {
    let s = String::from("hello");

    s
}

fn test_first_word() {
    let mut s = String::from("hello, world!");
    let index = first_word_with_index(&s);

    println!("test_first_word => {}", &s[..index]);

    let sliced = first_word(&s);
    // s.clear(); error occur because immutable borrowed create sliced variable

    println!("test_first_word => {}", sliced);
}

fn first_word_with_index(text: &String) -> usize {
    let bytes = text.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' || c == b',' {
            return i;
        }
    }

    return bytes.len();
}

fn first_word(text: &String) -> &str {
    let bytes = text.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' || c == b',' {
            return &text[..i];
        }
    }

    &text[..]
}

fn main() {
    move_data();
    move_to_fun();
    test_str_len();
    test_str_len_with_ref();
    test_mut_str();
    test_dangle_ref();
    test_first_word();
}
