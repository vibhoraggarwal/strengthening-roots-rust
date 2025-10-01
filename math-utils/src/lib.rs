pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn factorial(num: u64) -> u64 {
    if num == 0 || num == 1 {
        1
    } else {
        let mut fact = 1;
        let mut temp_num = num;
        while temp_num > 1 {
            fact = fact * temp_num;
            temp_num -= 1;
        }
        fact
    }
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
    fn factorial_three() {
        let result = factorial(3);
        assert_eq!(result, 6);
    }
}
