pub fn calculate(s: String) -> i32 {
    enum Operator {
        Add,
        Sub,
        Mul,
        Div,
        Ext,
    }

    let mut stack: Vec<i32> = Vec::new();
    let mut n = 0;
    let mut op = Operator::Add;
    for c in (s + ".").chars() {
        if c == ' ' {
            continue;
        }
        if ('0'..='9').contains(&c) {
            n = n * 10 + (c as u8 - b'0') as i32;
        } else {
            match op {
                Operator::Add => stack.push(n),
                Operator::Sub => stack.push(-n),
                Operator::Mul => *stack.last_mut().unwrap() *= n,
                Operator::Div => *stack.last_mut().unwrap() /= n,
                Operator::Ext => {}
            };
            op = match c {
                '+' => Operator::Add,
                '-' => Operator::Sub,
                '*' => Operator::Mul,
                '/' => Operator::Div,
                _ => Operator::Ext,
            };
            n = 0;
        }
    }
    stack.iter().sum()
}

#[test]
fn run() {
    let n = calculate("3 /1+ 2*2".into());
    assert_eq!(n, 7);
}
