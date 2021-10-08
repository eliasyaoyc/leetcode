use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};
use rand::{Rng, thread_rng};

const DEFULT_HIGH: u32 = 32;
const DEFAULT_HIGH_FACTOR: f32 = 0.25;

struct Skiplist {
    head: *const Node,
    max_hegh: u32,
}

struct Node {
    val: i32,
    high: u32,
    next_nodes: [AtomicPtr<Node>; 0],
}

impl Node {
    #[inline]
    unsafe fn get_next(&self, high: u32) -> *mut Node {
        self.next_nodes.get_unchecked((high - 1) as usize).load(Ordering::Acquire)
    }
}

impl Skiplist {
    fn add(&self, num: i32) {
        let high = self.random_high();
    }

    fn find_less_than(&self, num: i32, prev_nodes: Option<&[*const Node]>) {
        let high = self.max_hegh;
        let p = self.head;
        loop {
            unsafe {
                let next_node = (*p).get_next(high);
            }
        }
    }

    fn random_high(&self) -> u32 {
        let mut high = 1;
        while rand::thread_rng().gen::<f32>() < DEFAULT_HIGH_FACTOR && self.max_hegh < DEFULT_HIGH {
            high += 1;
        }
        high
    }
}