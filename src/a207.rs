// https://leetcode.com/problems/course-schedule/

// detect cycle in a graph. if there's a cycle,
// then impossible to finish all course.

const PREREQUISITE: usize = 0;
const COURSE: usize = 1;

#[derive(Copy, Clone)]
enum Status {
    NotVisited,
    Visiting,
    Visited,
}

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    let num_courses = num_courses as usize;

    // dependencies between courses
    let mut graph = vec![Vec::new(); num_courses];
    for edge in prerequisites.iter() {
        // hashtable itself is a vector of vector
        graph[edge[PREREQUISITE] as usize].push(edge[COURSE] as usize);
    }

    let mut status = vec![Status::NotVisited; num_courses];
    (0..num_courses).all(|course| !has_cycle(course, &mut status, &graph))
}

fn has_cycle(course: usize, status: &mut Vec<Status>, graph: &Vec<Vec<usize>>) -> bool {
    match status[course] {
        Status::Visited => false,
        Status::Visiting => true,
        _ => {
            status[course] = Status::Visiting;
            if graph[course]
                .iter()
                .any(|&next_course| has_cycle(next_course, status, graph))
            {
                return true;
            }
            status[course] = Status::Visited;
            false
        }
    }
}

#[test]
fn run() {
    let n = 2;
    let pre: Vec<Vec<i32>> = vec![vec![1, 0]];
    assert_eq!(can_finish(n, pre), true);

    let n = 2;
    let pre: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1]];
    assert_eq!(can_finish(n, pre), false);
}
