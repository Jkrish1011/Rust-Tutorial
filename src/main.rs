// mod test;

// Two goals: 1. Modify assert! to make it work 2. Make println! output: 97 - 122

// fn main() {
//     let mut sum = 0;
//     for i in -3..2 {
//         sum += i
//     }

//     assert!(sum == -5);

//     for c in 'a'..='z' { // z is included
//         // println!("{}",c);
//         println!("{}",c as u8); // type casting as u8 is important here
//     }
// }

// Fill the blanks
// use std::ops::{Range, RangeInclusive};

// fn main() {
//     assert_eq!((1..5), Range{ start: 1, end: 5 });
//     assert_eq!((1..=5), RangeInclusive::new(1, 5)); // 1..=5 is the important thing here

//     println!("Success!");
// }

// Fill the blanks and fix the errors
// fn main() {
//     // Integer addition
//     assert!(1u32 + 2 == 3); // 2 will be inferred as 2u32
//     assert!(1u32 + 2u32 == 3u32);

//     // Integer subtraction
//     assert!(1i32 - 2 == -1i32);
//     assert!(1i8 - 2 == -1);

//     assert!(3 * 50 == 150);
//     assert!(3 * 50 == 150i32);

//     assert!(9.6 as f32 / 3.2 as f32 == 3.0); // error ! make it work

//     assert!(24 % 5 == 4);
//     // // Short-circuiting boolean logic
//     assert!(true && false == false);
//     assert!(true || false == true);
//     assert!(!true == false);

//     // // Bitwise operations
//     println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     println!("1 << 5 is {}", 1u32 << 5);
//     println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
// }

// Make it work
// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),4); // 4 bytes

//     // println!("{}", size_of_val(&c2)); // 4 bytes

//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),4);

//     println!("Success!");
// }

// Make it work
// fn main() {
//     let c1 = '中';
//     print_char(c1);
// }

// fn print_char(c : char) {
//     println!("{}", c);
// }

// Modify `4` in assert to make it work
// use std::mem::size_of_val;
// fn main() {
//     let unit: () = ();
//     assert!(size_of_val(&unit) == 0);
//     // println!("{}", size_of_val(&unit)); // 0

//     println!("Success!");
// }

// fn main() {
//     let x = 5u32;

//     let y = {
//         let x_squared = x * x;
//         let x_cube = x_squared * x;

//         // This expression will be assigned to `y`
//         x_cube + x_squared + x
//     };

//     let z = {
//         // The semicolon suppresses this expression and `()` is assigned to `z`
//         2 * x;
//     };

//     println!("x is {:?}", x);
//     println!("y is {:?}", y);
//     println!("z is {:?}", z);
// }

// Make it work with two ways
// fn main() {
//     let v: i32 = {
//         let mut x: i32 = 1;
//         x += 2; // If this is the last line, () will be returned
//         x
//     };

//     assert_eq!(v, 3);

//     println!("Success!");
//  }

// fn main() {
//     let s: i32 = sum(1 , 2);
//     assert_eq!(s, 3);

//     println!("Success!");
// }

// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// fn main() {
//     // Don't modify the following two lines!
//     let (x, y) = (1, 2);
//     let s = sum(x, y);

//     assert_eq!(s, 3);

//     println!("Success!");
// }

// // Functions always have to annotate their type for their arguements
// fn sum(x: i32, y: i32) -> i32 {
//     x + y
// }

// Solve it in two ways
// DON'T let `println!` work
// fn main() {
//     never_return();

//     println!("Failed!");
// }

// fn never_return() -> ! {
//     // Implement this function, don't modify the fn signatures
//     panic!();
// }

// fn main() {
//     println!("Success!");
// }

// fn get_option(tp: u8) -> Option<i32> {
//     match tp {
//         1 => {
//             // TODO
//         }
//         _ => {
//             // TODO
//         }
//     };

//     // Rather than returning a None, we use a diverging function instead
//     never_return_fn()
// }

// // IMPLEMENT this function in THREE ways
// fn never_return_fn() -> ! {
//     panic!();
//     unimplemented!();
//     todo!();
// }

// Each value in RUST has an owner
// There can only be one owner at a time
// When the owner goes out of scope, the value will be dropped

// The owner of a value is the variable or data structure that holds it and responsible for allocating
// and freeing the memory used to store the data.

// Scope
// The range within a program for which an item is valid

// Global Scope - accessible throughout the entire program
// Local Scope - Accesible only within particular function or block of code

// Data Types
// String
// String is mutable
// String size can change at runtime
// String stored on the stack with a pointer to the heap
// Value of String is stored on the heap

// fn main() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     // Now cannot use s1 afterwards since it is copied to s2.
//     // println!("s1 is {}", s1);
//     println!("s2 is {}", s2);

//     let s3 = String::from("hello");
//     // we need to use clone to continue using the copied variable in the program
//     let s4 = s3.clone();
//     println!("s3 = {} and s4 = {}", s3, s4);
// }

// Ownership
// ownership is transferred to functions if the data structure's is held on heap
// ownership is not transferred to functions if the data structure's is held on stack

// fn main() {
//     // Use as many approaches as you can to make it work
//     let x: String = String::from("Hello world");
//     let y: String = x.clone();
//     println!("{}, {}",x, y);
// }

// Don't modify code in main!
// fn main() {
//     let s1: String = String::from("Hello world");
//     let s2: String = take_ownership(s1);

//     println!("{}", s2);
// }

// // Only modify the code below!
// // If something is being returned from a function, we have to annotate the datatype of it ( -> DataType)
// fn take_ownership(s: String) -> String {
//     println!("{}", s);
//     s
// }

// fn main() {
//     let s = give_ownership();
//     println!("{}", s);
// }

// // Only modify the code below!
// fn give_ownership() -> String {
//     let s = String::from("Hello world");
//     // Convert String to Vec
//     let _s = s.as_bytes();
//     // println!("{}", _s);
//     s
// }

// Don't use clone ,use copy instead
// fn main() {
//     let x: (i32, i32, (), &str) = (1, 2, (), "hello");
//     let y: (i32, i32, (), &str) = x;
//     println!("{:?}, {:?}", x, y);
// }

// fn main() {
//     // Box is heap allocated data structure
//     let x: Box<i32> = Box::new(5);

//     let mut y: Box<i32> = Box::new(1);       // update this line, don't change other lines!

//     *y = 4;
//     println!("{}", x);
//     assert_eq!(*x, 5);

//     println!("Success!");
// }

// fn main() {
//     #[derive(Debug)]
//     struct Person {
//         name: String,
//         age: Box<u8>,
//     }

//     let person: Person = Person {
//         name: String::from("Alice"),
//         age: Box::new(20),
//     };

//     // `name` is moved out of person, but `age` is referenced
//     let Person { name, ref age } = person;

//     println!("The person's age is {}", age);

//     println!("The person's name is {}", name);

//     // Error! borrow of partially moved value: `person` partial move occurs
//     //println!("The person struct is {:?}", person);

//     // `person` cannot be used but `person.age` can be used as it is not moved
//     println!("The person's age from person struct is {}", person.age);
// }

// fn main() {
//     let t: (String, String) = (String::from("hello"), String::from("world"));
//     // t.0's ownership is transferred to _s. only t.1 is owned by t after this.
//     let _s: String = t.0;

//     // Modify this line only, don't use `_s`
//     println!("{:?}", t.1);
//  }

// fn main() {
//     let t: (String, String) = (String::from("hello"), String::from("world"));

//     // Fill the blanks
//     let (s1, s2) = t.clone();

//     println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
// }

// Borrowing
// Way of temporarily accessing data without taking ownership of it.

// Rules of References
// 1. At any given time, you can have either one mutable reference or any number of immutable references to the same data
// 2. References must always be valid.

// fn main() {
//     let mut s: String = String::from("Hello");
//     let r1 = &s;
//     let r2 = &s;
//     // let r3 = &mut s; - Will not work as the 1st rule will be violated!
//     println!("{} and {}", r1, r2);

//     let r3 = &mut s;
//     println!("{}", r3);
// }

// Dangling References

// fn main() {
//     // here reference_to_nothing points to s which will be dropped after the scope of function. This violates 2nd rule of References.
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// fn main() {
//     let x:i32 = 5;
//     let y:&i32 = &x;

//     // Modify this line only
//     assert_eq!(5, *y);

//     println!("Success!");
// }

// // Fix error
// fn main() {
//     let s: String = String::from("hello, ");

//     borrow_object(&s);

//     println!("Success!");
// }

// fn borrow_object(s: &String) {}

// Fix error
// fn main() {
//     let mut s: String = String::from("hello, ");

//     push_str(&mut s);

//     println!("Success!");
// }

// fn push_str(s: &mut String) {
//     s.push_str("world")
// }

// fn main() {
//     let mut s:String = String::from("hello, ");

//     // Fill the blank to make it work
//     let p:&mut String = &mut s;

//     p.push_str("world");

//     println!("{:p}", p);
//     println!("Success!");
// }

// fn main() {
//     let c: char = '中';

//     let r1: &char = &c;
//     // Fill the blank，dont change other code
//     // ref is same as &. here r1 and r2 will point to the same variable c.
//     let ref r2 = c;
//     // let r2:&char = &c; // same as above

//     assert_eq!(*r1, *r2);

//     // Check the equality of the two address strings
//     assert_eq!(get_addr(r1),get_addr(r2));

//     println!("Success!");
// }

// // Get memory address string
// fn get_addr(r: &char) -> String {
//     format!("{:p}", r)
// }

// fn main() {
//     let mut s: String = String::from("hello");

//     let r1: &String = &s;
//     let r2: &String = &s;

//     println!("{}, {}", r1, r2);

//     println!("Success!");
// }

// fn main() {
//     // Fix error by modifying this line
//     let mut s:String = String::from("hello, ");

//     // To pass a variable as mutable reference, the variable should be defined as mutable
//     borrow_object(&mut s);

//     println!("Success!");
// }

// fn borrow_object(s: &mut String) -> () {
//     println!("{}", s);
// }

// This code has no errors!
// fn main() {
//     let mut s:String = String::from("hello, ");

//     borrow_object(&s);

//     s.push_str("world");
//     println!("{}", s);
//     println!("Success!");
// }

// fn borrow_object(s: &String) -> () {
//     println!("{}", s);
// }

