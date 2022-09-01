#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let vec1 = create_vec();

    let vec2 = vec![1, 2, 3];

    println!("vec 1 {:?}", vec1);
    println!("vec 2 {:?}", vec2);
}

fn create_vec() -> Vec<i32> {
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
