fn is_plural (s: &String) -> bool{
    s.ends_with('s')
}

pub fn inspect (s: &String){
    if is_plural(s) {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

pub fn change (s: &mut String){
    if !is_plural(s) {
        s.push_str("s");
    }
}

pub fn eat (s: String) -> bool {
    s.starts_with('b') && s.contains("a")
}

pub fn add(a:&i32, b:&i32) -> i32 {
    *a + *b
}