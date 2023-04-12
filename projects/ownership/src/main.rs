use std::cmp::max;

fn main() {

    let r = first_word("simply dummy text of the printing");
    println!("first ok: '{r}'");
    // let r = first_word_with_bug(" simply dummy text of the printing");
    let r = first_word(" simply dummy text of the printing");
    println!("first ok: '{r}'");

    let r = take_after("simply dummy text of the printing", "dummy");
    println!("middle: '{r}'");
    let r = take_after("dummy", "simply dummy text of the printing");
    println!("overflow: '{r}'");
    let r = take_after("simply dummy text of the printing", "printing");
    println!("last: '{r}'");
// let s = String::from("hello world");
// let hello = &s[0..5];
// let hello = &s[..5];
// let world = &s[6..11];
// let world = &s[6..];
// let hello_world = &s[..];
// println!("{hello} {world}: {hello_world}");


    let mut hi = String::from("hi");
    let len = get_length(&hi);
    // hi is still in memory
    println!("'{hi}' length is {len}");
    change(&mut hi);
    println!("'{hi}' has changed");

    take_ownership(hi);
    // println!("Illigal use of '{hi}'");

    let mut s = String::from("hello");
    {
        let r1 = &s; // OK
        println!("r1: {r1}");
    }
    s.push_str(", world"); // OK
    let r2 = &s; // OK
    println!("r2: {r2}");

    println!("End!");
}

fn take_ownership(s: String) {
    println!("Take ownership of '{s}'");
}

fn get_length(s: &String) -> usize {
    // borrow ownership
    s.len()
}

fn change(v: &mut String) {
    v.push_str(" there");
}


fn first_word (source: &str) -> &str {
    let enumerator = source.chars().into_iter().enumerate();
    for (i, c) in enumerator {
        if c == ' ' {
            return &source[..i];
        }
    }
    &source[..]
}

fn take_after<'a> (source: &'a str, criteria: &str) -> &'a str {
    let criteria_len = criteria.len();
    let source_len = source.len();
    let limit = max(source_len.saturating_sub(criteria_len), 0);
    // println!("limit {limit}");
    for i in 0..limit {
        let slice = &source[i..i + criteria_len];
        if slice == criteria {
            return &source[i + criteria_len..];
        }
    }
    ""
}
