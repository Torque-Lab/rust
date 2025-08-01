fn fact(i: u128) -> u128 {
    if i == 0 {
        return 1;
    } else {
        return i * fact(i - 1);
    }
}
fn _fact2(i: i128) -> i128 {
    if i == 0 {
        return 1;
    } else {
        return i * _fact2(i - 1);
    }
}

fn give_length(s: &String) -> usize {
    s.len()
}

fn main() {
    let str: String = String::from("Hello, world!");
    let len: usize = give_length(&str);
    println!("Length of string: {}", len);

    println!("{}", str);
    println!("Hello, world!");
    let res: u128 = fact(20);
    println!("{}", res);
}
