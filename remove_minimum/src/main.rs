fn main() {
    let result = remove_smallest(&[1, 2, 3, 4, 5]);
    println!("result = {:?} ", result);
}

fn remove_smallest(numbers: &[u32]) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    if numbers.is_empty()
    {
        return result;
    }
    let smallest =*(numbers.iter().min()).unwrap();
    let mut counter = 0;
    for n in numbers
        {
            if *n == smallest && counter == 0
            {
                counter = 1;
            }
            else {
                result.push(*n);
            }

        }
    return result;
    
}