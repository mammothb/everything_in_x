struct TrieNode {
    TrieNode* children[26];
    bool end_of_word = false;
};

class WordDictionary {
    TrieNode* root;

   public:
    WordDictionary() { root = new TrieNode(); }

    void addWord(string word) {
        TrieNode* curr = root;
        for (char c : word) {
            int idx = c - 'a';
            if (curr->children[idx] == nullptr) {
                curr->children[idx] = new TrieNode();
            }
            curr = curr->children[idx];
        }
        curr->end_of_word = true;
    }

    bool search(string word) { return dfs(root, word, 0); }

    bool dfs(TrieNode* node, string word, int pos) {
        if (pos == word.size()) {
            return node->end_of_word;
        }
        if (word[pos] == '.') {
            for (int i = 0; i < 26; i++) {
                if (node->children[i] != nullptr && dfs(node->children[i], word, pos + 1)) {
                    return true;
                }
            }
            return false;
        }

        int idx = word[pos] - 'a';
        if (node->children[idx] != nullptr && dfs(node->children[idx], word, pos + 1)) {
            return true;
        }
        return false;
    }
};

int main() {
    return 0;
}
