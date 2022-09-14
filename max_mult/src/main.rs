fn main() {
    let r = max_multiple(2, 7);
    println!("result = {}", r);
}


/*fn max_multiple(divisor: u32, bound: u32) -> u32 {
    let mut ans: u32 = 0;
    for i in divisor .. bound + 1
        {
            if i % divisor == 0 && i <= bound
            {
                ans = i;
            }
        }
    return ans;
}*/

fn max_multiple(divisor: u32, bound: u32) -> u32 {
    bound - (bound % divisor)
}