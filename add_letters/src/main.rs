fn main() {
    let a = add_letters(vec!['z']);
    println!("result = {}", a);
}


fn add_letters(letters: Vec<char>) -> char {
    if letters.is_empty() {
        return 'z';
    }
    let a_num = 'a' as u32;
    let r = letters.iter().map(|&x| x as u32 - a_num + 1);
    let s: u32 = r.sum();
    if s % 26 == 0
        {
            return 'z';
        }
    return std::char::from_u32((s % 26) + a_num - 1).unwrap();
} 