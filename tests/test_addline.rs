// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html

//fn split(input:String) -> Vec<i32> {
//   return input.split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
//}

use validate4x4rust::split;
use validate4x4rust::addline;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_vector1() {
        let input = "10 20 30 40 50";
    
        // Parse into a Vector of integers
        let numbers: Vec<i32> = split(input.to_string());
        assert_eq!(numbers[0], 10);
    }
    
    #[test]
    fn test_vec2() {
        let mut v = Vec::new();
    let input = "10 20 30 40 50";
        // Parse into a Vector of integers
        let numbers: Vec<i32> = split(input.to_string());
        assert_eq!(numbers[1],20);
    let input2 = "10 20 32 40 50";
        let numbers2: Vec<i32> = split(input2.to_string());
        assert_eq!(numbers2[2],32);
        v.push(numbers);
        v.push(numbers2);
        assert_eq!(v[1][2],32);
    }
    
    #[test]
    fn test_addline() {
        let mut v = Vec::new();
        let line1 = "1 2 3 4";
        addline(line1.to_string(), &mut v);
        assert_eq!(v[0][2],3);
        let line2 = "3 4 1 2";
        addline(line2.to_string(), &mut v);
        let line3 = "2 1 4 3";
        addline(line3.to_string(), &mut v);
        let line4 = "4 3 2 1";
        addline(line4.to_string(), &mut v);
        assert_eq!(v[3][3],1);
    }
}
