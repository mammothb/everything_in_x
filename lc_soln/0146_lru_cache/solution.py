class Node:
    def __init__(self, key: int = -1, val: int = -1):
        self.key = key
        self.val = val
        self.prev = None
        self.next = None


def remove_node(node: Node) -> Node:
    node.prev.next = node.next
    node.next.prev = node.prev
    node.prev = None
    node.next = None
    return node


def insert_after(node: Node, target: Node) -> Node:
    node.prev = target
    node.next = target.next
    target.next.prev = node
    target.next = node
    return node


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.size = 0
        self.head = Node()
        self.tail = Node()
        self.head.next = self.tail
        self.tail.prev = self.head

        self.key_to_node = {}

    def get(self, key: int) -> int:
        node = self.key_to_node.get(key)
        if node is None:
            return -1
        node = remove_node(node)
        node = insert_after(node, self.head)
        return node.val

    def put(self, key: int, value: int) -> None:
        node = self.key_to_node.get(key)
        if node is None:
            if self.size == self.capacity:
                removed = remove_node(self.tail.prev)
                del self.key_to_node[removed.key]
                self.size -= 1

            node = Node(key=key, val=value)
            node = insert_after(node, self.head)
            self.key_to_node[key] = node
            self.size += 1
        else:
            node.val = value
            node = remove_node(node)
            node = insert_after(node, self.head)


def main(): ...


if __name__ == "__main__":
    main()
