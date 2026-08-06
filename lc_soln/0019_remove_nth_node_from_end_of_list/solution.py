# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        curr = head
        count = 0
        while curr is not None:
            curr = curr.next
            count += 1

        prev = None
        curr = head
        while count > n:
            prev = curr
            curr = curr.next
            count -= 1

        if prev is None:
            return head.next
        prev.next = curr.next
        return head


def main(): ...


if __name__ == "__main__":
    main()
