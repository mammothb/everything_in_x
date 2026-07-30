from typing import List


class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        n1 = len(nums1)
        n2 = len(nums2)
        if n1 > n2:
            nums1, nums2 = nums2, nums1
            n1, n2 = n2, n1

        half = (n1 + n2) // 2

        l = 0
        r = n1 - 1
        while True:
            mid = l + (r - l) // 2
            j = half - mid - 2

            left1 = nums1[mid] if mid >= 0 else float("-inf")
            right1 = nums1[mid + 1] if mid < n1 - 1 else float("inf")
            left2 = nums2[j] if j >= 0 else float("-inf")
            right2 = nums2[j + 1] if j < n2 - 1 else float("inf")

            if left1 <= right2 and left2 <= right1:
                if (n1 + n2) % 2 == 0:
                    return (max(left1, left2) + min(right1, right2)) / 2
                return min(right1, right2)
            elif left1 > right2:
                r = mid - 1
            else:
                l = mid + 1


def main(): ...


if __name__ == "__main__":
    main()
