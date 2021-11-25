struct CombinationIterator {
    data: Vec<String>,
    index: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CombinationIterator {
    fn new(characters: String, combinationLength: i32) -> Self {
        fn helper(characters: &[char], length: usize, index: usize, cur: &mut String, ret: &mut Vec<String>) {
            if cur.len() == length {
                ret.push(cur.clone());
                return;
            }
            for i in index..characters.len() {
                cur.push(characters[i]);
                helper(characters, length, i + 1, cur, ret);
                cur.pop();
            }
        }
        let mut ret = vec![];
        let chars = characters.chars().collect::<Vec<_>>();
        helper(&chars, combinationLength as usize, 0, &mut String::new(), &mut ret);
        CombinationIterator {
            data: ret,
            index: 0,
        }
    }

    fn next(&mut self) -> String {
        let ret = self.data[self.index].clone();
        self.index += 1;
        ret
    }

    fn has_next(&self) -> bool {
        self.data.len() > self.index
    }
}

#[test]
fn test() {
    let mut c = CombinationIterator::new("abc".to_string(), 2);
    println!("{}", c.next());
}