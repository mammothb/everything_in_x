struct TrieNode {
    TrieNode* children[26];
    string word = "";
};

class Trie {
   public:
    TrieNode* root = new TrieNode();
    int count = 0;

    void add(const string& word) {
        TrieNode* curr = root;
        for (char c : word) {
            int idx = c - 'a';
            if (curr->children[idx] == nullptr) {
                curr->children[idx] = new TrieNode();
            }
            curr = curr->children[idx];
        }
        curr->word = word;
        count++;
    }

    void remove(const string& word) {
        TrieNode* curr = root;
        std::stack<TrieNode*> nodes;
        std::stack<int> indices;
        for (char c : word) {
            nodes.push(curr);
            int idx = c - 'a';
            indices.push(idx);
            curr = curr->children[idx];
        }
        curr->word = "";
        count--;
    }
};

class Solution {
    int dirs[5] = {0, 1, 0, -1, 0};

   public:
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        Trie trie;
        for (const string& word : words) {
            trie.add(word);
        }
        std::vector<std::string> result;
        int nr = board.size();
        int nc = board[0].size();
        for (int i = 0; i < nr; i++) {
            for (int j = 0; j < nc; j++) {
                if (trie.count == 0) {
                    return result;
                }
                dfs(board, nr, nc, i, j, trie, trie.root, result);
            }
        }
        return result;
    }

    void dfs(std::vector<std::vector<char>>& board, int nr, int nc, int row, int col, Trie& trie,
             TrieNode* trie_node, std::vector<std::string>& result) {
        if (!(0 <= row && row < nr && 0 <= col && col < nc)) {
            return;
        }
        char c = board[row][col];
        if (c == '#') {
            return;
        }
        int idx = c - 'a';
        if (trie_node->children[idx] == nullptr) {
            return;
        }

        board[row][col] = '#';
        trie_node = trie_node->children[idx];
        if (!trie_node->word.empty()) {
            result.push_back(trie_node->word);
            trie.remove(trie_node->word);
        }

        for (int k = 0; k < 4; k++) {
            int i = row + dirs[k];
            int j = col + dirs[k + 1];
            dfs(board, nr, nc, i, j, trie, trie_node, result);
        }

        board[row][col] = c;
    }
};

int main() {
    return 0;
}
