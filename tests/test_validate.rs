#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use validate4x4rust::split;
    fn addline(line_to_add:String, v:&mut Vec<Vec<i32>>) {
        let numbers: Vec<i32> = split(line_to_add.to_string());
        v.push(numbers);
    }
    fn validate4x4(vector:Vec<Vec<i32>>) -> bool {
        return false;
    }

    #[test]
    fn test_vector1() {
        let input = "10 20 30 40 50";
    
        // Parse into a Vector of integers
        let numbers: Vec<i32> = split(input.to_string());
        assert_eq!(numbers[0], 10);
    }
    
    #[test]
    fn test_vec3() {
        let mut v = Vec::new();
        let line1 = "1 2 3 4";
        addline(line1.to_string(), &mut v);
        let line2 = "3 4 1 2";
        addline(line2.to_string(), &mut v);
        let line3 = "2 1 4 3";
        addline(line3.to_string(), &mut v);
        let line4 = "4 3 2 1";
        addline(line4.to_string(), &mut v);
        assert!(validate4x4(v));
    }
}
