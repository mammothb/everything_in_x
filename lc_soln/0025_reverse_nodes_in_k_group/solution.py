# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next


class Solution:
    def reverseKGroup(self, head: Optional[ListNode], k: int) -> Optional[ListNode]:
        curr = head
        count = 0
        while count < k and curr is not None:
            curr = curr.next
            count += 1
        if count < k:
            return head
        curr = self.reverseKGroup(curr, k)
        while count > 0:
            tmp = head.next
            head.next = curr
            curr = head
            head = tmp
            count -= 1
        return curr


def main(): ...


if __name__ == "__main__":
    main()
