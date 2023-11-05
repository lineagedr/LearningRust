// Reguar struct
struct Player {
    name: String,
    level: u8,
}

impl Player {
    fn print(&self) {
        println!("Player {}, Level {}", self.name, self.level);
    }
}

// Rust also supports structs that look similar to tuples, called tuple structs.
// Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields;
// rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple a name
// and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
struct RGB16(u16, u16, u16);

// You can also define structs that don’t have any fields!
// Such a struct is called ‘unit-like’ because it resembles the empty tuple, (), sometimes called ‘unit’. Like a tuple struct, it defines a new type.
// This is rarely useful on its own (although sometimes it can serve as a marker type), but in combination with other features, it can become useful.
// For instance, a library may ask you to create a structure that implements a certain trait to handle events.
// If you don’t have any data you need to store in the structure, you can create a unit-like struct.
struct AlwaysEqual;

fn main() {
    let player = Player {
        name: String::from("Tester"),
        level: 3,
    };
    player.print();

    // Tuple struct
    let color = RGB16(65535, 0, 0);
    println!("red {}, green {}, blue {}", color.0, color.1, color.2);

    // Unit-like struct
    let subject = AlwaysEqual;

    // Example using tuple structs this version isn't as clear as it should be
    // what is 30? what is 50? that's the drawback of using tuple structs
    let rectangle = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels",
        area(rectangle)
    );

    // example using a regular struct
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    rectangle.print_area();

    // using constructor pattern
    let rectangle = Rectangle::new(10);

    // default constructed rectangle
    let rectangle = Rectangle::default();
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// a better example than tuple struct
#[derive(Debug)] // we can use #[derive(Debug)] for println!("{:?}", rectange);
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // when we create an associative function called new
    // usually we use it as a constructor
    pub fn new(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // methods inside the iml are called associative functions
    fn get_area(&self) -> u32 {
        self.width * self.height
    }

    fn print_area(&self) {
        println!(
            "The area of the rectangle is {} square pixels",
            self.get_area()
        );
    }
}

// Rust supports default constructors with the Default trait:
impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
        }
    }
}
