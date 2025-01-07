#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from the state: {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    value_in_cents(&coin);
    let five = Some(5);
    let six = plus_one(five);

    println!("{}", six.unwrap_or(0));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // other is exhaustive for all the other options
        _ => reroll(); // special pattern that matches any value and does not bind to that value
        _ => (); // Does not do anything for any other numbers than 3 or 7
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
