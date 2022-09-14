fn main() {
    let result = persistence(999);
    println!("result = {}", result);
}


fn mult_digits (num: u64) -> u64
    {   
        loop {
            if num / 10 == 0
            {
                break num;
            }
            return (num % 10) * mult_digits(num / 10);
        }         
           
    }

fn persistence (num: u64) -> u64 {
    if num < 10
    {
        0;
    }
    return helper(0, num);
}

fn helper (acc: u64, num:u64) -> u64
    {
        loop {
            if num < 10
            {
                break acc;
            }
            return helper(acc + 1, mult_digits(num));
        }
    }