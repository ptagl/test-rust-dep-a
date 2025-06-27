#[macro_use]
extern crate log;

/// A simple function that adds two numbers together.
pub fn add(left: u64, right: u64) -> u64 {
    log::info!("Adding {} and {}", left, right);
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
