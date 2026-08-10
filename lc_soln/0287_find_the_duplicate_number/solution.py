class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        fast = nums[0]
        slow = nums[0]
        while True:
            fast = nums[nums[fast]]
            slow = nums[slow]
            if fast == slow:
                slow2 = nums[0]
                while slow != slow2:
                    slow = nums[slow]
                    slow2 = nums[slow2]
                return slow


def main(): ...


if __name__ == "__main__":
    main()
