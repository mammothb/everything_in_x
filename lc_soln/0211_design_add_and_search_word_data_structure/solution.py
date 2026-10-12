class TrieNode:
    def __init__(self):
        self.children = [None for _ in range(26)]
        self.end_of_word = False


class WordDictionary:
    def __init__(self):
        self.root = TrieNode()

    def addWord(self, word: str) -> None:
        curr = self.root
        for c in word:
            idx = ord(c) - ord("a")
            if curr.children[idx] is None:
                curr.children[idx] = TrieNode()
            curr = curr.children[idx]
        curr.end_of_word = True

    def search(self, word: str) -> bool:
        candidates = deque([self.root])
        for c in word:
            n = len(candidates)
            for _ in range(n):
                candidate = candidates.popleft()
                if c == ".":
                    for node in candidate.children:
                        if node is None:
                            continue
                        candidates.append(node)
                else:
                    idx = ord(c) - ord("a")
                    if candidate.children[idx] is not None:
                        candidates.append(candidate.children[idx])
        return any(c.end_of_word for c in candidates)


class WordDictionary:
    def __init__(self):
        self.root = {}

    def addWord(self, word: str) -> None:
        curr = self.root
        for c in word:
            if c not in curr:
                curr[c] = {}
            curr = curr[c]
        curr["end_of_word"] = True

    def search(self, word: str) -> bool:
        return self.dfs(self.root, word, 0)

    def dfs(self, node, word, idx):
        if len(word) == idx:
            return node.get("end_of_word", False)
        c = word[idx]
        if c == ".":
            return any(
                self.dfs(v, word, idx + 1)
                for k, v in node.items()
                if k != "end_of_word"
            )
        if c in node:
            return self.dfs(node[c], word, idx + 1)
        return False


def main(): ...


if __name__ == "__main__":
    main()
