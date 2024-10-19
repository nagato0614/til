fn takes_ownership(s: &String) {
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn first_word(s: &String) -> String
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return s[..i].to_string();
        }
    }
    
    s[..].to_string()
}
fn main() {
    let s = String::from("hello");
    let mut t = s.clone();

    t.push_str(", world");

    takes_ownership(&s);
    println!("{}", s);
    println!("{}", t);

    let u = &mut t;

    println!("{}", u);

    let len = first_word(&t);
    println!("{}", len);

}
