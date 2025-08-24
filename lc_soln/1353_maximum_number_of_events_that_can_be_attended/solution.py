import heapq
from typing import List


class Solution:
    def maxEvents(self, events: List[List[int]]) -> int:
        events = sorted(events, key=lambda x: x[0])
        h = []
        day = 1
        i = 0
        n = len(events)
        result = 0
        while i < n or h:
            while h and h[0] < day:
                heapq.heappop(h)

            while i < n and events[i][0] == day:
                heapq.heappush(h, events[i][1])
                i += 1

            if h:
                heapq.heappop(h)
                result += 1

            day += 1
        return result
