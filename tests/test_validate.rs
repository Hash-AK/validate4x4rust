#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use validate4x4rust::split;
    use validate4x4rust::addline;

    fn validate_size(vector:Vec<Vec<i32>>) -> bool {
        if vector.len() != 4{return false;}
        for i in 0..3 {
            if vector[i].len() != 4 {return false;}
        }
        return true;
    }
    fn validate_line(line:Vec<i32>) -> bool {
        let mut checkv = vec![false,false,false,false];
        for i in 0..3 {
            if checkv[line[i]-1] == true { return false }
            checkv[line[i]-1] = true
        }
        return true
    }

    fn validate4x4(vector:Vec<Vec<i32>>) -> bool {
        return false;
    }
    
    fn test_setup() -> Vec<Vec<i32>> {
        let mut v = Vec::new();
        let line1 = "1 2 3 4";
        addline(line1.to_string(), &mut v);
        let line2 = "3 4 1 2";
        addline(line2.to_string(), &mut v);
        let line3 = "2 1 4 3";
        addline(line3.to_string(), &mut v);
        let line4 = "4 3 2 1";
        addline(line4.to_string(), &mut v);
        return v;
    }
    
    #[test]
    fn test_vector1() {
        let input = "10 20 30 40 50";
    
        // Parse into a Vector of integers
        let numbers: Vec<i32> = split(input.to_string());
        assert_eq!(numbers[0], 10);
    }

    #[test]
    fn test_validate_line_fail(){
        let mut v = vec![1,2,3,3];
        assert!(!validate_line(v));
    }
    #[test]
    fn test_validate_line() {
        let mut v = test_setup();
        assert!(validate_line(<Vec<i32> as Clone>::clone(&v[0])));
    }
    #[test]
    fn test_validate_size() {
        let mut v = test_setup();
        assert!(validate_size(v));
    }
    #[test]
    fn test_validate4x4() {
        let mut v = test_setup();
        assert!(validate4x4(v));
    }
}
