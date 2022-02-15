use std::marker::PhantomPinned;

struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(cap: usize) -> Self {
        Queue {
            cap,
            data: Vec::with_capacity(cap),
        }
    }

    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            return Err("No space.".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    fn dequeue(&mut self) -> Option<T> {
        if self.data.len() > 0 {
            return self.data.pop();
        }
        None
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

fn hot_potato(names: Vec<&str>, num: usize) -> &str {
    let mut q = Queue::new(names.len());
    for n in names {
        q.enqueue(n);
    }

    while q.size() > 1 {
        for _ in 0..num {
            let name = q.dequeue().unwrap();
            q.enqueue(name);
        }
        q.dequeue();
    }
    q.dequeue().unwrap()
}

#[test]
fn hot_potato_test(){
    let name = vec!["Shieber","David","Susan","Jane","Kew","Brad"];
    println!("{}",hot_potato(name,8));
}