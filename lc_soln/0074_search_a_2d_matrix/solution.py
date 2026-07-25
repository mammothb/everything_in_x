from typing import List


class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        nr = len(matrix)
        nc = len(matrix[0])

        row = -1
        t = 0
        b = nr - 1
        while t <= b:
            mid = t + (b - t) // 2
            if matrix[mid][0] <= target <= matrix[mid][-1]:
                row = mid
                break
            if matrix[mid][-1] < target:
                t = mid + 1
            else:
                b = mid - 1

        if row == -1:
            return False

        l = 0
        r = nc - 1
        while l <= r:
            mid = l + (r - l) // 2
            if matrix[row][mid] == target:
                return True
            if matrix[row][mid] < target:
                l = mid + 1
            else:
                r = mid - 1

        return False


def main(): ...


if __name__ == "__main__":
    main()
