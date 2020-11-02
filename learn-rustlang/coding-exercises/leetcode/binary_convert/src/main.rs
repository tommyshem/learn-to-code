fn main() {
    let list = vec![0, 1, 0, 1];
    print!("Binary number is {}", convert(list));
}

fn powerof(index: u32) -> u128 {
    if index == 0 {
        return 1;
    } else {
        let mut mult = 1;
        for _ in 0..index {
            mult = mult * 2;
        }
        return mult;
    }
}

fn convert(list: Vec<i32>) -> u128 {
    let mut result: u128 = 0;
    // loop through the list
    for (index, binary_value) in list.iter().rev().enumerate() {
        match binary_value {
            0 => {}
            1 => result += powerof(index as u32),
            _ => panic!("Not a binary number"),
        }
    }
    return result;
}

#[cfg(test)]
mod tests {

    #[test]
    fn testpowerof() {
        assert_eq!(super::powerof(0), 1);
        assert_eq!(super::powerof(1), 2);
        assert_eq!(super::powerof(2), 4);
        assert_eq!(super::powerof(3), 8);
        assert_eq!(super::powerof(4), 16);
        assert_eq!(super::powerof(5), 32);
        assert_eq!(super::powerof(6), 64);
        assert_eq!(super::powerof(7), 128);
        assert_eq!(super::powerof(8), 256);
        assert_eq!(super::powerof(9), 512);
        assert_eq!(super::powerof(10),1024)
    }
    #[test]
    fn test1() {
        let binary = vec![1];
        assert_eq!(super::convert(binary), 1);
    }
    #[test]
    fn test10() {
        let binary = vec![1, 0, 1, 0];
        assert_eq!(super::convert(binary), 10);
    }
    #[test]
    fn test128() {
        let binary = vec![1, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(super::convert(binary), 128);
    }

    #[test]
    fn test8() {
        let binary = vec![
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0,
        ];
        let result = super::convert(binary);
        println!("result = {}", result);
        assert_eq!(result, 1073741824);
    }
}
