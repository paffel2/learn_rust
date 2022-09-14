fn main() {
    let r = index(&[1, 2, 3, 4], 2);
    println!("result = {}", r.unwrap());
}


fn index(nums: &[u64], n: usize) -> Option<u64> {
    if n >= nums.len()
    {
        return None;
    }
    let s = n as u32;
    let r = u64::pow(nums[n],s);
    return Some(r);

}