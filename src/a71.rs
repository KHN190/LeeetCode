// https://leetcode.com/problems/simplify-path/

pub fn simplify_path(path: String) -> String {
    let mut stack = vec![];
    for s in path.split("/") {
        match s {
            ".." => {
                stack.pop();
            }
            "." => {
                continue;
            }
            _ => {
                if !s.is_empty() {
                    stack.push(s);
                }
            }
        };
    }
    format!("/{}", stack.join("/"))
}

#[test]
fn run() {
    let res = simplify_path("/home/".into());
    assert_eq!(&res, "/home");
}
