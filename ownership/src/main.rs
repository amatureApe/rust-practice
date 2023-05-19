fn main() {
    let var = 1;
    let mut s = "hello".to_string();
    s.push_str(",world");

    let x = vec!["tyler".to_string()];
    let y = x;
    let z = y;
    println!("{:?}", z);

    let x = vec!["tyler".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("{:?}", x);
    println!("{:?}", y);
    println!("{:?}", z);

    let x = 1;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s = String::from("takes");
    takes_ownership(s);

    let val = 1;
    make_copy(val);

    let str1: String = give_ownership();
    println!("{}", str1);

    let str3: String = take_and_give(str1);

    // if true {
    //     let str4 = str3;
    // } else {
    //     let str5 = str3;
    // }

    println!("{}", str3);

    let mut string1 = String::from("Tyler");
    let mut string2: String;

    // loop {
    //     string2 = string1;
    // }

    let mut s = String::from("hello");
    change_string(&mut s);
    println!("{}", s);
}

fn change_string(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn takes_ownership(s: String) {
    let strin = s;
    println!("{}", strin);
}

fn make_copy(one: i32) {
    let val1 = one;
    println!("{}", val1);
}

fn give_ownership() -> String {
    "given".to_string()
}

fn take_and_give(str2: String) -> String {
    str2
}
