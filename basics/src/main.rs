fn main() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("{}", ans);

    println!("--------------------");

    let mut v: Vec<i32> = (0..=10).step_by(2).collect();
    v.pop();
    v.push(12);
    println!("{:?}", v);

    println!("--------------------");

    let string = "Hello";
    let concatenated_string = concat_string(string);
    println!("{}", concatenated_string);

    println!("--------------------");

    control_flow(50);
}

fn concat_string(input: &str) -> String {
    let result = String::from(input) + " World";
    result
}

fn control_flow(x: i32) {
    if x == 1 {
        println!("The value is 1")
    } else if x > 49 {
        println!("The value is greater than equal to 50");
    } else if x < 25 {
        println!("The value is less than 25");
    } else {
        println!("The value is greater than 25 but less than 50");
    }
}
