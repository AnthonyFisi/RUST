#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,

}


enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {

    //The match Control Flow Construct


    let x = value_in_cents(Coin::Quarter(UsState::Alabama));


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}",six);
    println!("{:?}",none);


    let dice_roll = 20;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other1 => move_player(other1),
    }


    //Concise Control Flow with if let

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

}


fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    println!("Data");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}",state);
            25
        },
    }
}


