#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysEqual;

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
}

impl Circle {
    fn area(&self) -> f64 {
        self.radius.pow(2) as f64 * std::f64::consts::PI
    }

    fn can_hold(&self, other: &Circle) -> bool {
        self.radius >= other.radius
    }

    fn smallest() -> Circle {
        Circle {
            center: Point(0, 0, 0),
            radius: 1,
        }
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    println!("{:#?}", user1);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{:#?}", user2);

    let black = Color(1, 3, 4);
    let origin = Point(0, 1, 7);

    println!("{:?} {:?}", black, origin);

    let subject = AlwaysEqual;

    println!("{:?}", subject);

    let circle1 = Circle {
        center: Point(0, 1, 2),
        radius: 2,
    };

    println!("{:?}", circle1);
    println!("{}", area(&circle1));

    dbg!(&circle1);

    println!("{}", circle1.area());

    let circle2 = &Circle {
        center: Point(0, 1, 2),
        radius: 4,
    };

    println!("{}", circle2.area());

    println!("{}", circle1.can_hold(&circle2));
    println!("{}", circle2.can_hold(&circle1));
    println!("{}", Circle::smallest().area());
}

fn area(c: &Circle) -> f64 {
    c.radius.pow(2) as f64 * std::f64::consts::PI
}
