use chrono::{Timelike, Utc};
// use std::time::SystemTime;

fn another_function(x: i32) -> i32 {
    if x % 2 == 0 {
        return x * 3;
    } else if x % 3 == 0 {
        return x * 4;
    } else {
        return x * 5;
    }
}

fn main() {
    let x = ['H', 'e', 'l', 'l', 'o'];
    let y = x;
    for c in x {
        print!("{c}");
    }
    println!();
    for c in y {
        print!("{c}");
    }
    println!();

    let x = String::from("Hello");
    let mut y = x.clone();
    y.push_str(", World");

    println!("x = {x}, y = {y}");
    println!("x = {x}, y = {y}");
    println!("Hello, world!");
    let v = another_function(3);
    println!("Return = {v}");

    let number = if v % 2 == 0 { 5 } else { 6 };

    println!("#{number}");

    println!("Hello, cargo!");

    // // statement
    // let val = 1;
    // // let val = let sub = 1; // NOT VALID
    // // value of the inner expression get bound to the outer statement
    // let y = {
    //     let x = 3;
    //     x + 1 // Expressions do not include ending semicolons
    // };

    // println!(val);

    let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("first = {first}, second = {second}");

    // // initial array with same repeatative value
    // let b = [3; 5];
    // // let b = [3, 3, 3, 3, 3];

    // Because the enumerate method returns a tuple
    for (index, &_) in [0; 10].iter().enumerate() {
        println!("index! = {index}");
    }

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // deconstruct
    let (fst, snd, trd) = tup;
    println!("{fst}, {snd}, {trd}");
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    let f = -2.1;
    println!("floating {f}");

    let a: u8 = u8::MAX; // 255 max value
                         /****************** START: overflow ******************/
    // let b:u8 = a + 1; // panic
    // allow wrapping intentionally
    let b: u8 = a.wrapping_add(1); // a + 1;
    println!("b = {b}");
    // try add (into tuple)
    let (b1, b2): (u8, bool) = a.overflowing_add(1); // a + 1;
    println!("overflowed = {b2}, {b1}");
    // check into an option
    let b: Option<u8> = (a - 1).checked_add(1); // a - 1 + 1;
    let r = match b {
        Some(s) => s.to_string(),
        None => "NONE".to_string(),
    };
    println!("r = {r}");
    // check into an option
    let b: Option<u8> = a.checked_add(1); // a + 1;
    let r = match b {
        Some(s) => s.to_string(),
        None => "NONE".to_string(),
    };
    println!("r = {r}");
    /****************** END: overflow ******************/

    /***************** START: shadow sample *******************/
    let now = Utc::now();
    let shadow: u32 = 10 * 10;
    println!("shadow = {shadow}");
    let shadow = shadow * now.second();
    // let shadow = shadow * SystemTime::now().;
    println!("shadow = {shadow}");
    {
        let shadow = now.minute();
        // let shadow = OffsetDateTime::now_utc().month();
        println!("shadow = {shadow}");
    }
    println!("shadow = {shadow}");
    /***************** END: shadow sample *******************/

    let vector = vec!["a", "b", "c"];
    println!("For loop");
    for x in &vector {
        println!("\tItem = {x}");
    }

    println!("is actually: iter + while");
    // todo: let mut iter = vector.into_iter();
    let mut iter = vec!["a", "b", "c"].into_iter();
    while let Some(e) = iter.next() {
        println!("\tItem = {e}");
    }

    // https://www.youtube.com/watch?v=yozQ9C69pNs&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=3
    println!("For loop + .iter");
    for x in vector.into_iter() {
        println!("\tItem = {x}");
    }

    // todo: fix ^^^^ value borrowed here after move
    // println!("For &loop");
    // for x in &vector {
    //     // equivalent to vector.into_iter()
    //     println!("\tItem = {x}");
    // }
}
