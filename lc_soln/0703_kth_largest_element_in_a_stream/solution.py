class KthLargest:
    def __init__(self, k: int, nums: List[int]):
        self.k = k
        self.h = nums
        heapq.heapify(self.h)
        while len(self.h) > self.k:
            heapq.heappop(self.h)

    def add(self, val: int) -> int:
        if len(self.h) == self.k:
            heapq.heappushpop(self.h, val)
        else:
            heapq.heappush(self.h, val)

        return self.h[0]


def main(): ...


if __name__ == "__main__":
    main()
