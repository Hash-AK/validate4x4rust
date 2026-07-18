// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_vector1() {
        let input = "10 20 30 40 50";
    
        // Parse into a Vector of integers
        let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Not a valid number"))
        .collect();
        assert_eq!(numbers[0], 10);
    }
    
    #[test]
    fn test_vec3() {
        let mut v = Vec::new();
	// comment b0
    let input = "10 20 30 40 50";
        // Parse into a Vector of integers
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
        assert_eq!(numbers[1],20);
    let input2 = "10 20 32 40 50";
        let numbers2: Vec<i32> = input2.split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
        assert_eq!(numbers2[2],32);
        v.push(numbers);
        v.push(numbers2);
        assert_eq!(v[1][2],32);
    }
}
