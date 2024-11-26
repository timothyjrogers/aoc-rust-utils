pub fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> u32 {
    return p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manhattan_distance_equal() {
        let p1: (i32, i32) = (5, -5);
        let p2: (i32, i32) = (5, -5);
        assert_eq!(manhattan_distance(p1, p2), 0);
    }

    #[test]
    fn test_manhattan_distance() {
        let p1: (i32, i32) = (0, 0);
        let p2: (i32, i32) = (10, 10);
        assert_eq!(manhattan_distance(p1, p2), 20);
    }
}