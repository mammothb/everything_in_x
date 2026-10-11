struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            end_of_word: false,
        }
    }
}

struct PrefixTree {
    root: TrieNode,
}

impl PrefixTree {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            curr = curr.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        curr.end_of_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut curr = &self.root;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            match &curr.children[idx] {
                Some(node) => curr = node,
                None => return false,
            }
        }
        curr.end_of_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut curr = &self.root;
        for c in prefix.bytes() {
            let idx = (c - b'a') as usize;
            match &curr.children[idx] {
                Some(node) => curr = node,
                None => return false,
            }
        }
        true
    }
}

fn main() {}