// Comment one line to make it work
// fn main() {
//     let mut s:String = String::from("hello, ");

//     let r1:&mut String = &mut s;
//     r1.push_str("world");

//     let r2: &mut String = &mut s;
//     r2.push_str("!");

//     // Can't print r1 here because Rust only allows 1 mutable reference at a time.
//     // Solution would be to not use r1 after r2 has been defined.
//     println!("{}",r2);
// }

// fn main() {
//     let mut s:String = String::from("hello, ");

//     let r1:&mut String = &mut s;
//     println!("{}", r1);

//     let r2:&mut String = &mut s;
//     println!("{}", r2);

//     // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
//     // You can't use r1 and r2 at the same time
//     // println!("{}, {}", r1, r2);
// }

// String vs &str
// String is a heap-allocated string type that owns it's contents and is mutable
// &str is immutable sequence of UTF-8 bytes and is immutable. - &str is more of a view than edit

// use &str if you just want a view of a string and &str is more light weight and efficent than a String

// fn main() {
//     let s:String = String::from("hello,World");
//     let hello:&str = &s[0..5];
//     let world:&str = &s[6..11];

//     println!("{}", hello);
//     println!("{}", world);
// }

// Fix all errors without adding newline
// fn main() {
//     let mut s:String = String::from("hello");
//     s.push(',');
//     s.push_str(" world");
//     s += "!";

//     println!("{}", s);
// }

// Fill the blank
// fn main() {
//     let s = String::from("I like dogs");
//     // Allocate new memory and store the modified string there
//     let s1 = s.replace("dogs", "cats");

//     assert_eq!(s1, "I like cats");

//     println!("Success!");
// }

// Fix errors without removing any line
// fn main() {
//     let s1: String = String::from("hello,");
//     let s2: String = String::from("world!");
//     // let s3 = s1 + s2.as_str(); // to add string using '+' operator, we need to convert the second String to string literal(&str) using as_str() function
//     let s3 = s1 + &s2;
//     assert_eq!(s3, "hello,world!");
//     // s1 and s2 are not valid here since s3 became the owner for values in s1 and s2
//     println!("{}", s3);
// }

// Fix error with at least two solutions
// fn main() {
//     // let s:String = String::from("hello, world");
//     // The String literal created and assigned becomes &str. To get a string use String::from("") method
// &str -> String using to_string()
//     let s:&str = "hello, world";
//     greetings(s)
// }

// fn greetings(s: &str) {
//     println!("{}", s)
// }

// Use two approaches to fix the error and without adding a new line
// fn main() {
//     let s:String = "hello, world".to_string();
//     // let s1: &str = &s; // &String is &str
//     let s1: &str = s.as_str(); // &String is &str

//     println!("Success!");
// }

// fn main() {
//     let s1 = String::from("hi,中国");
//     let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
//     assert_eq!(h, "h");

//     let h1 = &s1[3..5]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
//     // println!("{}", h1); // error: byte index 5 is not a char boundary; it is inside '中' (bytes 3..6) of `hi,中国`
//     // assert_eq!(h1, "中");

//     println!("Success!");
// }

// #[tokio::main]
// async fn main() {
//     // Fill the blank to print each char in "你好，世界"
//     // for c in "你好，世界".chars() { // .chars() will allow us to iterate over every character in a given string irrespective of how many bytes it takes
//     for c in "你好，世界中".chars() {
//         println!("{}", c)
//     }
// }


// #[tokio::main(flavor = "multi_thread")]
// async fn main() {
//     println!("Hello async!");
//     let s = do_something().await;
//     // println!("{}", s);
//     print_type_of(&s);
// }

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

// async fn do_something() {
//     std::thread::sleep(std::time::Duration::from_millis(5000));
//     println!("Do Something!");
// }


// #[tokio::main(flavor = "multi_thread")]
// async fn main() {
    
// }

// struct F1Racer {
//     name: String,
//     completed_laps: u8,
//     laps: u8,
//     best_lap_time: u8,
//     lap_times: Vec<u8>
// }

// impl F1Racer {
//     fn new() -> F1Racer {
//         return F1Racer{name: "Lewis Hamilton".to_string() , laps: 5, completed_laps: 0, best_lap_time: 255, lap_times: vec![87u8, 64, 126, 95, 76] };
//     }
// }

// fn main() {
//     // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
//     let _arr0 = [1, 2, 3];
//     let arr: [_; 3] = ['a', 'b', 'c']; // char occupies 4 bytes
    
//     // Fill the blank
//     // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
//     // A char takes 4 bytes in Rust: Unicode char
//     assert!(std::mem::size_of_val(&_arr0) == 12);

//     println!("Success!");
// }

// fn main() {
//     // Fill the blank
//     let list: [i32; 100] = [1;100]; // [constant; length_of_array] to assign all the elements to the constant value

//     assert!(list[0] == 1);
//     assert!(list.len() == 100);

//     println!("Success!");
// }


// fn main() {
//     let arr: [char; 3] = ['a', 'b', 'c'];
    
//     let ele = arr[0]; // Only modify this line to make the code work!

//     assert!(ele == 'a');

//     println!("Success!");
// }

// Slice - References to contiguous sequence of elements in a collection
// slices provides a way to `borrow` part of a collection without taking `ownership` of the entire collection
// Can be created from arrays, vectors, Strings and other collections implementing the `Deref trait`

// Slices are similar to arrays but their length is not known at compile time

// fn main() {
//     let arr:[i32; 3] = [1, 2, 3];
//     let s1: &[i32] = &arr[0..2]; // type annotation of a slice

//     let s2: &str = "hello, world"; // "hello, world" as &str;

//     println!("Success!");
// }


// fn main() {
//     let arr: [char; 3] = ['中', '国', '人'];

//     let slice = &arr[..2]; // Short-hand notation for accessing the first 2 elements
    
//     // Modify '8' to make it work
//     // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
//     println!("{}", std::mem::size_of_val(&slice)); // 16 because slices are pointers to the actual data. So it will have the pointer, and the length thereby doubling the size.
//     assert!(std::mem::size_of_val(&slice) == 16);

//     println!("Success!");
// }

// fn main() {
//     let arr: [i32; 5] = [1, 2, 3, 4, 5];
//     // Fill the blanks to make the code work
//     let slice: &[i32] = &arr[1..4];
//     assert_eq!(slice, &[2, 3, 4]);

//     println!("Success!");
// }



// fn main() {
//     let s = String::from("hello");

//     let slice1: &str = &s[0..2];
//     // Fill the blank to make the code work, DON'T USE 0..2 again
//     let slice2: &str = &s[..2];

//     assert_eq!(slice1, slice2);

//     println!("Success!");
// }


// fn main() {
//     let s: &str = "你好，世界";
//     // Modify this line to make the code work
//     let slice: &str = &s[0..3];

//     assert!(slice == "你");

//     println!("Success!");
// }

// Fix errors
// fn main() {
//     let mut s: String = String::from("hello world");

//     // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
//     // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
//     let letter: &str = first_letter(&s);
//     println!("the first letter is: {}", letter);
//     s.clear(); // The above line becomes invalid from here

    
// }

// fn first_letter(s: &str) -> &str {
//     &s[..1]
// }


// Tuple - way to store related pieces of information in a single variable
// As if Collection of values of different types grouped together as a single compund value (type composed of other types)
// tuples can contain multiple types of values
// Stored as a fixed-size contiguous block of memory on the stack


// fn main() {
//     let _t0: (u8,i16) = (0, -1);
//     // Tuples can be tuple's members
//     let _t1: (u8, (i16, u32)) = (0, (-1, 1));
//     // Fill the blanks to make the code work
//     let _t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

//     println!("Success!");
// }


// Make it work
// fn main() {
//     let t: (&str, &str, &str) = ("i", "am", "sunface");
//     assert_eq!(t.2, "sunface");
//     println!("Success!");
// }


// fn main() {
//     let tup = (1, 6.4, "hello");

//     // Fill the blank to make the code work
//     let (x, z, y) = tup;

//     assert_eq!(x, 1);
//     assert_eq!(y, "hello");
//     assert_eq!(z, 6.4);

//     println!("Success!");
// }


// Struct - compund type allowing to group together values of different types into a named data structure
// It is similar to tuples, but each value has a name associated with it - so accessing becomes easier by using that name


// Fix the error
// struct Person {
//     name: String,
//     age: u8,
//     hobby: String
// }
// fn main() {
//     let age = 30;
//     let p: Person = Person {
//         name: String::from("sunface"),
//         age,
//         hobby: String::from("swimming")
//     };

//     println!("Success!");
// } 

// Tuple Structs - Like normal structs but using tuple-like syntax for defining their fields - a named tuple
// they are instantiated by paranthesis instead of curly braces
// Accessed through point notation

// Examples
// struct Color(i32, i32, i32);

// fn main() {
//     let black = Color(0,0,0);
// }


// Fix the error and fill the blanks
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let v = Point(0, 127, 255);
//     check_color(v);

//     println!("Success!");
// }   

// fn check_color(p: Point) {
//     let Point(x, _, z) = p; // Destructuring a Named tuple
//     assert_eq!(x, 0); 
//     assert_eq!(p.1, 127);
//     assert_eq!(z, 255);
// }


// Fill the blank and fix the error without adding/removing new line
// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {
//     let age: u8 = 18;
//     let mut p: Person = Person {
//         name: String::from("sunface"),
//         age,
//     };

//     // How can you believe sunface is only 18? 
//     p.age = 30;

//     // Fill the blank
//     p.name = String::from("sunfei");

//     println!("Success!");
// }

// Fill the blank
// struct Person {
//     name: String,
//     age: u8,
// }
// fn main() {
//     println!("Success!");
// } 

// fn build_person(name: String, age: u8) -> Person {
//     Person {
//         age,
//         name // if a parameter name matches the Key name of the struct, we can use it directly without annotation Key 
//     }
// }


// Fill the blank to make the code work
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }
// fn main() {
//     let u1: User = User {
//         email: String::from("someone@example.com"),
//         username: String::from("sunface"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let u2: User = set_email(u1);

//     println!("Success!");
// } 

// fn set_email(u: User) -> User {
//     User {
//         email: String::from("contact@im.dev"),
//         ..u
//     }
// }


// Fill the blanks to make the code work
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale), // Print debug info to stderr and assign the value of  `30 * scale` to `width`
//         height: 50,
//     };

