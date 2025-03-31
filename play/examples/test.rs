pub fn percentage_letter(s: String, letter: char) -> i32 {
    s.chars().into_iter().filter(|c| *c == letter).count() as i32 * 100 / s.len() as i32
}

fn main(){
    println!("{}", percentage_letter("abbca".to_string(), 'a'));
}