fn main() {
    let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    let y: Option<i8> = None;

    let y = match y {
        Some(value) => value,
        None => 1
    };
    // println!("{}", x + y.unwrap_or(1));
    println!("{}", x + y);
}
