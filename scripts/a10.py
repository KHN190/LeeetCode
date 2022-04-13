# LC 16

class Solution(object):

    def threeSumClosest(self, nums, target):
        nums = sorted(nums)
        curr = nums[0] + nums[1] + nums[len(nums)-1]
        for i in range(len(nums)-2):
            # skip dup nums
            if i > 0 and nums[i] == nums[i-1]:
                continue
            # try all combinations
            l = i + 1
            r = len(nums) - 1
            while l < r:
                val = nums[i] + nums[l] + nums[r]
                if abs(val - target) < abs(curr - target):
                    curr = val
                # if we have found exact sum
                if val == target:
                    return target
                # < target? find larger num
                # > target? find smaller num
                if val < target:
                    l += 1
                else:
                    r -= 1
        # the sum of 3
        return curr

assert Solution().threeSumClosest([-1, 2, 1, -4], 1) == 2
