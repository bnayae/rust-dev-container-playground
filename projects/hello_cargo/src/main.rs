fn main() {
    println!("Hello, cargo!");

    let vector = vec!["a", "b", "c"];
    println!("For loop");
    for x in vector {
        // consume vector, owned x
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
        // borrows vector, & to x
        println!("\tItem = {x}");
    }

    // todo: fix ^^^^ value borrowed here after move
    // println!("For &loop");
    // for x in &vector {
    //     // equivalent to vector.into_iter()
    //     println!("\tItem = {x}");
    // }
}
