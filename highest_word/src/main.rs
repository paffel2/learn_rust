fn main() {
    println!("result = {}", high("man i need a taxi up to ubud"));
}


fn high(input: &str) -> &str {
    let mut max_word_size = 0;
    let mut max_word = "";

    let vec: Vec<&str> = input.split(' ').collect();

    if input.is_empty()
        {
            return "";
        }
    for word in vec
        {
            if count(word) > max_word_size
                {
                    max_word = word;
                    max_word_size = count(word);
                }
        }
    return max_word;
}


fn count(s:&str) -> i32 {
    let mut result = 0;
    let a_num = 'a' as i32;
    for char in s.chars()
    {
        result += char as i32 - a_num + 1;
    }
    return result;
}