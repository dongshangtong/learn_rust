fn sum_u32_array(numbers: &[u32]) -> Option<u32> {
    let mut result:u32 = 0;
    for &number in numbers {
        result = match result.checked_add(number) {
            Some(res) => res,
            None => return None,
        };
    }
    Some(result)
}


fn main() {
   let numbers = [1, 2, 3, 4, 5];
println!("Sum of numbers: {:?}", sum_u32_array(&numbers));
}
