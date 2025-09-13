from typing import List


class Solution:
    def sort_features(self, features: List[str], responses: List[str]) -> List[str]:
        popularity = collections.Counter(features)
        for response in responses:
            for word in set(response.split()):
                popularity[word] += 1
        return sorted(features, key=lambda f: popularity[f], reverse=True)
