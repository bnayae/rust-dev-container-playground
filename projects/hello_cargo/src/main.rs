use chrono::{Timelike, Utc};
// use std::time::SystemTime;

fn main() {
    println!("Hello, cargo!");

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
