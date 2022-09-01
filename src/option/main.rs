fn main() {
    let some_x = Some(99);
    let none_x = None;

    println!("{:?}", add_one_if_not_null(none_x));
    println!("{:?}", add_one_if_not_null_short(none_x));
    println!("{:?}", add_one_if_not_null(some_x));
    println!("{:?}", add_one_if_not_null_short(some_x));
}

fn add_one_if_not_null(maybe_x: Option<i32>) -> Option<i32> {
    match maybe_x {
        Some(x) => Some(x + 1),
        None => None,
    }
}

fn add_one_if_not_null_short(maybe_x: Option<i32>) -> Option<i32> {
    Some(maybe_x? + 1)
}
