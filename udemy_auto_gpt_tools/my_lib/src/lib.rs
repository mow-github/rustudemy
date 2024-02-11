/*
    Note:
        Create lib:     `cargo new --lib my_lib`
        DocBlocks:      cargo doc && cargo doc --open
*/


/// Function: add
/// 
/// #Example
/// ```
/// let a: usize = 5;
/// let b: usize = 6;
/// let y = add(a, b); 
/// ``` 
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
