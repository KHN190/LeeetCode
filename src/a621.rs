// https://leetcode.com/problems/task-scheduler/

// Each task can be done with 1 cpu cycle.
// Each alphabet is a different task.
// Each task has cooldown (n >= 0) for same type.
// Find min time to finish a list of tasks.

// Thoughts:
//  sort by num of the task
//  A: n1
//  B: n2,
//  C: n3,
//  ..
//  pick the top to execute.
//  after that, decrease it by 1
//  and move it down by n.

use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
struct Task {
    pub count: i32,
    pub priority: i32, // count + cooldown
}

impl Task {
    pub fn new(count: i32) -> Self {
        Self {
            count,
            priority: count,
        }
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.priority.cmp(&self.priority))
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
    use std::collections::HashMap;
    // count tasks
    let mut tasks_count = HashMap::<char, i32>::new();
    for t in &tasks {
        *tasks_count.entry(*t).or_insert(0) += 1;
    }
    // tasks priority
    let mut counts: Vec<Task> = vec![];
    for v in tasks_count.values() {
        counts.push(Task::new(*v));
    }
    counts.sort();
    // take the top
    let mut cycles = 0;
    while counts[0].count > 0 {
        execute(&mut counts, n);
        // decrease cooldown for all
        for i in 0..n as usize {
            if i >= counts.len() {
                break;
            }
            let t = &mut counts[i];
            if t.priority != t.count {
                t.priority += 1;
            }
        }
        // println!("{:?}", counts);
        cycles += 1;
    }
    cycles
}

fn execute(tasks: &mut Vec<Task>, n: i32) {
    let mut t = &mut tasks[0];
    if t.priority != t.count {
        println!("  wait {:?}", t);
        return;
    }
    println!("  exec {:?}.", t);
    t.count -= 1;
    t.priority -= 1;
    t.priority -= n;
    tasks.sort();
}

#[test]
#[ignore]
fn run() {
    // @todo
    let tasks = vec!['A', 'A', 'A', 'B', 'B', 'B'];
    assert_eq!(least_interval(tasks, 2), 8);
}

#[test]
fn order() {
    let mut tasks = vec![Task::new(1), Task::new(10)];
    tasks.sort();
    assert!(tasks[0] == Task::new(10));
}
