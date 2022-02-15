use std::os::unix::raw::dev_t;

struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack {
            data: vec![],
        }
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
    }

    fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        self.data.pop()
    }

    fn peek(&self) -> Option<&T> {
        if self.data.is_empty() {
            return None;
        }
        self.data.get(self.data.len() - 1)
    }

    fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}


// 使用栈 —> 有效括号算法
fn par_check(par: &str) -> bool {
    let mut stack = Stack::new();
    for c in par.chars() {
        if c == '(' {
            stack.push(c)
        } else {
            if stack.is_empty() {
                return false;
            }
            stack.pop();
        }
    }
    return stack.is_empty();
}

// 使用栈进行二进制转换
fn divide_by_two(mut dec_num: u32) -> String {
    let mut stack = Stack::new();
    while dec_num > 0 {
        let rem = dec_num % 2;
        stack.push(rem);
        dec_num /= 2;
    }
    let mut ret = String::new();
    while !stack.is_empty() {
        let v = stack.pop().unwrap().to_string();
        ret.push_str(&v);
    }
    ret
}

#[test]
fn par_test() {
    let sa = "()(())";
    let sb = "()((()";
    println!("{},{}", par_check(sa), par_check(sb))
}

#[test]
fn divide_test(){
    let v = divide_by_two(10);
    println!("{}",v);
}




































