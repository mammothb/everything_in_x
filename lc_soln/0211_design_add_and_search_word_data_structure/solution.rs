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

struct WordDictionary {
    root: TrieNode,
}

impl WordDictionary {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            curr = curr.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        curr.end_of_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        Self::dfs(&self.root, word.as_bytes(), 0)
    }

    fn dfs(root: &TrieNode, word: &[u8], pos: usize) -> bool {
        if pos == word.len() {
            return root.end_of_word;
        }
        if word[pos] == b'.' {
            for child in &root.children {
                if let Some(node) = child && Self::dfs(node, word, pos + 1) {
                    return true;
                }
            }
            return false;
        }
        let idx = (word[pos] - b'a') as usize;
        if let Some(ref node) = root.children[idx] {
            return Self::dfs(node, word, pos + 1);
        }
        return false;
    }
}

fn main() {
}
