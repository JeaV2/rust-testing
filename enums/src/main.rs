use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum UsStates {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStates),
}

fn main() {
    // Using enums to represent different types of IP addresses
    let home = IpAddress::V4(Ipv4Addr::new(127, 0, 0, 1));
    let loopback = IpAddress::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

    print_ip(&home);
    print_ip(&loopback);

    // Using enums to represent optional values
    enum_option();

    // Using enums to represent different types of coins and their values
    let coin = &Coin::Penny;
    let coin2 = &Coin::Quarter(UsStates::California);
    let coin3 = &Coin::Dime;
    let coin4 = &Coin::Nickel;
    println!("Value of the coin: {} cents \n", value_in_cents(coin));
    println!("Value of the coin: {} cents \n", value_in_cents(coin2));
    println!("Value of the coin: {} cents \n", value_in_cents(coin3));
    println!("Value of the coin: {} cents \n", value_in_cents(coin4));

    let dime_penny = value_in_cents(coin3) + value_in_cents(coin);
    println!("Value of dime and penny: {} cents \n", dime_penny);

    // Using enums to represent optional values and performing operations on them
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("plus_one(Some(5)) = {:?} \n", six);
    println!("plus_one(None) = {:?} \n", none);

}

fn print_ip(ip: &IpAddress) {
    match ip {
        IpAddress::V4(addr) => println!("IPv4: {}", addr),
        IpAddress::V6(addr) => println!("IPv6: {}", addr),
    }
}

fn enum_option() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    match y {
        Some(value) => println!("{} \n", add(x, value)),
        None => println!("No value provided for y \n"),
    }
}
fn add(x: i8, y: i8) -> i8 {
    let z = x + y;
    return z;
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            print!("Lucky penny! \n");
            1
        }
        Coin::Nickel => {
            println!("Nickel coin!");
            5
        }
        Coin::Dime => {
            println!("Dime coin!");
            10
        }
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
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