use std::cell::RefCell;
use std::rc::Rc;

use crate::types::TreeNode;

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n == 0 {
        return vec![];
    }
    gen_tree(1, n as usize)
}

fn gen_tree(start: usize, end: usize) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut trees = vec![];
    if start > end {
        trees.push(None);
        return trees;
    }

    for i in start..=end {
        let left = gen_tree(start, i - 1);
        let right = gen_tree(i + 1, end);

        for l in left.iter() {
            for r in right.iter() {
                let mut node = TreeNode::new(i as i32);
                node.left = l.clone();
                node.right = r.clone();

                trees.push(Some(Rc::new(RefCell::new(node))));
            }
        }
    }
    trees
}

#[test]
fn run() {
    let trees = generate_trees(3);
    assert_eq!(trees.len(), 5);
}
