// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     struct Color(i32, i32, i32);

//     struct Point(i32, i32, i32);

//     let mut user1 = User {
//         email: String::from("yafi@mail.com"),
//         username: String::from("kerenzikov"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let name = user1.username;
//     user1.username = String::from("karenzikov");
//     let user2 = build_user(String::from("yafi@gm"), String::from("gm"));

//     let user3 = User {
//         email: String::from("adaud@"),
//         username: String::from("adad"),
//         ..user2
//     };
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }
//

#[derive(Debug)]
struct Rectangle {
    widht: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.widht * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.widht > other.widht && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            widht: size,
            height: size,
        }
    }
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    let rect = Rectangle {
        widht: 30,
        height: 50,
    };
    let rect1 = Rectangle {
        widht: 3,
        height: 5,
    };
    let rect2 = Rectangle {
        widht: 8,
        height: 10,
    };
    let rect3 = Rectangle::square(25);

    println!("the area if rect is {}", rect.area());
    println!("can hold? {}", rect.can_hold(&rect1));
    println!("can hold? {}", rect.can_hold(&rect2));
}