//     dbg!(&rect1); // Print debug info to stderr

//     println!("{:?}", rect1); // Print debug info to stdout
// }

// Fix errors to make it work
// #[derive(Debug)]
// struct File {
//     name: String,
//     data: String,
// }
// fn main() {
//     let f = File {
//         name: String::from("readme.md"),
//         data: "Rust By Practice".to_string()
//     };

//     let _name = f.name; // name ownership has been moved to the variable _name. So f cannot be used as a whole instance again

//     // ONLY modify this line
//     // println!("{}, {}, {:?}",f.name, f.data, f);// will fail as we are using f as a whole instance
//     println!("{}, {}", _name, f.data);
// } 


// The option ENUM - represents a value that may or may not be present (known as NULL in other languages).
// Used to handle cases where a function or method might fail to return a value.

// Option enum as defined by the standard library
// enum Option<T> {
//     None,
//     Some(T)
// }

// fn main() {
//     let five: Option<i32> = Option::Some(5);
//     let six: Option<i32> = plus_one(five);
//     let none: Option<i32> = plus_one(None);
    
//     // To destructure the OPTION we use `if let`
//     if let Some(n) = six { 
//         println!("{}", n);
//         println!("Success!");
//     } else {
//         panic!("NEVER LET THIS RUN!");
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1)
//     }
// }


// if else can be used in assignments as well
// fn main() {
//     let n = 5;

//     let big_n: i32 =
//         if n < 10 && n > -10 {
//             println!(", and is a small number, increase ten-fold");

//             10 * n
//         } else {
//             println!(", and is a big number, halve the number");

//             n / 2.0 as i32
//         };

//     println!("{} -> {}", n, big_n);
// } 


// Fix the errors without adding or removing lines
// fn main() {
//     let names: [String; 2] = [String::from("liming"),String::from("hanmeimei")];
//     // We need to add `&` as reference as we are using names again in the program - concept of ownership
//     for name in &names {
//         println!("{}", name);
//     }

//     println!("{:?}", names);

//     let numbers: [i32;3] = [1, 2, 3];
//     // The elements in numbers are Copy，so there is no move here
//     for n in numbers {
//         println!("{}", n);
//     }
    
//     println!("{:?}", numbers);
// } 


// fn main() {
//     let a: [i32; 4] = [4, 3, 2, 1];

//     // Iterate the indexing and value in 'a'
//     for (i,v) in a.iter().enumerate() {
//         println!("The {}th element is {}",i+1,v);
//     }
// }


// Fill in the blanks to make the last println! work !
// fn main() {
//     // A counter variable
//     let mut n: i32 = 1;

//     // Loop while the condition is true
//     while n < 10 {
//         if n % 15 == 0 {
//             println!("fizzbuzz");
//         } else if n % 3 == 0 {
//             println!("fizz");
//         } else if n % 5 == 0 {
//             println!("buzz");
//         } else {
//             println!("{}", n);
//         }


//         n+=1;
//     }

//     println!("n reached {}, so loop is over",n);
// }



// Fill in the blank
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     assert_eq!(result, 20);
//     println!("Success!");
// }


// Fill in the blank
// fn main() {
//     let mut count = 0;
//     'outer: loop {
//         'inner1: loop {
//             if count >= 20 {
//                 // This would break only the inner1 loop
//                 break 'inner1; // `break` is also works.
//             }
//             count += 2;
//         }
//         count += 5;
//         'inner2: loop {
//             if count >= 30 {
//                 // This breaks the outer loop
//                 break 'outer;
//             }

//             // This will continue the outer loop
//             continue 'outer;
//         }
//     }

//     assert!(count == 30);
//     println!("Success!");
// }

// #[derive(Debug)]
// // #[allow(dead_code)]
// ![alt text](http://url/to/img.png)
// enum VehicleColor {
//     Silver,
//     White,
//     Black,
//     Green,
//     Red,
//     Blue
// }

// #[derive(Debug)]
// struct Vehicle {
//     manufacturer: String,
//     model: String,
//     year: u16,
//     color: VehicleColor
// }

// impl Vehicle {
//     fn paint(&mut self, new_color: VehicleColor) {
//         self.color = new_color;
//     }   

//     fn create_vehicle() -> Vehicle {
//         let new_vehicle = Vehicle {
//             manufacturer: "default".to_string(),
//             model: "default".to_string(),
//             year: 1990,
//             color: VehicleColor::Blue 
//         };
//         new_vehicle
//     }
// }

// fn main() {
//     println!("Hello!");
//     let mut new_vehicle: Vehicle = Vehicle::create_vehicle();
//     println!("{:?}", new_vehicle);
// }


// Pattern match - powerful construct that allows to compare a value against a set of patterns, 
// then execute different code based on which pattern matches.

// - Patterns can be made up of literal values, variable names, wildcards, etc.
// - in match, all possible matches must be handled, enforced by the compiler

// fn main() {
//     let config_max: Option<u32> = Some(3u32);
//     match config_max {
//         Some(max) => println!("The maximum is configured to be {}", max),
//         _ => ()
//     }

//     // same as match, we can use if let
//     if let Some(max) = config_max {
//         println!("The maximum is configured to be {}", max)
//     }
// }


// // Fill the blanks
// enum Direction {
//     East,
//     West,
//     North,
//     South,
// }

// fn main() {
//     let dire: Direction = Direction::West;
//     match dire {
//         Direction::East => println!("East"),
//         Direction::South | Direction::North  => { // Matching South or North here
//             println!("South or North");
//         },
//         _ => println!("West"),
//     };
// }


// fn main() {
//     let boolean = true;

//     // Fill the blank with a match expression:
//     //
//     // boolean = true => binary = 1
//     // boolean = false =>  binary = 0
//     let binary: u8 = match boolean {
//         true => 1,
//         false => 0
//     };

//     assert_eq!(binary, 1);

//     println!("Success!");
// }



// Fill in the blanks
// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// fn main() {
//     let msgs: [Message; 3] = [
//         Message::Quit,
//         Message::Move{x:1, y:3},
//         Message::ChangeColor(255,255,0)
//     ];

//     for msg in msgs {
//         show_message(msg)
//     }

//     println!("Success!");
// } 

// fn show_message(msg: Message) {
//     match msg {
//         Message::Move { x:a, y: b} => { // match  Message::Move
//             assert_eq!(a, 1);
//             assert_eq!(b, 3);
//         },
//         Message::ChangeColor(r, g, b) => {
//             assert_eq!(g, 255);
//             assert_eq!(b, 0);
//         }
//         __ => println!("no data in these variants")
//     }
// }


// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

//     // Fill the blank with `matches!` to make the code work
//     for ab in alphabets {
//         assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
//     }

//     println!("Success!");
// } 


// enum MyEnum {
//     Foo,
//     Bar
// }

// fn main() {
//     let mut count = 0;

//     let v: Vec<MyEnum> = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];

//     for e in v {
//         // if matches!(e, MyEnum::Foo) {
//         //     count += 1;
//         // }

//         // A different approach
//         match e {
//             MyEnum::Foo => {
//                 count +=1 ;
//             },
//             _ => {
//                 println!("No matches");
//             }

//         }
//     }

//     assert_eq!(count, 2);

//     println!("Success!");
// }


// fn main() {
//     let o : Option<i32> = Some(7);

//     // Remove the whole `match` block, using `if let` instead 
//     match o {
//         Some(i) => {
//             println!("This is a really long string and `{:?}`", i);

//             println!("Success!");
//         }
//         _ => {}
//     };

//     if let Some(i) = o {
//         println!("This is a really long string and `{:?}`", i);

//         println!("Success!");
//     };
// }


// Fill in the blank
// enum Foo {
//     Bar(u8)
// }

// fn main() {
//     let a = Foo::Bar(1);

//     if let Foo::Bar(i) = a {
//         println!("foobar holds the value: {}", i);

//         println!("Success!");
//     }
// }


// enum Foo {
//     Bar,
//     Baz,
//     Qux(u32)
// }

// fn main() {
//     let a : Foo = Foo::Qux(10);

//     // Remove the codes below, using `match` instead 
//     if let Foo::Bar = a {
//         println!("match foo::bar")
//     } else if let Foo::Baz = a {
//         println!("match foo::baz")
//     } else {
//         println!("match others")
//     }

//     match a {
//         Foo::Bar => {
//             println!("Bar");
//         },
//         Foo::Baz => {
//             println!("Baz");
//         },
//         Foo::Qux(i) => {
//             println!("match others");
//             println!("Qux : {:?}", i);
//         }
//     }
// }

// shadowing

// // Fix the errors in-place
// fn main() {
//     let age : Option<i32> = Some(30);
//     if let Some(age) = age { // Create a new variable with the same name as previous `age`
//        assert_eq!(age, 30);
//     } // The new variable `age` goes out of scope here
    
//     match age {
//         // Match can also introduce a new shadowed variable
//         Some(age) =>  println!("age is a new variable, it's value is {}",age),
//         _ => ()
//     }
//  }


// fn main() { 
//     match_number(10);
// }
// fn match_number(n: i32) {
//     match n {
//         // Match a single value
//         1 => println!("One!"),
//         // Fill in the blank with `|`, DON'T use `..` or `..=`
//         2 | 3 | 4 | 5 => println!("match 2 -> 5"),
//         // Match an inclusive range
//         6..=10 => {
//             println!("match 6 -> 10")
//         },
//         _ => {
//             println!("match -infinite -> 0 or 11 -> +infinite")
//         }
//     }
// }


// struct Point {
//     x: i32,
//     y: i32,
// }

// // The @ operator lets us create a variable that holds a value, 
// // at the same time we are testing that value to see whether it matches a pattern. 
// fn main() {
//     // Fill in the blank to let p match the second arm
//     let p: Point = Point { x: 3, y: 13 };

//     // This is how we match a struct
//     match p {
//         Point { x: x, y: 0 } => println!("On the x axis at {}", x),
//         // Second arm
//         Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
//         Point { x: x, y: y } => println!("On neither axis: ({}, {})", x, y),
//     }
// }


// // Fix the errors
// enum Message {
//     Hello { id: i32 },
// }

// fn main() {
//     let msg: Message = Message::Hello { id: 5 };

