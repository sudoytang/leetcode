
struct TrieNode {
    is_leaf: bool,
    sub: [Option<Box<TrieNode>>; 26],
}

#[allow(unused)]
struct Trie {
    root: [Option<Box<TrieNode>>; 26],
}


#[allow(unused)]
/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Self {
            root: [const { None }; 26]
        }
    }

    fn insert(&mut self, word: String) {
        let len = word.len();
        let mut p: &mut [Option<Box<TrieNode>>; 26] = &mut self.root;
        for (i, ch) in word.chars().enumerate() {
            let index = ch as usize - 'a' as usize;
            match &mut p[index] {
                Some(n) => {
                    if i + 1 == len {
                        n.is_leaf = true;
                    }
                    p = &mut n.sub;
                }
                none => {
                    *none = Some(Box::new(TrieNode { is_leaf: i + 1 == len, sub: [const { None }; 26] }));
                    p = &mut none.as_mut().unwrap().sub;
                }
            }
        }
    }

    fn search(&self, word: String) -> bool {
        let len = word.len();
        let mut p: &[Option<Box<TrieNode>>; 26] = &self.root;
        for (i, ch) in word.chars().enumerate() {
            let index = ch as usize - 'a' as usize;
            match &p[index] {
                Some(n) => {
                    if i + 1 == len {
                        return n.is_leaf
                    }
                    p = &n.sub;
                }
                None => {
                    return false;
                }
            }
        }
        unreachable!()
    }

    fn starts_with(&self, prefix: String) -> bool {
        let len = prefix.len();
        let mut p: &[Option<Box<TrieNode>>; 26] = &self.root;
        for (i, ch) in prefix.chars().enumerate() {
            let index = ch as usize - 'a' as usize;
            match &p[index] {
                Some(n) => {
                    if i + 1 == len {
                        return true;
                    }
                    p = &n.sub;
                }
                None => {
                    return false;
                }
            }
        }
        unreachable!()
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
#[allow(unused)]
 struct PH;