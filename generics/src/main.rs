#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

trait Overview {
    fn overview(&self) -> String {
        String::from("This is Rust!")
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Drop for Course{
    fn drop(&mut self) {
        println!("Dropping: {}", self.author)
    }
}

struct AnotherCourse {
    headline: String,
    author: String,
}

impl Overview for Course {

}

impl Overview for AnotherCourse {
    fn overview(&self) -> String {
        format!("{}, {}", self.author, self.headline)
    }
}

use std::ops::Add;

impl<T> Add for Point<T>
    where
    T: Add<Output = T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            Point{
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        }
    }

fn main() {
    let coord = Point{x: 5.0, y: 5.0};
    // let coord2 = Point{x: 'x', y: 5};
    let coord3 = Point{x: 5.0, y: 5.0};
    let coord4 = Point{x: 1.0, y: 2.0};

    let sum = coord3 + coord4;
    println!("{:?}", sum);

    let course1 = Course{headline: String::from("Headline!"), author: String::from("Jake"),};
    let course2 = AnotherCourse{headline: String::from("Another headline!"), author: String::from("Mike"),};

    println!("{}", course1.overview());
    println!("{}", course2.overview());

    call_overview(&course1);
    call_overview(&course2);

    drop(course1);
}

fn call_overview<T: Overview>(item: &T) {
    println!("Overivew: {}", item.overview())
}

trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self){
        *self = source.clone()
    }
}

// fn call_overview(item: &impl Overview) {
//     println!("Overivew: {}", item.overview())
// }

// fn overview(item1: &impl Overview, item2: &impl Overview) {}
// fn overview<T: Overview>(item1: &T, item2: &T){}
// fn overview(item1: &impl Overview + AnotherTrait)
// fn overview<T: Overview + AnotherTrait>(item1: &T, item2: &T)