class TimeMap:
    def __init__(self):
        self.data = {}

    def set(self, key: str, value: str, timestamp: int) -> None:
        self.data.setdefault(key, []).append((timestamp, value))

    def get(self, key: str, timestamp: int) -> str:
        values = self.data.get(key)
        if not values:
            return ""
        l = 0
        r = len(values) - 1
        idx = -1
        while l <= r:
            mid = l + (r - l) // 2
            if values[mid][0] <= timestamp:
                idx = mid
                l = mid + 1
            else:
                r = mid - 1
        return "" if idx == -1 else values[idx][1]


def main(): ...


if __name__ == "__main__":
    main()
