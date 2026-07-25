class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int nc = matrix[0].size();

        int t = 0;
        int b = matrix.size() - 1;
        while (t <= b) {
            int row = t + (b - t) / 2;
            if (matrix[row][0] <= target && target <= matrix[row][nc - 1]) {
                int l = 0;
                int r = nc - 1;
                while (l <= r) {
                    int mid = l + (r - l) / 2;
                    if (matrix[row][mid] == target) {
                        return true;
                    }
                    if (matrix[row][mid] < target) {
                        l = mid + 1;
                    } else {
                        r = mid - 1;
                    }
                }
            }
            if (matrix[row][nc - 1] < target) {
                t = row + 1;
            } else {
                b = row - 1;
            }
        }
        return false;
    }
};

int main() {
    return 0;
}
