pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn wrong_add(x: i32, y: i32) -> i32 {
    x - y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 3), 4);
    }

    #[test]
    fn test_wrong_add() {
        assert_eq!(wrong_add(1, 3), 4);
    }
}

fn main() {}
