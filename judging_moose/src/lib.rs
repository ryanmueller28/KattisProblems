pub fn judging_moose(points: &[u32]) -> String {
    if points.iter().sum::<u32>() == 0 {
        return String::from("Not a moose");
    }

    if points[0] == points[1] {
        return format!("Even {}", points.iter().sum::<u32>());
    }

    format!("Odd {}", points[1] * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let arr: [u32; 2] = [2, 3];
        assert_eq!(String::from("Odd 6"), judging_moose(&arr));
    }

    #[test]
    fn test_case_2() {
        let arr: [u32; 2] = [3, 3];
        assert_eq!(String::from("Even 6"), judging_moose(&arr));
    }

    #[test]
    fn test_case_3() {
        let arr: [u32; 2] = [0, 0];
        assert_eq!(String::from("Not a moose"), judging_moose(&arr));
    }
}
