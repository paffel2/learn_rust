fn main() {
    println!("result = {:#?}", two_sum(&[1,2,3],4));
}


fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    for i in 0.. numbers.len()
        {
            for j in i+1 .. numbers.len()
            {
                if numbers[i] + numbers[j] == target
                {   
                    return (i,j);
                }
            }
            if i == numbers.len()
                {
                    break;
                }
        }
    return (0,0);
}