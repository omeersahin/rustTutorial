use std::io;


fn main(){


    
    

    //


    // Scalar Other Data Types

    let data  : i8   = -1;
    let data  : i16  = -2;
    let data  : i32  = -3;
    let data  : i64  = -4;    
    let data  : i128 = -5;
    let data  : u8   = 6;
    let data  : u16  = 8;
    let data  : u32  = 9;
    let data  : u64  = 10;
    let data  : u128 = 11;
    let data  : f32  = 12.0;
    let data  : f64  = 13.0;

    // Data Types Limits

    println!("The range of {} is: {} - {}", "i8"  , i8::MIN    , i8::MAX   );
    println!("The range of {} is: {} - {}", "i16" , i16::MIN   , i16::MAX  );
    println!("The range of {} is: {} - {}", "i32" , i32::MIN   , i32::MAX  );
    println!("The range of {} is: {} - {}", "i64" , i64::MIN   , i64::MAX  );
    println!("The range of {} is: {} - {}", "i128", i128::MIN  , i128::MAX );
    // Architecture Size x86 -> i32 , x64 -> i64
    println!("The range of {} is: {} - {}", "isize", isize::MIN , isize::MAX); 

    println!("The range of {} is: {} - {}", "u8"  , u8::MIN    , u8::MAX   );
    println!("The range of {} is: {} - {}", "u16" , u16::MIN   , u16::MAX  );
    println!("The range of {} is: {} - {}", "u32" , u32::MIN   , u32::MAX  );
    println!("The range of {} is: {} - {}", "u64" , u64::MIN   , u64::MAX  );
    println!("The range of {} is: {} - {}", "u128", u128::MIN  , u128::MAX );
    // Architecture Size x86 -> u32 , x64 -> u64
    println!("The range of {} is: {} - {}", "usize", usize::MIN , usize::MAX);

    println!("The range of {} is: {} - {}", "f32"  , f32::MIN  , f32::MAX  );
    println!("The range of {} is: {} - {}", "f64"  , f64::MIN  , f64::MAX  );

    //let x = i32::MAX + 1;

    // integer default i32
    // float   default f64


    //Boolean
    let t = true;
    let f: bool = false;
    println!("t is: {}", t);
    println!("f is: {}", f);

    // Char
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("c is: {}", c);
    println!("z is: {}", z);
    println!("heart_eyed_cat is: {}", heart_eyed_cat);

    // Compound Data Types


    // Tuple

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("five_hundred is: {}", five_hundred);
    println!("six_point_four is: {}", six_point_four);
    println!("one is: {}", one);


    // Array

    let array1 = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];


    let array2: [i32; 5] = [1, 2, 3, 4, 5];

    let array3 = [3; 5];

    let first = array1[0];
    println!("first is: {}", first);
    let second = array1[1];
    println!("second is: {}", second);
    
    // Accessing array element with input

    println!("Please enter an array index : ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = array1[index];

    println!("The value of the element at index {index} is: {element}");


    // Number Seperator

    let x = 1000000;
    let y: i32 = 1_000_000;



    // constants

    const HOURS_IN_SECONDS: u32 = 60 * 60;

    // const HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // Mutable

    // 
    let x = 5;
    println!("The value of x is: {x}");

    /*
    x = 6;
    println!("The value of x is: {x}");
    */
    

    let x = 7;
    println!("The value of x is: {x}");

    let x = x + 3;
    println!("The value of x is: {x}");   

    let x = "xstring";
    println!("The value of x is: {x}");

    let mut y = 10;
    println!("The value of y is: {x}");

    y = 15;
    println!("The value of y is: {x}");

    /*
    y = "ystring";
    println!("The value of y is: {y}");
    */

}