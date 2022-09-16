fn main()
    {   let result = get_middle("test");
        let another = get_middle_another("tes");
        println!("result = {}", result);
        println!("another = {}", another);
    }


fn get_middle(s:&str) -> String {
    if s.len() == 0
        {
            return String::new();
        }
    else if s.len() % 2 == 0
        {
            let ch1 = s.chars().nth(s.len() / 2 - 1).unwrap();
            let ch2 = s.chars().nth(s.len() / 2).unwrap();
            let mut result = String  :: new();
            result.push(ch1);
            result.push(ch2);
            return result;
        }
    else
    {
        let ch2 = s.chars().nth(s.len() / 2).unwrap();
        return ch2.into();
    }
    }

fn get_middle_another(s:&str) -> &str {
        let mid = s.len() /2;
        return &s[if s.len() % 2 == 0 {mid - 1} else {mid} .. mid + 1]
    }