//     match msg {
//         Message::Hello {
//             id: id @ (3..=7), // @ is used to de-structure the possible values that it can take into a variable
//         } => println!("Found an id in range [3, 7]: {}", id),
//         Message::Hello { id: newid @ (10 | 11 | 12) } => {
//             println!("Found an id in another range [10, 12]: {}", newid)
//         }
//         Message::Hello { id: id } => println!("Found some other id: {}", id),
//     }
// }


// // Fill in the blank to make the code work, `split` MUST be used
// fn main() {
//     let num: Option<i32> = Some(4);
//     let split: i32 = 5;
//     match num {
//         Some(x) if x < split  => assert!(x < split),
//         Some(x) => assert!(x >= split),
//         None => (),
//     }

//     println!("Success!");
// }


// // Fill the blank to make the code work
// fn main() {
//     let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

//     match numbers {
//         (first, .., last) => {
//            assert_eq!(first, 2);
//            assert_eq!(last, 2048);
//         }
//     }

//     println!("Success!");
// }


// FIX the error with least changing
// DON'T remove any code line
// fn main() {
//     let mut v: String = String::from("hello,");
//     let r: &mut String = &mut v;

//     match r {
//        value => value.push_str(" world!")  // When we are matching, it would automatically be a mutable reference to value.
//     }
// }

// Methods - functions that is associated with a particular type or struct
// Takes parameters and returns a value, but defined as a member of a struct or enum
// Implemented through an "impl" block and called using dot notation

// Example
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     // let rec1: Rectangle = Rectangle { // normal instantiation
//     //     width: 10,
//     //     height: 40
//     // };
//     let rec1 = Rectangle::new(20, 30); // associated function related call
//     println!("The area is {:?}", rec1.area());
// }

// Associated Functions - function that is associated with a struct or an enum, but doesn't take an instance as its first parameter.
// Called using the name of the type, not an instance of it.
// often used as constructores for a struct or enum.

// associated function example

// impl Rectangle {
//     fn new(width: u32, height: u32) -> Rectangle {
//         Rectangle {
//             width: width,
//             height: height
//         }
//     }
// }

// IMPORTANT
// self will take the ownership of current struct instance, however, &self will only borrow a reference from the instance.

// Only fill in the blanks, DON'T remove any line!
// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     pub fn show_state(self: &Self)  {
//         println!("the current state is {}", self.color);
//     }
// }
// fn main() {
//     let light = TrafficLight{
//         color: String::from("red"),
//     };
//     // Don't take the ownership of `light` here.
//     light.show_state();
//     // ... Otherwise, there will be an error below
//     println!("{:?}", light);
// }

// #[derive(Debug)]
// struct TrafficLight {
//     color: String,
// }

// impl TrafficLight {
//     // 1. Implement an associated function `new`,
//     // 2. It will return a TrafficLight contains color "red"
//     // 3. Must use `Self`, DONT use `TrafficLight` in fn signatures or body
//     pub fn new() -> Self { // Self in impl refers to the TrafficLight itself. We can give `TrafficLight` as return type also
//         TrafficLight {
//             color: "red".to_owned()
//         }
//     }

//     pub fn get_state(&self) -> &str {
//         &self.color
//     }
// }

// fn main() {
//     let light = TrafficLight::new();
//     assert_eq!(light.get_state(), "red");

//     println!("Success!");
// }

// #[derive(Debug)]
// enum TrafficLightColor {
//     Red,
//     Yellow,
//     Green,
// }

// // Implement TrafficLightColor with a method.
// impl TrafficLightColor {
//     fn color(&self) -> &str {
//         match self {
//             Self::Yellow => "Yellow", // Here `Self` refers to the TrafficLightColor itself  
//             Self::Red => "Red",
//             Self::Green => "Green"
//         }
//     }
// }

// fn main() {
//     let c: TrafficLightColor = TrafficLightColor::Green;

//     assert_eq!(c.color(), "Green");

//     println!("{:?}",c);
// }

// Generics - placeholders for concrete types
// enables writing more reusable and flexible code
// Zero Cost Abstraction - Rust compiler will figure out types at compile time , and fill out gnerics with concerete types.

// Const Generic - Type Parameter that represents a compile-time constant value
// used for array sizes, bit widths and other constants


// // Fill in the blanks to make it work
// struct A;          // Concrete type `A`.
// struct S(A);       // Concrete type `S`.
// struct SGen<CC>(CC); // Generic type `SGen`.

// fn reg_fn(_s: S) {}

// fn gen_spec_t(_s: SGen<A>) {}

// fn gen_spec_i32(_s: SGen<i32>) {}

// fn generic<TABO>(_s: SGen<TABO>) {}

// fn main() {
//     // Using the non-generic functions
//     reg_fn(S(A));          // Concrete type.
//     gen_spec_t(SGen(A));   // Implicitly specified type parameter `A`.
//     gen_spec_i32(SGen(7i32)); // Implicitly specified type parameter `i32`.

//     // Explicitly specified type parameter `char` to `generic()`.
//     generic::<char>(SGen('A'));

//     // Implicitly specified type parameter `char` to `generic()`.
//     generic(SGen(7.9));

//     println!("Success!");
// }

// Implement the generic function below.
// fn sum<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
//     return a + b;
// }

// fn main() {
//     assert_eq!(5, sum(2i8, 3i8));
//     assert_eq!(50, sum(20, 30));
//     assert_eq!(2.46, sum(1.23, 1.23));

//     println!("Success!");
// }



// Implement struct Point to make it work.
// struct Point<T> {
//     x: T,
//     y: T
// }

// fn main() {
//     let integer: Point<i32> = Point { x: 5, y: 10 };
//     let float: Point<f64> = Point { x: 1.0, y: 4.0 };

//     println!("Success!");
// }



// Modify this struct to make the code work
// struct Point<T, Y> {
//     x: T,
//     y: Y,
// }

// fn main() {
//     // DON'T modify this code.
//     let p: Point<i32, String> = Point{x: 5, y : "hello".to_string()};

//     println!("Success!");
// }


// // Add generic for Val to make the code work, DON'T modify the code in `main`.
// struct Val<T> {
//     val: T,
// }

// impl<T> Val<T> { // impl<T> allows T to be used inside the body
//     fn value(&self) -> &T {
//         &self.val
//     }
// }


// fn main() {
//     let x: Val<f64> = Val{ val: 3.0 };
//     let y: Val<String> = Val{ val: "hello".to_string()};
//     println!("{}, {}", x.value(), y.value());
// }

// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     // Implement mixup to make it work, DON'T modify other code.
//     fn mixup<V, W>(self, other: Point<V,W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y
//         }
//     }
// }

// fn main() {
//     let p1: Point<i32, i32> = Point { x: 5, y: 10 };
//     let p2: Point<&str, char> = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');

//     println!("Success!");
// }


// Fix the errors to make the code work.
// struct Point<T> {
//     x: T,
//     y: T,
// }

// impl Point<f64> {
//     fn distance_from_origin(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p: Point<f64> = Point{x: 5.0, y: 10.0};
//     println!("{}",p.distance_from_origin());
// }


// Trait - set of methods that can be implemented for multiple types in order to provide 
// common functionality and behaviour between them

// Traits consist of method signatures only, which then have to be implemented by the target type

// Traits are similar to "classes" in other languages, not quite the same though

// Traits defined shared beviour in an abstract way

// Example

// trait Animal {
//     fn sound(&self) -> String;
// }

// struct Sheep;
// struct Cow;

// impl Animal for Sheep {
//     fn sound(&self) -> String {
//         String::from("Maaah!")
//     }
// }

// impl Animal for Cow {
//     fn sound(&self) -> String {
//         String::from("Moooh!")
//     }
// }

// fn main() {
//     let ani: Cow = Cow;
//     println!("Sound is {}", ani.sound());
// }

// Derivable Traits - Trait that can be automatically implemented for a struct or an enum by the Rust compiler
// Called "derivable" because they can be derived automatically
// Most common derivable traits: Debug - allow to output via {:?}, 
// Clone - enables type to be duplicated with clone() method , 
// Copy - enables type to be copied implicitly, 
// PartialEq - enables comparison

// Traits can be used as parameters for functions
// Example

// Here the function notify() will only take an argument of a type that has implemented the Summary Trait
// pub fn notify(item: &impl Summary) {
//      println!("Breaking news! {}", item.summarize());
// }

// Trait bounds - declared like generics, after the name of the function.
// use trait bounds if you have lots of parameters.

// pub fn notify(item1: &impl Summary, item2: &impl Summary){}
// same as the below
// pub fn notify<T: Summary>(item1: &T, item2: &T){} // says that generic type T should implement Summary trait


// // Fill in the two impl blocks to make the code work.
// // DON'T modify the code in `main`.
// trait Hello {
//     // Default implementation for function say_hi
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }

//     fn say_something(&self) -> String;
// }

// struct Student {}
// impl Hello for Student {
//     fn say_hi(&self) -> String {
//         String::from("Hi")
//     }

//     fn say_something(&self) -> String {
//         String::from("I'm a good student")
//     }
// }
// struct Teacher {}
// impl Hello for Teacher {
//     fn say_something(&self) -> String {
//         String::from("I'm not a bad teacher")
//     }
// }

// fn main() {
//     let s = Student {};
//     assert_eq!(s.say_hi(), "Hi");
//     assert_eq!(s.say_something(), "I'm a good student");

//     let t = Teacher {};
//     assert_eq!(t.say_hi(), "hi");
//     assert_eq!(t.say_something(), "I'm not a bad teacher");

//     println!("Success!");
// }


// use std::ops;

// Implement fn multiply to make the code work.
// As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
// So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/
// fn multiply<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
//     a * b
// }

// fn main() {
//     assert_eq!(6, multiply(2u8, 3u8));
//     assert_eq!(5.0, multiply(1.0, 5.0));

//     println!("Success!");
// }


// Implement `fn summary` to make the code work.
// Fix the errors without removing any code line
// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post: Post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo: Weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };

//     summary(&post);
//     summary(&weibo);

//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }

// // Implement `fn summary` below.
// // fn summary(a: &impl Summary) { // Both the ways are possible to be declare when defining the function
// fn summary<T: Summary>(a: &T) {
//     let output:String = a.summarize();
//     println!("Output: {}", output);
// }

// Trait Objects
// using `impl Trait` doesn't work when returning multiple types
// Different implementations of a trait probably use different amounts of memory, but sizes of types must be know at 
// compile time. - In this case, `trait objects` can be used.

