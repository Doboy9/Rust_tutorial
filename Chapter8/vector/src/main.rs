fn main() {
    let v: Vec<i32> = Vec::new(); //Creating a new, empty vector

    let v = vec![1, 2, 3]; //Creating a new vector containing values
    let third: &i32 = &v[2];
    println!("The third argument in the vector is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        None => println!("No third element"),
        Some(third) => println!("The third element is {}", third),
    }

    for i in &v {
        println!("{i}");
    }
}
