fn main() {
    // Both works
    let data = "This is a string";
    let s = data.to_string();
    let mut s = "This is a string".to_string();

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");
    // let s2 = s2.to_string();
    let s3 = s1.clone() + &s2; // Here s1 will be available again
    println!("s3 is {s3}");
    println!("s1 is {s1}");

    other_function();

    let hello = "Здравствуйте";

    let ss = &hello[0..4];
    // Will be Зд because special characters like this takes two bytes

    for i in "another string".chars() {
        println!("{i}");
    }
}

fn other_function() {
    let s1 = "Tic".to_string();
    let s2 = "Tac".to_string();
    let s3 = "Toe".to_string();

    let s = format! {"{s1}-{s2}-{s3}"}; //
    println!("{s}");
}
