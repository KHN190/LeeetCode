// https://leetcode.com/problems/add-binary/

pub fn add_binary(a: String, b: String) -> String {
    let n1 = u128::from_str_radix(&a, 2).unwrap_or(0);
    let n2 = u128::from_str_radix(&b, 2).unwrap_or(0);
    format!("{:b}", n1 + n2)
}

#[test]
fn run() {
    assert_eq!(&add_binary("01".into(), "11".into()), "100");
}
