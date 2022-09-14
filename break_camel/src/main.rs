fn main() {
    let result = solution("");
    println!("result = {}", result);

}


fn solution(s: &str) -> String {
    let chars = s.chars();
    let mut result = String::new();
    for c in chars
    {
        if c.is_ascii_uppercase()
        {
            result.push(' ');
        }
        result.push(c);
        
    }
    return result;

}