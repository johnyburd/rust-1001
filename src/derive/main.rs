#[derive(PartialEq, Clone, Debug)]
struct TestData {
    count: u32,
    message: String,
    state: State,
}

#[derive(PartialEq, Clone, Debug)]
enum State {
    Good,
    Bad(String),
}

fn main() {
    let test = TestData {
        count: 42,
        message: "hello".into(),
        state: State::Good,
    };

    let test2 = TestData {
        count: 42,
        message: "hello".into(),
        state: State::Good,
    };

    assert_eq!(test, test2);

    let test3 = test2.clone();

    println!("debug: {:?}", test3);
}
