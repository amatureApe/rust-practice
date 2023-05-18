fn main() {
    let x: i8 = 10;
    println!("{}", x);

    let y: u8 = 10;

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';
    println!("{}", byte);

    let p = 2_0;
    let m: f32 = 1.0;

    let t = true;
    let f: bool = false;

    let c = 'c';

    println!("{}", c);

    let a = 10;
    let b = 4;

    let remainder = a % b;
    println!("{}", remainder);

    println!(" TUPLES -----------------------");

    let tup = (500, "hi", true);
    println!("{}", tup.0);

    let (x, y, z) = tup;

    println!("{}", x);
    println!("{}", y);
    println!("{}", z);

    println!(" ARRAYS -----------------------");

    let array = [1, 2, 3];

    println!("{}", array[0]);

    let mut array2: [i32; 3] = [4, 5, 6];
    println!("{}", array2[0]);
    array2[0] = 10;
    println!("{}", array2[0]);

    let mut nums = vec![1, 2, 3];

    nums.push(4);
    println!("{:?}", nums);
    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new();
    vec.push("Test");
    vec.push("String");
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);

    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{}", vect.capacity());

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv: &[i32] = &v[2..4];
    println!("{:?}", sv);

    println!(" STRINGS -----------------------");

    let name = String::from("John Doe");
    let course = "Rust".to_string();
    let new_name = name.replace("John Doe", "JD");

    println!("{}", name);
    println!("{}", course);
    println!("{}", new_name);

    let str1 = "hello";
    let str2 = str1.to_string();
    let str3 = &str2;

    println!("{}", str1);
    println!("{}", str2);
    println!("{}", str3);

    println!("{}", "ONE".to_lowercase() == "one");

    let rust = "\x52\x75\x73\x74";
    println!("{}", rust);
}
