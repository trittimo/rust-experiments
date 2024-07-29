pub mod hello_world_001;

pub fn run_all_experiments() {
    hello_world_001::run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let result = hello_world_001::simple_add(3, 4);
        assert_eq!(7, result);
        assert_ne!(-5, result);
    }
}