// A trait object is essentially a pointer to any type that implements the given trait, where the precise type can only
// Be known at compile time.

// Static Dispatch - resolves a method calls at compile time.
// Compiler generates function code for each concrete type that implements traits
// Calls appropriate function based on concrete types
// Faster and more efficient than dynamic dispatch, but doesn't provide great flexibility

// Dynamic Dispatch - specific methods to be called is dtermined at runtime and not at compile time.
// Works by creating a reference or smart pointer to a trait object using `&dyn` or `Box<dyn >`
// When trait object is created, compiler will build a vtable for the trait.
// vtable is a table that contains a pointer to the implementation of each method in the trait for the specific 
// type of the object that references points to - compiler will do a lookup in a vtable to determine which method should be
// called for which type that implements the given trait.
// This lookup will cause overhead buy allows for more flexible code.

// Example

// trait Animal {
//     fn noise(&self);
// }

// struct Cat;
// struct Dog;

// impl Animal for Cat {
//     fn noise(&self) -> () {
//         println!("Meow!");
//     }
// }

// impl Animal for Dog {
//     fn noise(&self) -> () {
//         println!("Woff");
//     }
// }

// fn random_animal(random_animal: u8) -> Box<dyn Animal> {
//     if(random_animal < 10) {
//         Box::new(Cat {})
//     } else {
//         Box::new(Dog {})
//     }
// }

// fn main() {
//     let random_number = 14;
//     let animal = random_animal(random_number);
//     animal.noise();
// }

// Box - smart pointer that allows to store data on the heap rather than the stack
// Use Box when you have a type whose size can't be known at compile time
// Returns a pointer to the data stored on the heap

// Difference between `&`(reference) and Box
// Memory : Box allocates data on heap and owns it, also responsible for deallocating when value goes out of scope,
// Reference only points to a value already in memory

// Lifetime: Box can be passed across scopes, reference has limited lifetime.

// Box can be clonse, reference not.
// Box can be used in pattern matching.

// FIX the errors.
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x,
//             y,
//         }
//     }
// }

// impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
// #[derive(Debug, PartialOrd, PartialEq)]
// struct Unit(i32);

// fn main() {
//     // let pair: Pair<Unit> = Pair{
//     //     x: Unit(1),
//     //     y: Unit(3)
//     // };
//     let pair: Pair<Unit> = Pair::new(Unit(1), Unit(3));

//     pair.cmp_display();
// }

//  Associated Types - allows to specify a type that is associated with the trait.
// When implementing the trait for a specific type we have to specify the concrete type - basically a placeholder that
// the trait methods can use in their signature.
// Similar to generic types but are more flexible because they allow a trait to have different associated types for different
// implementing types.

// trait Bird {
//     fn quack(&self) -> String;
// }

// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String{
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String{
//         "swan swan".to_string()
//     }
// }

// fn main() {
//     // FILL in the blank.
//     let duck: Duck = Duck;
//     duck.swim();

//     let bird: Box<dyn Bird> = hatch_a_bird(2);
//     // This bird has forgotten how to swim, so below line will cause an error.
//     // bird.swim();
//     // But it can quak.
//     assert_eq!(bird.quack(), "duck duck");

//     let bird: Box<dyn Bird> = hatch_a_bird(1);
//     // This bird has forgotten how to fly, so below line will cause an error.
//     // bird.fly();
//     // But it can quak too.
//     assert_eq!(bird.quack(), "swan swan");

//     println!("Success!");
// }

// // IMPLEMENT this function.
// // This functions returns an Trait Object
// fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
//     match species {
//         1 => Box::new(Swan),
//         2 => Box::new(Duck),
//         _ => panic!(),
//     }
// }

// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }

// fn main() {
//     // FILL in the blank to make the code work.
//     // We either needs to implement it via Box or provide the references to the same.
//     // let birds: [Box<dyn Bird>; 3] = [Box::new(Duck), Box::new(Duck), Box::new(Swan)];
//     let birds: [&dyn Bird; 2] = [&Duck, &Swan];

//     for bird in birds {
//         bird.quack();
//         // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
//         // So, the code below will cause an error.
//         // bird.fly();
//     }
// }

// FILL in the blanks.
// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }

// fn main() {
//     let x: f64 = 1.1f64;
//     let y: u8 = 8u8;

//     // Draw x.
//     draw_with_box(Box::new(x));

//     // Draw y.
//     draw_with_ref(&y);

//     println!("Success!");
// }

// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw_with_ref(x: &dyn Draw) {
//     x.draw();
// }


// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String { format!("u8: {}", *self) }
// }

// impl Foo for String {
//     fn method(&self) -> String { format!("string: {}", *self) }
// }

// // IMPLEMENT below with generics.
// fn static_dispatch<T: Foo>(a: T) {
//     a.method();
// }

// // Implement below with trait objects.
// // A pointer is always of the size - usize
// fn dynamic_dispatch(a: &dyn Foo) {
//     a.method();
// }

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success!");
// }
// 

// FILL in the blanks
// fn main() {
//     let mut s: String = String::from("hello, world");
 
//     let slice1: &str = &s; // In two ways
//     assert_eq!(slice1, "hello, world");
 
//     let slice2: &str = &s[..5];
//     assert_eq!(slice2, "hello");
 
//     let slice3: &mut String = &mut s; 
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");
 
//     println!("Success!");
// }



// Question: how many heap allocations are happening here? 
// Your answer: 2
// fn main() {  
//     // Create a String type based on `&str`
//     // The type of string literals is `&str`
//    let s: String = String::from("hello, world!"); // Heap allocation happening here, because of using String

//    // Create a slice point to String `s`
//    let slice: &str = &s; // Just a reference, no new allocation.

//    // Create a String type based on the recently created slice
//    let s: String = slice.to_string(); // Heap allocation happening

//    assert_eq!(s, "hello, world!");

//    println!("Success!");
// }



// FILL in the blank and FIX errors
// fn main() {
//     let s: String = String::from("hello, 世界");
//     let slice1: &str = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
//     assert_eq!(slice1, "h");

//     let slice2: &str = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
//     assert_eq!(slice2, "世");
    
//     // Iterate through all chars in s
//     for (i, c) in s.chars().enumerate() {
//         if i == 7 {
//             assert_eq!(c, '世')
//         }
//     }

//     println!("Success!");
// }

// FILL in the blanks
// fn main() {
//     let mut s: String = String::new(); // String = Vec<u8> String is a vector of u8 behind the hood
//     s.push_str("hello");

//     // Some bytes, in a vector
//     let v: Vec<u8> = vec![104, 101, 108, 108, 111];

//     // Turn a byte's vector into a String
//     // String is just a vector of bytes.
//     let s1: String = String::from_utf8(v).unwrap();
//     assert_eq!(s, s1);

//     println!("Success!");
// } 


// Modify the code below to print out: 
// 25
// 25
// 25
// Here, there’s no need to allocate more memory inside the loop.
// fn main() {
//     // let mut s = String::new();
//     let mut s = String::with_capacity(25);

//     println!("{}", s.capacity());

//     for _ in 0..2 {
//         s.push_str("hello");
//         println!("{}", s.capacity());
//     }

//     println!("Success!");
// }

// Vectors - Like array but dunamically sized - it can grow and shrink
// Allocated on the heap as contiguous block of memory
// All elements must have the same type

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];
    
//     let v: Vec<u8> = Vec::from(arr);
//     is_vec(&v);

//     let v: Vec<u8> = vec![1, 2, 3];
//     is_vec(&v);

//     // vec!(..) and vec![..] are same macros, so
//     let v: Vec<u8> = vec!(1, 2, 3);
//     is_vec(&v);
    
//     // In code below, v is Vec<[u8; 3]> , not Vec<u8>
//     // USE Vec::new and `for` to rewrite the below code 
//     let mut v1 = Vec::new();
//     // let v1: Vec<[u8; 3]> = vec!(arr);

//     for i in &v {
//         v1.push(*i); // Need to de-reference the iterator by using points since we are taking a reference to v.
//     }

//     is_vec(&v1);
 
//     assert_eq!(v, v1);

//     println!("Success!");
// }

// fn is_vec(v: &Vec<u8>) {}



// FILL in the blank
// fn main() {
//     let mut v1: Vec<i32> = Vec::from([1, 2, 4]);
//     v1.pop(); // [1, 2]
//     v1.push(3); // [1, 2, 3]
    
//     let mut v2: Vec<i32> = Vec::new();
//     v2.extend(v1.clone());

//     assert_eq!(v1, v2);

//     println!("Success!");
// }



// FILL in the blanks
// fn main() {
//     // Array -> Vec
//     // impl From<[T; N]> for Vec
//     let arr: [i32; 3] = [1, 2, 3];
//     let v1: Vec<i32> = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();
 
//     assert_eq!(v1, v2);
 
    
//     // String -> Vec
//     // impl From<String> for Vec
//     let s: String = "hello".to_string();
//     let v1: Vec<u8> = s.into();

//     let s: String = "hello".to_string();
//     let v2: Vec<u8> = s.into_bytes();
//     assert_eq!(v1, v2);

//     // impl<'_> From<&'_ str> for Vec
//     let s: &str = "hello";
//     let v3: Vec<u8> = Vec::from(s);
//     assert_eq!(v2, v3);

//     // Iterators can be collected into vectors
//     let v4: Vec<i32> = [0; 10].into_iter().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success!");
// }


// FIX the error and IMPLEMENT the code
// fn main() {
//     let mut v = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v.get(i)); // get will return an Option<i32>, so it's safe
//     }

//     for i in 0..5 {
//         // IMPLEMENT the code here...
//         match v.get(i) {
//             Some(e) => v[i] =  e + 1,
//             None => v.push(i+2)
//         }
//     }
    
//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!");
// }

// Slicing - A vec can be mutable. slices are read-only objects.
// to get a slice, use &
// It's more common to pass slices as arguments than vectors, (same applies for String & &str)


// FIX the errors
// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];

//     let slice1: &[i32] = &v[..]; // .. - indicates from start till end.
//     // Out of bounds will cause a panic
//     // You must use `v.len` here
//     let slice2: &[i32] = &v[0..v.len()]; // here its excluding v.len() - 1.
    
//     assert_eq!(slice1, slice2);
    
//     // Slices are read only
//     // Note: slice and &Vec are different
//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3: &[i32] = &v[0..v.len()];
//     // slice3.push(4); - cannot mutate a slice.

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success!");
// }

