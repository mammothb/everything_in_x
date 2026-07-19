class MinStack:
    def __init__(self):
        self._data = []
        self._mins = []

    def push(self, val: int) -> None:
        self._data.append(val)
        if not self._mins:
            self._mins.append(val)
        else:
            self._mins.append(min(self.getMin(), val))

    def pop(self) -> None:
        self._data.pop()
        self._mins.pop()

    def top(self) -> int:
        return self._data[-1]

    def getMin(self) -> int:
        return self._mins[-1]


def main(): ...


if __name__ == "__main__":
    main()
