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
fn main() {
    println!("Hello, world!");
    let res: u128 = fact(20);
    println!("{}", res);
}
