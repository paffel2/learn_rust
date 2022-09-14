fn main() {
    let a = vaporcode("Lets go to the movies");
    println!( "{}", a);
}

use itertools::Itertools;
/*fn vaporcode(s: &str) -> String
    {
        let a = s.to_uppercase().replace(" ","");
        let chars = a.chars();
        let mut result = String::new();
        for c in chars
            {
                result.push(c);
                result.push(' ');
                result.push(' ');
            }
        let r = &result[0..result.len()-2];
        return r.to_string();
    } */

fn vaporcode(s: &str) -> String
    {   
        s.replace(" ", "").to_uppercase().chars().join("  ")
    }