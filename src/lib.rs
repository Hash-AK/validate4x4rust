pub fn split(input:String) -> Vec<i32> {
   return input.split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
}

