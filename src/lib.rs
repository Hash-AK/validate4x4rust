pub fn split(input:String) -> Vec<i32> {
   return input.split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
}
pub fn addline(line_to_add:String, v:&mut Vec<Vec<i32>>) {
    let numbers: Vec<i32> = split(line_to_add.to_string());
    v.push(numbers);
}