// Capacity - If a vector’s length exceeds its capacity, its capacity will automatically be increased, but its elements will have to be reallocated.

// FIX the errors
// fn main() {
//     let mut vec = Vec::with_capacity(10);

//     // The vector contains no items, even though it has capacity for more
//     assert_eq!(vec.len(), 0); // vec doesn't hold any values
//     assert_eq!(vec.capacity(), 10);

//     // These are all done without reallocating...
//     for i in 0..10 {
//         vec.push(i);
//     }
//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);

//     // ...but this may make the vector reallocate
//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);


//     // Fill in an appropriate value to make the `for` done without reallocating 
//     let mut vec = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 100);
    
//     println!("Success!");
// }

// #[derive(Debug,PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
// fn main() {
//     // FILL in the blank
//     let v : Vec<IpAddr>= vec![
//         IpAddr::V4("127.0.0.1".to_string()),
//         IpAddr::V6("::1".to_string())
//     ];
    
//     // Comparing two enums need to derive the PartialEq trait
//     assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
//     assert_eq!(v[1], IpAddr::V6("::1".to_string()));

//     println!("Success!");
// }

// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4: {:?}",self.0)
//     }
// }
// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6: {:?}",self.0)
//     }
// }

// fn main() {
//     // FILL in the blank
//     let v: Vec<Box<dyn IpAddr>>= vec![
//         Box::new(V4("127.0.0.1".to_string())),
//         Box::new(V6("::1".to_string())),
//     ];

//     for ip in v {
//         ip.display();
//     }
// }

// HashMap - Data structure to store key-value pairs.
// Allocated on the heap as it is dynamically sized, can grow and shrink.
// Allows for efficient lookup, insertion and deletion of data.
// Each key is hashed to a unique index in underlying array.
// In HashMaps order of elements won't be retained


// FILL in the blanks and FIX the errors
// use std::collections::HashMap;
// fn main() {
//     // All keys and values of the HashMap should be of the same type.

//     let mut scores: HashMap<&str, i32> = HashMap::new();
//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69);
//     scores.insert("Katie", 58);

//     // Get returns an Option<&V>
//     let score: Option<&i32> = scores.get("Sunface");
//     assert_eq!(score, Some(&98));

//     if scores.contains_key("Daniel") {
//         // Indexing returns a value V
//         let score: i32 = scores["Daniel"];
//         assert_eq!(score, 95);
//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), 3);

//     for (name, score) in scores {
//         println!("The score of {} is {}", name, score);
//     }
// }

// use std::collections::HashMap;
// fn main() {
//     let teams: [(&str, i32); 3] = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];

//     let mut teams_map1: HashMap<&str, i32> = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }

//     // IMPLEMENT team_map2 in two ways
//     // Tips: one of the approaches is to use `collect` method
//     // let teams_map2: HashMap<&str, i32> = HashMap::from(teams); // This will directly convert the tuple into HashMap
    
//     let teams_map2: HashMap<&str, i32> = teams.into_iter().collect(); // iterator will collect the values into the return type denoted.

//     assert_eq!(teams_map1, teams_map2);

//     println!("Success!");
// }



// FILL in the blanks
// use std::collections::HashMap;
// fn main() {
//     // Type inference lets us omit an explicit type signature (which
//     // would be `HashMap<&str, u8>` in this example).
//     let mut player_stats = HashMap::new();

//     // Insert a key only if it doesn't already exist
//     player_stats.entry("health").or_insert(100);

//     assert_eq!(player_stats["health"], 100);

//     // Insert a key using a function that provides a new value only if it
//     // doesn't already exist
//     player_stats.entry("health").or_insert_with(random_stat_buff);
//     assert_eq!(player_stats["health"], 100);

//     // Ensures a value is in the entry by inserting the default if empty, and returns
//     // a mutable reference to the value in the entry.
//     let health: &mut u8 = player_stats.entry("health").or_insert(50);
//     assert_eq!(health, &100);
//     *health -= 50;
//     assert_eq!(*health, 50);

//     println!("Success!");
// }

// fn random_stat_buff() -> u8 {
//     // Could actually return some random value here - let's just return
//     // some fixed value for now
//     42
// }


// FIX the errors
// Tips: `derive` is usually a good way to implement some common used traits
// use std::collections::HashMap;

// #[derive(Debug, Eq, PartialEq, Hash)]
// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     /// Creates a new Viking.
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// fn main() {
//     // Use a HashMap to store the vikings' health points.
//     let vikings: HashMap<Viking, i32> = HashMap::from([
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ]);

//     // Use derived implementation to print the status of the vikings.
//     for (viking, health) in &vikings {
//         println!("{:?} has {} hp", viking, health);
//     }
// }

// Ownership in HashMaps.
// For types that implement the Copy trait, like i32 , the values are copied into HashMap. 
// For owned values like String, the values will be moved and HashMap will be the owner of those values.

// FIX the errors with least changes
// DON'T remove any code line
// use std::collections::HashMap;

// fn main() {
//     let v1 = 10;
//     let mut m1: HashMap<i32, i32> = HashMap::new();
//     m1.insert(v1, v1);
//     println!("v1 is still usable after inserting to hashmap : {}", v1);

//     let v2 = "hello".to_string();
//     let mut m2: HashMap<&str, i32> = HashMap::new();
//     // Ownership moved here
//     m2.insert(&v2, v1);

//     assert_eq!(v2, "hello");
//     println!("v1 : {}", v1); // still possible to print v1 because ownership did not move!
//     println!("Success!");
// }


// Type Coercion - as
// Type convertion also called type casting is coercing primitive types that can be performed by `as` keyword
// as conversions can be chained.
// Using unsafe methods can lead to undefined behaviour.
// When casting to an unsigned type, T, T::MAX + 1 is added or subtracted until the value fits into the new type.

// FIX the errors and FILL in the blank
// DON'T remove any code
// fn main() {
//     let decimal: f32 = 97.123_f32;

//     let integer: u8 = decimal as u8;

//     let c1: char = decimal as u8 as char;
//     let c2: char = integer as char;

//     assert_eq!(integer, 'a' as u8);

//     println!("Success!");
// }

// #[allow(overflowing_literals)]
// fn main() {
//     assert_eq!(u8::MAX, 255);
//     // The max of `u8` is 255 as shown above.
//     // so the below code will cause an overflow error: literal out of range for `u8`.
//     // PLEASE looking for clues within compile errors to FIX it.
//     // DON'T modify any code in main.
//     let v = 1000 as u8; // 1000 - 256 = 744 - 256 = 488 - 256 = 232
//     println!("{} ", v); // 232
//     println!("Success!");
// }

// #[allow(overflowing_literals)]
// fn main() {
//     assert_eq!(1000 as u16, 1000);

//     assert_eq!(1000 as u8, 232);

//     // For positive numbers, this is the same as the modulus
//     println!("1000 mod 256 is : {}", 1000 % 256);

//     assert_eq!(-1_i8 as u8, 255); // 256 - 1 = 255
    
//     // Since Rust 1.45, the `as` keyword performs a *saturating cast* 
//     // when casting from float to int. If the floating point value exceeds 
//     // the upper bound or is less than the lower bound, the returned value 
//     // will be equal to the bound crossed.
//     assert_eq!(300.1_f32 as u8, 255);
//     assert_eq!(-100.1_f32 as u8, 0);
    

//     // This behavior incurs a small runtime cost and can be avoided 
//     // with unsafe methods, however the results might overflow and 
//     // return **unsound values**. Use these methods wisely:
//     unsafe {
//         // 300.0 is 44
//         println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
//         // -100.0 as u8 is 156
//         println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
//         // nan as u8 is 0
//         println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
//     }
// }

// From/Into conversion
// `From` and `Into` traits are used for type conversions between different types without requiring explicit casts.
// Part of standard library
// Can be implemented for custom types.
// Implementing `From` for a type will give us `Into` implementation for the given type for free!

// fn main() {
//     // impl From<bool> for i32
//     let i1: i32 = false.into(); // 0
//     let i2: i32 = i32::from(false); // 0
//     assert_eq!(i1, i2);
//     assert_eq!(i1, 0);

//     // FIX the error in two ways
//     /* 1. use a similar type which `impl From<char>`, maybe you
//     should check the docs mentioned above to find the answer */
//     // 2. a keyword from the last chapter
//     let i3: u32 = 'a'.into(); // not implemented for i32.

//     // FIX the error in two ways
//     // let s: String = 'a' as String;
//     let s: String = String::from('a');
//     // if from is implemented, into is automatically implemented for the same.
//     let s: String = 'a'.into();

//     println!("Success!");
// }

// From is now included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::From;

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     // IMPLEMENT `from` method
//     fn from(i: i32) -> Number {
//         Number {
//             value: i
//         }
//     }
// }

// // FILL in the blanks
// fn main() {
//     let num = Number::from(30);
//     assert_eq!(num.value, 30);

//     let num: Number = 30.into() ;
//     assert_eq!(num.value, 30);

//     println!("Success!");
// }


// TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
// use std::convert::TryInto;

// fn main() {
//     let n: i16 = 256;

//     // Into trait has a method `into`,
//     // hence TryInto has a method ?
//     let n: u8 = match n.try_into() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("there is an error when converting: {:?}, but we catch it", e.to_string());
//             0
//         }
//     };

//     assert_eq!(n, 0);

//     println!("Success!");
// }

// #[derive(Debug, PartialEq)]
// struct EvenNum(i32);

// impl TryFrom<i32> for EvenNum {
//     type Error = ();

//     // IMPLEMENT `try_from`
//     fn try_from(value: i32) -> Result<Self, Self::Error> { // This indicates that it will either return the result `EvenNum` or it will return the Error `()` unit type
//         if value % 2 == 0 {
//             Ok(EvenNum(value))
//         } else {
//             Err(())
//         }
//     }
// }

// fn main() {
//     assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
//     assert_eq!(EvenNum::try_from(5), Err(()));

//     // FILL in the blanks
//     let result: Result<EvenNum, ()> = 8i32.try_into();
//     assert_eq!(result, Ok(EvenNum(8)));
//     let result: Result<EvenNum, ()> = 5i32.try_into();
//     assert_eq!(result, Err(()));

//     println!("Success!");
// }

// Convert any type to String
// To convert any type to String, you can simply use the ToString trait for that type

// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "The point is ({}, {})", self.x, self.y) // write! macro writes to a fmt buffer `f`
//     }
// }

// fn main() {
//     let origin: Point = Point { x: 0, y: 0 };
//     // FILL in the blanks
//     assert_eq!(origin.to_string(), "The point is (0, 0)");
//     assert_eq!(format!("{}", origin), "The point is (0, 0)");

//     println!("Success!");
// }

// panic! - simplest method to end the program.
// it will print out an error message, unwind the stack and exit the program.
// NOTE: in multithreaded env, it will end the thread where panic! occured, not the entire program.

  
// FILL the blanks
// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success!");
//         // IMPLEMENT the below code
//         panic!()
//      }

//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     drink("lemonade");

//     println!("Exercise Failed if printing out this line!");
// }

// MAKE the code work by fixing all panics
// fn main() {
//     println!("{:?}", "abc".as_bytes());
//     assert_eq!("abc".as_bytes(), [97, 98, 99]);

//     let v: Vec<i32> = vec![1, 2, 3]; 
//     let ele: i32 = v[2];
//     // unwrap may panic when get return a None
//     let ele: i32 = *v.get(1).unwrap(); // Some(2) -> get will get a reference to the inner value. so use *v to get value or &i32 as return type.

//     // Sometimes, the compiler is unable to find the overflow errors for you in compile time ,so a panic will occur
//     let v = production_rate_per_hour(2);

//     // because of the same reason as above, we have to wrap it in a function to make the panic occur
//     divide(15, 1);

//     println!("Success!")
// }

// fn divide(x:u8, y:u8) {
//     println!("{}", x / y)
// }

// fn production_rate_per_hour(speed: u16) -> f64 {
//     let cph: u16 = 221;
//     match speed {
//         1..=4 => (speed * cph) as f64,
//         5..=8 => (speed * cph) as f64 * 0.9,
//         9..=10 => (speed * cph) as f64 * 0.77,
//         _ => 0 as f64,
//     }
// }

// pub fn working_items_per_minute(speed: u16) -> u32 {
//     (production_rate_per_hour(speed) / 60 as f64) as u32
// }

// Result - `Result` is an enum type that represents the outcome of an operation that could potential fail.
// Two possible variants:
// Ok(T) - A value T was found // Expect outcome is `Ok`
// Err(e) - an error was found with value e // Unexpected outcome is `Err`
// Since `Result` is an `enum`, it can be matched with match pattern

// unwrap - takes an input of type Result and takes out the value which is wrapped inside Ok(T) in case of success
// or panics in case of Error. 


// FILL in the blanks and FIX the errors
// use std::num::ParseIntError;

// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     let n1: Result<i32, ParseIntError> = n1_str.parse::<i32>();
//     let n2: Result<i32, ParseIntError> = n2_str.parse::<i32>();
//     Ok(n1.unwrap() * n2.unwrap())
// }

// fn main() {
//     let result = multiply("10", "2");
//     assert_eq!(result, Ok(20));

//     let result = multiply("4", "2");
//     assert_eq!(result.unwrap(),  8);

//     println!("Success!");
// }


// use std::num::ParseIntError;

// // IMPLEMENT multiply with ?
// // DON'T use unwrap here
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     let n1 = n1_str.parse::<i32>()?;
//     let n2 = n2_str.parse::<i32>()?; // parse returns a Result. `?` will unwrap the result or throw err automatically.

//     Ok(n1 * n2)
// }

// fn main() {
//     assert_eq!(multiply("3", "4").unwrap(), 12);
//     println!("Success!");
// }


// use std::fs::File;
// use std::io::{self, Read};

// fn read_file1() -> Result<String, io::Error> {
//     let f: Result<File, io::Error> = File::open("hello.txt");
//     let mut f: File = match f { 
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s: String = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// // FILL in the blanks with one code line
// // DON'T change any code lines
// fn read_file2() -> Result<String, io::Error> {
//     let mut s = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn main() {
//     assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
//     println!("Success!");
// }

// map and and_then are two common combinators for Result<T, E> (also for Option<T>).
 
// use std::num::ParseIntError;

// // FILL in the blank in two ways: map, and then
// fn add_two(n_str: &str) -> Result<i32, ParseIntError> {
// //    n_str.parse::<i32>().map(|n| n+2)
//    n_str.parse::<i32>().and_then(|n| Ok(n+2)) // and_then returns a Result whereas map unwraps and returns the value. that's the only differences.
// }

// fn main() {
//     assert_eq!(add_two("4").unwrap(), 6);

//     println!("Success!");
// }


// use std::num::ParseIntError;

// // With the return type rewritten, we use pattern matching without `unwrap()`.
// // But it's so Verbose...
// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     match n1_str.parse::<i32>() {
//         Ok(n1)  => {
//             match n2_str.parse::<i32>() {
//                 Ok(n2)  => {
//                     Ok(n1 * n2)
//                 },
//                 Err(e) => Err(e),
//             }
//         },
//         Err(e) => Err(e),
//     }
// }

// // Rewriting `multiply` to make it succinct
// // You should use BOTH of  `and_then` and `map` here.
// fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     // IMPLEMENT...
//     n1_str.parse::<i32>() // Ok(10)
//      .and_then( // 10 is unwrapped here automatically
//         |n1| n2_str.parse::<i32>().map(|n2| n1 * n2)
//     )
// }

// fn print(result: Result<i32, ParseIntError>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn main() {
//     // This still presents a reasonable answer.
//     let twenty = multiply1("10", "2");
//     print(twenty);

//     // The following now provides a much more helpful error message.
//     let tt = multiply("4", "2");
//     print(tt);

//     println!("Success!");
// }

// use std::num::ParseIntError;

// // FILL in the blank
// type Res<i32> = Result<i32, ParseIntError>;

// // Use the above alias to refer to our specific `Result` type.
// fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
//     first_number_str.parse::<i32>().and_then(|first_number| {
//         second_number_str.parse::<i32>().map(|second_number| first_number * second_number)
//     })
// }

// // Here, the alias again allows us to save some space.
// fn print(result: Res<i32>) {
//     match result {
//         Ok(n)  => println!("n is {}", n),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn main() {
//     print(multiply("10", "2"));
//     print(multiply("t", "2"));

//     println!("Success!");
// }

// use std::num::ParseIntError;

// fn main() -> Result<(), ParseIntError> {
//     let t: &str = "10";
//     let result: i32 = match t.parse::<i32>() {
//         Ok(number) => number,
//         Err(e) => return Err(e)
//     };
//     let result2: i32 = t.parse::<i32>()?; // same as the above statement
//     println!("{}", result);
//     Ok(())
// } 

// use std::fmt;

// struct Structure(i32);

// struct Deep(Structure);

// impl fmt::Debug for Deep {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0.0)
//     }
// }

// fn main() {    
//     // The problem with `derive` is there is no control over how
//     // the results look. What if I want this to just show a `7`?

//     /* Make it print: Now 7 will print! */
//     println!("Now {:?} will print!", Deep(Structure(7)));
// }


// Lifetimes - only necessary when dealing with references.
// another kind of generic which ensures that the references are valid as long as needed.
// Every reference has a lifetime, which is the scope of which that reference is valid.
// sometimes lifetime annotations are needed, if the compiler can't infer it automatically.

