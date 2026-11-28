class Solution:
    def kClosest(self, points: List[List[int]], k: int) -> List[List[int]]:
        h = []
        for x, y in points:
            dist = x * x + y * y
            if len(h) == k:
                heapq.heappushpop_max(h, (dist, x, y))
            else:
                heapq.heappush_max(h, (dist, x, y))

        return [[x, y] for _, x, y in h]


def main(): ...


if __name__ == "__main__":
    main()
