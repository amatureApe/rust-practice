fn main() {
    let one = 1;

    if one > 0 {
        println!("True");
    } else if one == 1 {
        println!("Equal");
    } else {
        println!("False");
    }

    println!("------------------------");

    let mut num = 0;
    'counter: loop {
        println!("Count: {}", num);

        let mut decrease = 5;
        loop {
            println!("Decreasing: {}", decrease);
            if decrease == 4 {
                break;
            }
            if num == 2 {
                break 'counter;
            }
            decrease -= 1;
        }
        num += 1;
    }

    println!("------------------------");

    let mut num = 0;
    while num < 5 {
        println!("Num: {}", num);
        num += 1;
    }

    println!("------------------------");

    let vec: Vec<i8> = (0..10).collect();

    for element in vec {
        println!("{}", element);
    }

    println!("------------------------");

    for number in (1..4).rev() {
        println!("{}", number)
    }
    println!("LIFTOFF");
}
