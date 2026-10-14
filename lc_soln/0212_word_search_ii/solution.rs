struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    index: i32,
    ref_count: i32,
}

impl TrieNode {
    fn new() -> Self {
        Self {
            children: Default::default(),
            index: -1,
            ref_count: 0,
        }
    }

    fn add(&mut self, word: &str, i: i32) {
        let mut curr = self;
        curr.ref_count += 1;
        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            curr = curr.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
            curr.ref_count += 1;
        }
        curr.index = i;
    }

    fn remove(&mut self, word: &str) {
        let mut curr = self;
        curr.ref_count -= 1;

        for c in word.bytes() {
            let idx = (c - b'a') as usize;
            let should_prune = {
                let child = curr.children[idx].as_deref_mut().unwrap();
                child.ref_count -= 1;
                child.ref_count == 0
            };

            if should_prune {
                curr.children[idx] = None;
                return;
            }
            curr = curr.children[idx].as_deref_mut().unwrap();
        }
    }
}

impl Solution {
    pub fn find_words(mut board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::new();
        for (i, word) in words.iter().enumerate() {
            root.add(word, i as i32);
        }
        let mut result = Vec::new();
        let nr = board.len();
        let nc = board[0].len();
        for i in 0..nr {
            for j in 0..nc {
                Self::dfs(
                    &mut board,
                    i as i32,
                    j as i32,
                    &mut root,
                    &words,
                    &mut result,
                );
            }
        }
        result
    }

    fn dfs(
        board: &mut Vec<Vec<char>>,
        row: i32,
        col: i32,
        root: &mut TrieNode,
        words: &[String],
        result: &mut Vec<String>,
    ) {
        if !(0 <= row && row < board.len() as i32 && 0 <= col && col < board[0].len() as i32) {
            return;
        }
        let (ii, jj) = (row as usize, col as usize);
        let c = board[ii][jj];
        if c == '#' {
            return;
        }
        let idx = (c as u8 - b'a') as usize;
        if root.children[idx].is_none() {
            return;
        }

        let child = root.children[idx].as_mut().unwrap();
        if child.index != -1 {
            result.push(words[child.index as usize].clone());
            child.index = -1;
        }

        board[ii][jj] = '#';
        Self::dfs(board, row - 1, col, child, words, result);
        Self::dfs(board, row + 1, col, child, words, result);
        Self::dfs(board, row, col - 1, child, words, result);
        Self::dfs(board, row, col + 1, child, words, result);
        board[ii][jj] = c;
    }
}

fn main() {}
