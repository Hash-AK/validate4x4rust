// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html


#[cfg(test)]
mod tests {
	fn t0() -> bool {
  		return true;
	}

	fn t0() -> usize {
	 return 123;
	}

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;
    #[test]
    fn test_fn_forward() {
        let mut b:bool = t0();
	let mut u:usize = t0();

	assert!(b);
	assert_eq!(u,123);
    }

    #[test]
    fn test_vector() {
        let v = vec![1,2,3,4];

	let i:usize = 0;
	let j:i32 = 0;
 
        assert_eq!(v[0], 1);
        assert_eq!(i, 0);
        assert_eq!(v[i], 1);
        let index:usize  = j.try_into().expect("failing convert");
        assert_eq!(v[index], 1);

    let input2 = "10 20 32 40 50";
        //let numbers2: Vec<usize> = split(input2.to_string());
        let numbers2: Vec<usize> = input2.split_whitespace().map(|s| s.parse().expect("Not a valid number")).collect();
        assert_eq!(numbers2[2],32);


    }
}
