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

    #[test]
    fn ok_result()  -> Result<(), String> {
        Ok(())
    }

    #[test]
    fn bad_result()  -> Result<(), String> {
        Err(String::from("something is wrong"))
    }
}
