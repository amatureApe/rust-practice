struct MyString<'a> {
    text: &'a str,
}

fn main() {
    let str1 = String::from("This is my string");
    let x = MyString {
        text: str1.as_str(),
    };
    let s: &'static str = "I have a static lifetime";

    let r;

    {
        let x = 5;
        r = &x;
    }

    // println!("{}", r);
}

fn example<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
