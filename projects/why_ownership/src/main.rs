fn main() {
    let mut s = String::from("hello world ");
    let word = first_word(&s);
    // let word = first_word_slice(&s);
    s.clear();
    println!("'{s}' starts with '{word}'");
    let word = rest_slice("hello world");
    println!("'hello world' ends with '{word}'");
    let word = rest_slice("hello ");
    println!("'hello ' ends with '{word}'");
    let word = rest_slice("hello-world ");
    println!("'hello-world' ends with '{word}'");
}

fn first_word(source: &str) -> String {
    let mut result = String::new();
    for c in source.chars() {
        if c == ' ' {
            break;
        }
        result.push(c)
    }
    result
}

fn first_word_slice(source: &str) -> &str {
    for (i, c) in source.chars().into_iter().enumerate() {
        if c == ' ' {
            return &source[..i];
        }
    }
    &source[..]
}

fn rest_slice(source: &str) -> &str {
    for (i, c) in source.chars().into_iter().enumerate() {
        if c == ' ' {
            return &source[i + 1..];
        }
    }
    ""
}
