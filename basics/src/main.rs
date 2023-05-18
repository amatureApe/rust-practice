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
}

fn concat_string(input: &str) -> String {
    let result = String::from(input) + " World";
    result
}
