// code wars - count of position sum of negatives

fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return Vec::new();
    }
    let mut count = vec![0, 0];
    for x in input.iter() {
        if x > &0 {
            count[0] = count[0] + 1;
        } else {
            count[1] = x + count[1];
        }
    }
    count
}
// best solution - for me I found on the site
fn count_positives_sum_negatives1(input: Vec<i32>) -> Vec<i32> {
    if !input.is_empty() {
        let mut pc = 0;
        let mut ns = 0;
        for x in input {
            if x > 0 {
                pc += 1
            } else {
                ns += x
            }
        }
        vec![pc, ns]
    } else {
        vec![]
    }
}

#[test]
fn returns_expected() {
    let test_data1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15];
    let expected1 = vec![10, -65];
    assert_eq!(count_positives_sum_negatives(test_data1), expected1);
}

#[test]
fn returns_expected1() {
    let test_data1 = vec![];
    let expected1 = vec![];
    assert_eq!(count_positives_sum_negatives(test_data1), expected1);
}
