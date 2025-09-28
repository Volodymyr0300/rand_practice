fn main() {
    fn square_sum(vec: Vec<i32>) -> i32 {
        let mut sum = 0;
        for i in vec {
            sum += i * i;
        }
        sum
    }
    let result = square_sum(vec![1, 2, 2]);
    println!("Square sum: {}", result);

    fn create_phone_number(numbers: &[u8]) -> String {
        format!(
            "({}{}{}) {}{}{}-{}{}{}{}",
            numbers[0], numbers[1], numbers[2],
            numbers[3], numbers[4], numbers[5],
            numbers[6], numbers[7], numbers[8], numbers[9]
        )
    }
    let result = create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    println!("Phone number: {}", result);

    fn maskify(cc: &str) -> String {
        
        if cc.len() <= 4 {
            cc.to_string()
        } else {
        format!("{}{}", "#".repeat(cc.len()), &cc[cc.len()-4..])
        

        
        }
    }
    let result = maskify("4556364607935616");
    println!("Masked: {}", result);
    
}