// Borrow Checker - compares scopes to determine whether all borrows are valid.
// Tracks lifetimes of references and ensures that they don't violate the ownership rules.
// Rules ensure that the value is not accessed once it has been moved or freed from the memory.
// NOTE: A reference to a value must not outlive the value itself.

 /* Make it work by adding proper lifetime annotation */
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { 
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let x: &str = "long";
//     let y: &str = "longer";

//     println!("{}", longest(y, x));
// }

// // Lifetime - example 2
// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }

// /* Fix function signature */
// fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType
// { foo.b }

// fn main()
// {
//     let no_copy: NoCopyType = NoCopyType {};
//     let example: Example = Example { a: &1, b: &no_copy };
//     fix_me(&example);
//     println!("Success!")
// }

/* Make it work by adding proper lifetime annotations */
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl ImportantExcerpt<'_> {
//     fn level<'a >(&'a self) -> i32 {
//         3
//     }
// }

// fn main() {}

// The 3 rules of Lifetime Ellison
// 1. Compiler assigns a lifetime to each parameter that's a reference
// 2. If there is exactly one input parameter that lifetime is assigned to all output lifetime parameters
// 3. if there are multiple lifetime parameters but one of them is &self or &mut self the lifetime of self is assigned to all output lifetime parameters

// Static Lifetime - lasts for the entire duration of the execution of a program.
// Can be coerced to a shorter lifetime if needed.
// String literals (&str) are by implicitly convertered to &'static str because they are hardcode into binary of the program's entire lifetime execution.

// static and const are different as in static will always have the same location till program completes the execution.
// const can be inlined and can change the memory location.

/* Make it work */
// use std::fmt::Debug;

// fn print_it<T: Debug + 'static>( input: T) {
//     println!( "'static value passed in is: {:?}", input );
// }

// fn print_it1( input: impl Debug + 'static ) { // Same as the print_it()
//     println!( "'static value passed in is: {:?}", input );
// }

// fn print_it2<T: Debug + 'static>( input: &T) {
//     println!( "'static value passed in is: {:?}", input );
// }

// fn main() {
//     // i is owned and contains no references, thus it's 'static:
//     let i: i32 = 5;
//     print_it(i);

//     // oops, &i only has the lifetime defined by the scope of
//     // main(), so it's not 'static:
//     print_it(i);

//     print_it1(i);

//     // but this one WORKS !
//     print_it2(&i);
// }


// Closures - Anonymous functions that are able to capture the values from the scope in which they are defined in.
// can be defined inline (for example as a function parameter)
// They don't require type annotations.
// can take ownership of a value by using `move` keyword
// Both the input type and the return type of closures can be inferred by the compiler.

// example

// fn main() {
//     let x = 2;
//     let y = |val| val + x;
//     assert_eq!(y(3), 5);
// }

// example 2
// fn main() {
//     fn closure1(i: i32) -> i32 { i+1 }

//     let closure_annotated = |i: i32| -> i32 { i + 1};
//     let closure_inferred = |i| i+1;

//     let i = 1;
//     // Call the function and closures.
//     println!("function: {}", closure1(i));
//     println!("closure_annotated: {}", closure_annotated(i));
//     println!("closure_inferred: {}", closure_inferred(i));

// }


// Capturing - Closures can capture variables by borrowing or moving.
// it can be done by using &T(Reference), &mut T(mutable reference) or by T(value).

/* Make it work with least amount of changes*/
// fn main() {
//     let color = String::from("green");

//     let print = || println!("`color`: {}", color); // Closures by default take immutable reference to the variables used inside them.

//     print();
//     print();

//     // `color` can be borrowed immutably again, because the closure only holds
//     // an immutable reference to `color`. 
//     let _reborrow = &color;

//     println!("{}",color);
// }

/* Make it work 
- Dont use `_reborrow` and `_count_reborrowed`
- Dont modify `assert_eq`
*/
// fn main() {
//     let mut count = 0;

//     let mut inc = move || { // This move will get the copy of count not reference. Count is still accessible after this point.
//         count += 1;
//         println!("`count`: {}", count);
//     };

//     inc();

//     // In rust, we can only have one mutable references or any number of immutable reference - but not both at the same time.
//     let _reborrow = &count; 

//     inc();

//     // The closure no longer needs to borrow `&mut count`. Therefore, it is
//     // possible to reborrow without an error
//     let _count_reborrowed = &mut count; 

//     inc();

//     inc();

//     assert_eq!(count, 0);
// }

/* Make it work in two ways, none of them is to remove `take(movable)` away from the code
*/
// fn main() {
//     let movable = Box::new(3);

//     let consume = move || {
//         println!("`movable`: {:?}", movable); // Only needs a immutable reference to println!.
//         take(&movable);
//     };

//     consume();
//     consume();
// }

// fn take<T>(_v: &T) {} // Will take ownership of the data if argument is expecting `T`, so we have to make it as a reference(&)

// NO issues for the below code.
// fn main() {
//     let movable = Box::new(3);

//     let consume = move || {
//         println!("`movable`: {:?}", movable);
//     };

//     consume();
//     consume();
// }


// fn main() {
//     let example_closure = |x| x;

//     let s = example_closure(String::from("hello"));

//     /* Make it work, only change the following line */
//     let n = example_closure(5.to_string()); // passing 5 will cause an issue because, calling the closure before would make the compiler infer that the expected type is String.
// }

// Fn, FnMut, FnOnce - When taking closure as an input parameter, the closure's complete type must be annotated.
// Fn - The closure uses the captured value by reference.
// FnMut - the closure uses the captured value by mutable reference.
// FnOnce - the closure uses the captured value by value, by taking ownership.


/* Make it work by changing the trait bound, in two ways*/
// fn fn_once<F>(func: F)
// where
//     // F: FnOnce(usize) -> bool, // will take ownership of vec, cannot use it twice as showcased.
//     F: Fn(usize) -> bool,
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// }

// fn main() {
//     let x: Vec<i32> = vec![1, 2, 3];
//     fn_once(|z|{z == x.len()})
// }
 
// Example 2
// fn main() {
//     let mut s: String = String::new();
//     let update_string = |str| s.push_str(str);
//     exec(update_string);
//     println!("{:?}",s);
// }

// /* Fill in the blank */
// fn exec<'a, F: FnMut(&'a str)>(mut f: F)  { // since we are mutating the string literal in the closure, it's good to take the least restrictive approach.
//     f("hello")
// }

/* Fill in the blank */

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
// fn apply<F>(f: F) where
//     // The closure takes no input and returns nothing.
//     F: FnOnce() {

//     f();
// }

// A function which takes a closure and returns an `i32`.
// fn apply_to_3<F>(f: F) -> i32 where
//     // The closure takes an `i32` and returns an `i32`.
//     F: Fn(i32) -> i32 {

//     f(3)
// }

// fn main() {
//     use std::mem;

//     let greeting: &str = "hello";
//     // A non-copy type.
//     // `to_owned` creates owned data from borrowed one
//     let mut farewell: String = "goodbye".to_owned(); // to_owned() will convert the string literal(&str) to String

//     // Capture 2 variables: `greeting` by reference and
//     // `farewell` by value.
//     let diary = || {
//         // `greeting` is by reference: requires `Fn`.
//         println!("I said {}.", greeting);

//         // Mutation forces `farewell` to be captured by
//         // mutable reference. Now requires `FnMut`.
//         farewell.push_str("!!!");
//         println!("Then I screamed {}.", farewell);
//         println!("Now I can sleep. zzzzz");

//         // Manually calling drop forces `farewell` to
//         // be captured by value. Now requires `FnOnce`.
//         mem::drop(farewell);
//     };

//     // Call the function which applies the closure.
//     apply(diary);

//     // `double` satisfies `apply_to_3`'s trait bound
//     let double = |x| 2 * x;

//     println!("3 doubled: {}", apply_to_3(double));
// }


/* Fill in the blank */
// fn main() {
//     let mut s: String = String::new();

//     let update_string = |str| -> String {s.push_str(str); s }; // This one implements FnOnce, because we are returning a capture value and it should be the owner to return it. 

//     exec(update_string);
// }

// fn exec<'a, F: FnMut(&'a str) -> String>(mut f: F) {
//     f("hello");
// }


/* Implement `call_me` to make it work */
// fn call_me<F>(f: F) -> () where F: Fn() -> () {
// fn call_me<F: Fn()>(f: F) { // Both the ways works as shown
//     f();
// }

// fn function() {
//     println!("I'm a function!");
// }

// fn main() {
//     let closure = || println!("I'm a closure!");

//     call_me(closure);
//     call_me(function);
// }

/* Fill in the blank using two approaches,
 and fix the error */
// fn create_fn() -> impl Fn(i32) -> i32 { // Static dispatch methods.
//     let num: i32 = 5;

//     // How does the following closure capture the environment variable `num`
//     // &T, &mut T, T ?
//     // move is required because we are returning a closure which holds reference to num 
//     // variable which will be out of scope once the compiler exits the create_fn().

//     move |x| x + num 
// }

// fn create_fn2() -> Box<dyn Fn(i32) -> i32> { // Static dispatch methods.
//     let num: i32 = 5;

//     // How does the following closure capture the environment variable `num`
//     // &T, &mut T, T ?
//     // move is required because we are returning a closure which holds reference to num 
//     // variable which will be out of scope once the compiler exits the create_fn().

//     Box::new(move |x| x + num)
// }


// fn main() {
//     let fn_plain = create_fn2(); 
//     let result = fn_plain(1);

//     println!("{}", result);
// }

// Iterator - allows to perform a task on a sequence of items in turn 
// All iterator implements the trait Iterator which has the function next() which is called automatically when traversing over the data.
// Some consume iterator while some produce iterator.

// Example
// fn main() {
//     let v: Vec<i32> = vec![1, 2, 3];
//     for x in v {
//         println!("{}",x)
//     }
// }

// fn main() {
//     let v: Vec<i32> = vec![1, 2, 3];
//     for x in v.into_iter() {
//         println!("{}",x)
//     }
// }

// fn main() {
//     let arr = [0; 10];

//     for v in arr {
//         println!("{}", v);
//     }

//     for v in arr.into_iter() {
//         println!("{}", v);
//     }

//     for v in 0..arr.len() {
//         println!("{}", arr[v]);
//     }
// }

/* Fill the blanks and fix the errors.
Using two ways if possible */
// fn main() {
//     let mut v1 = vec![1, 2].into_iter(); // A collection to a iterator, the result variable should always be mutable.
//     assert_eq!(v1.next(), Some(1));
//     assert_eq!(v1.next(), Some(2));
//     assert_eq!(v1.next(), None);
// }

// into_iter, iter and iter_mut - all of them can convert a collection into iterator, but in different ways

// into_iter consumes the collection, once the collection has been consumed, 
// it is no longer available for reuse, because its ownership has been moved within the loop.

// iter, this borrows each element of the collection through each iteration, thus leaving the collection untouched and available for reuse after the loop

// iter_mut, this mutably borrows each element of the collection, allowing for the collection to be modified in place.

 /* Fill in the blank */
// fn main() {
//     let mut names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];

//     for name in names.iter_mut() {
//         *name = match name {
//             &mut "Ferris" => "There is a rustacean among us!",
//             _ => "Hello",
//         }
//     }

//     println!("names: {:?}", names);
// }

/* Fill in the blank */
// fn main() {
//     let mut values = vec![1, 2, 3];
//     let mut values_iter = values.iter_mut();

//     if let Some(v) = values_iter.next(){
//         *v = 0;
//     }

//     assert_eq!(values, vec![0, 2, 3]);
// }

// Fibonacci example
// struct Fibonacci {
//     curr: u32,
//     next: u32,
// }

// // Implement `Iterator` for `Fibonacci`.
// // The `Iterator` trait only requires a method to be defined for the `next` element.
// impl Iterator for Fibonacci {
//     // We can refer to this type using Self::Item
//     type Item = u32;
    
//     /* Implement next method */
//     fn next(&mut self) -> Option<Self::Item> {
//         let forward = self.curr + self.next;
//         self.curr = self.next;
//         self.next = forward;
//         Some(forward)
//     }
// }

// // Returns a Fibonacci sequence generator
// fn fibonacci() -> Fibonacci {
//     Fibonacci { curr: 0, next: 1 }
// }

// fn main() {
//     let mut fib = fibonacci();
//     assert_eq!(fib.next(), Some(1));
//     //  assert_eq!(fib.next(), Some(1));
//     assert_eq!(fib.next(), Some(2));
//     assert_eq!(fib.next(), Some(3));
//     assert_eq!(fib.next(), Some(5));
// }

// Collect - collect will consume the iterator
// Converting a collection into an iterator, we can also collect the result values into a collection

/* Make it work */
// use std::collections::HashMap;
// fn main() {
//     let names: [(&str, i32); 2] = [("sunface",18), ("sunfei",18)];
//     let folks: HashMap<&str, i32> = names.into_iter().collect(); // into_iter will take ownership of the data

//     println!("{:?}",folks);

//     let v1: Vec<i32> = vec![1, 2, 3];

//     let v2: Vec<&i32>  = v1.iter().collect(); // iter() will take only references so, it will be a collection of references to i32 values.

//     assert_eq!(v2, vec![&1, &2, &3]);
// }

/* Fill in the blanks */
// Chaining multiple iterators.
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<i32> = v1.iter().map(|x| x+1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}