// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_vector1() {
        let mut vec = Vec::new();
        vec.push(1);
        assert_eq!(vec[0], 1);
    }
}
