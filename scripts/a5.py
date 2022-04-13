# LC 1537
#
# https://leetcode.com/problems/get-the-maximum-score/submissions/

# process from the end,
# same if start from the head, just make sure the remained sum is max
#
def maxSum(nums1, nums2):
    v1 = v2 = 0
    while nums2 and nums1:
        if nums1[-1] > nums2[-1]:
            v1 += nums1.pop()
        elif nums1[-1] < nums2[-1]:
            v2 += nums2.pop()
        else:
            v1 = v2 = max(v1, v2) + nums1.pop()
            nums2.pop()
    v2 += sum(nums2)
    v1 += sum(nums1)
    return max(v1,v2) % 1000000007


# test cases

n1 = [2,4,5,8,10]
n2 = [4,6,8,9]

assert maxSum(n1, n2) == 30

n1 = [1,3,5,7,9]
n2 = [3,5,100]

assert maxSum(n1, n2) == 109

n1 = [1,2,3,4,5]
n2 = [6,7,8,9,10]

assert maxSum(n1, n2) == 40