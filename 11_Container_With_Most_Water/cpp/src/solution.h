// solution.h
#include <vector>
#include <algorithm>

class Solution {
public:
  int maxArea(std::vector<int> &height) {
    int left = 0;
    int right = height.size() - 1;
    int maxVol = 0;
    int vol = 0;
    while (left < right) {
      vol = (right - left) * std::min(height[left], height[right]);
      maxVol = std::max(vol, maxVol);
      if (height[left] > height[right]) {
        right--;
      } else {
        left++;
      }
    }
    return maxVol;
  }
};