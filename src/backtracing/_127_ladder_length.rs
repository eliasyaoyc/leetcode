use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        fn diff(from: &String, to: &String) -> bool {
            if from.len() != to.len() {
                return false;
            }
            let mut diff = 0;
            for i in 0..from.len() {
                if from.as_bytes()[i] != to.as_bytes()[i] {
                    diff += 1;
                    if diff > 1 {
                        return false;
                    }
                }
            }
            diff == 1
        }

        if !word_list.contains(&end_word) {
            return 0;
        }
        let mut step = 1;
        let mut visited = vec![];
        let mut queue = VecDeque::new();
        queue.push_back(begin_word.clone());
        visited.push(begin_word.clone());

        while !queue.is_empty() {
            let len = queue.len();
            for i in 0..len {
                if let Some(node) = queue.pop_front() {
                    for x in &word_list {
                        if diff(&node, &x) && !visited.contains(x) {
                            if *x == end_word {
                                return step + 1;
                            }
                            queue.push_back(x.clone());
                            visited.push(x.clone());
                        }
                    }
                }
            }
            step += 1;
        }
        0
    }
}