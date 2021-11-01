#[derive(Default)]
struct Trie {
    child: [Option<Box<Trie>>; 26],
    is_end: bool,
}

impl Trie {
    fn new() -> Self {
        Trie::default()
    }

    fn insert(&mut self, word: String) {
        let mut cur = self;
        for i in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            cur = cur.child[i].get_or_insert_with(|| Box::new(Trie::new()));
        }
        cur.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut cur = self;
        for i in word.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            match cur.child[i].as_ref() {
                Some(node) => cur = node,
                None => return false,
            }
        }
        cur.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut cur = self;
        for i in prefix.chars().map(|ch| (ch as u8 - 'a' as u8) as usize) {
            match cur.child[i].as_ref() {
                Some(node) => cur = node,
                None => return false,
            }
        }
        true
    }
}