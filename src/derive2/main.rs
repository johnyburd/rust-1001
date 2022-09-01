use serde::Serialize;

#[derive(Serialize)]
struct TestData {
    count: u32,
    message: String,
    state: State,
}

#[derive(Serialize)]
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

    let json = serde_json::to_string(&test).expect("error serializing");
    let yaml = serde_yaml::to_string(&test).expect("error serializing");

    println!("\njson:\n{}", json);
    println!("\nyaml:\n{}", yaml);
}

// serde_json is not the only option!
// https://github.com/johnyburd/manager-rs/blob/main/agent/manager/src/data_model.rs
