class Trie:
    def __init__(self):
        self.root = {}

    def add(self, word):
        curr = self.root
        for c in word:
            if c not in curr:
                curr[c] = {}
            curr = curr[c]
        curr["word"] = word

    def remove(self, word):
        stack = []
        curr = self.root
        for c in word:
            stack.append((c, curr))
            curr = curr[c]
        del curr["word"]
        while stack:
            c, node = stack.pop()
            if not node[c]:
                del node[c]


class Solution:
    def findWords(self, board: List[List[str]], words: List[str]) -> List[str]:
        trie = Trie()
        for word in words:
            trie.add(word)

        result = []
        nr = len(board)
        nc = len(board[0])
        for i in range(nr):
            for j in range(nc):
                if not trie.root:
                    return result
                self.dfs(board, nr, nc, i, j, trie, trie.root, result)

        return result

    def dfs(self, board, nr, nc, row, col, trie, trie_node, result):
        if not (0 <= row < nr and 0 <= col < nc):
            return

        c = board[row][col]
        if c == "#":
            return
        if c not in trie_node:
            return

        trie_node = trie_node[c]
        board[row][col] = "#"
        if "word" in trie_node:
            result.append(trie_node["word"])
            trie.remove(trie_node["word"])

        for i, j in ((row + 1, col), (row - 1, col), (row, col + 1), (row, col - 1)):
            self.dfs(board, nr, nc, i, j, trie, trie_node, result)

        board[row][col] = c


def main(): ...


if __name__ == "__main__":
    main()
