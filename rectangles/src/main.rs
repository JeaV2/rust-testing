use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 50
    }
}

fn main() {
    println!("Welcome to the rectangle calculator!\nType \"q\" to exit at any time.");
    'input_loop: loop {
        let mut width = String::new();
        let mut height = String::new();
        let mut scale = String::new();

        println!("Enter the width of the rectangle:");
        io::stdin()
            .read_line(&mut width)
            .expect("Failed to read line");
        println!("Enter the height of the rectangle:");
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read line");
        println!("Enter the scale of the width of the rectangle:");
        io::stdin()
            .read_line(&mut scale)
            .expect("Failed to read line");

        if width.trim() == "q" || height.trim() == "q" || scale.trim() == "q" {
            println!("Exiting the program.");
            break 'input_loop;
        }

        let width: u32 = match width.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type in a number for width");
                continue;
            }
        };
        let height: u32 = match height.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type in a number for height");
                continue;
            }
        };
        let scale: u32 = match scale.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type in a number for scale");
                continue;
            }
        };

        let rectangle = Rectangle { width: width * scale, height };
        println!("The area of the rectangle is: {}", rectangle.area());
        if rectangle.width() {
            println!("The rectangle is wider than 50.");
        } else {
            println!("The rectangle is not wider than 50.");
        }
        break 'input_loop;
    }
}

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
// impl Rectangle {
//     fn _area(&self) -> u32 {
//         self.width * self.height
//     }

//     fn _width(&self) -> bool {
//         self.width > 50
//     }

//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }

//     fn square(size: u32) -> Self {
//         Self {
//             width: size,
//             height: size,
//         }
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect3 = Rectangle {
//         width: 60,
//         height: 45,
//     };

//     println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
//     println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

//     let sq = Rectangle::square(3);
//     println!("The area of the square is: {}", sq._area());
// }