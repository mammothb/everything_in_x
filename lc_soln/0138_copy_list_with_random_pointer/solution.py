"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""


class Solution:
    def copyRandomList(self, head: "Optional[Node]") -> "Optional[Node]":
        if head is None:
            return None

        new_head = Node(head.val)
        old_to_new = {head: new_head}

        curr = head
        new_curr = new_head
        while curr is not None:
            if curr.next is not None:
                if curr.next in old_to_new:
                    new_next = old_to_new[curr.next]
                else:
                    new_next = Node(curr.next.val)
                    old_to_new[curr.next] = new_next
                new_curr.next = new_next
            if curr.random is not None:
                if curr.random in old_to_new:
                    new_random = old_to_new[curr.random]
                else:
                    new_random = Node(curr.random.val)
                    old_to_new[curr.random] = new_random
                new_curr.random = new_random
            curr = curr.next
            new_curr = new_curr.next
        return new_head


def main(): ...


if __name__ == "__main__":
    main()
