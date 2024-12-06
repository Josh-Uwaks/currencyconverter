Scaler Types in Rust / Rust Primitives
1. Integers.
2. Floats.
3. Boolean.
4. Characters.

Integers.
-- Signed Integers (numbers that includes both positive and negative integers).
-- Unsigned Integers (numbers that includes only positive integers).

Signed Integers 
numbers that includes both postive and negative integers, i8, i16, i32, i64, i128
size of each bits.
    println!("min i8 is {}", std::i8::MIN);
    println!("max i8 is {}", std::i8::MAX);

    println!("min i16 is {}", std::i16::MIN);
    println!("max i16 is {}", std::i16::MAX);

    println!("min i32 is {}", std::i32::MIN);
    println!("max i32 is {}", std::i32::MAX);

    println!("min i64 is {}", std::i64::MIN);
    println!("max i64 is {}", std::i64::MAX);

i8 = 255 = -128 to 127
u8 = 255 = 0 to 255

isize and usize are picked based on the computers architecture
default is i32 (+/- 2billion)

floats.
--f32.
--f64 (default).


characters(4 bits) are surrounded by single quotes and not double quotes.
example.
let name: char = 'a'


---------------------------------------------------

Strings
fn main() {

    let example_str: &str = "Hello";
    let string_from_str: String = example_str.to_string();
    let example_string: String = String::from("Hello");

    println!("example_str {}", string_from_str)
    println!("example_string {}", example_string)
}


Conditional Statements

Tuples 
Tuples are (a way to store multiple values in a compound type).

//EXAMPLE 1.
fn main() {
    let data = some_colors();
    println!("first color code is {}", data.0)
}
fn some_colors() -> (u8,u8,u8) {
    (1,2,3)
}

//EXAMPLE 2
fn main() {
    let data = (2,3,4);
    println!("tuple for the first data {} {}", data.0, data.1)
}

//EXAMPLE 3
fn main() {
    let (first_no1, second_no2, third_no3) = some_random_numbers();
    println!("first number is {}", first_no1)
}
fn some_random_numbers() -> (u8, u8, u8) {
   (100, 101, 102)
}

Structs, Traits and Impl

Generics
-Abstract types.
-Adds Flexibility.
-Reduce Code duplication.
-No Runtime cost.

//Example 1
struct Point<T> {
    x: T,
    y: T
}

fn main(){
    let coords = Point {x: 10, y: 20};
    println!("x is {}, y is {}", coords.x, coords.y)

    let coords_float = Point {x: 10.2, y: 20.4};
    println!("x is {}, y is {}", coords_float.x, coords_float.y)
}

it helps in abstract types especially avoiding multiple definition of struct of the same types.

//Example
struct Point {
    x: i32,
    y: i32
}

struct Point {
    x: f64,
    y: f64
}

by default compiler will throw an error for this scenario

Example// for multiple types.
struct Point<T, U>{
    x: T,
    y: U
}

fn main(){
    let coords = Point {x: 10, y:20.4};
    println!("x is {}, y is {}", coords.x, coords.y)
}