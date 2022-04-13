class Solution:
    def isStrobogrammatic(self, num):
        strobo = {1: 1, 6: 9, 8: 8, 9: 6, 0: 0}
        res = []
        for n in num:
            m = strobo.get(int(n))
            if m == None:
                return False
            res.append(str(m))

        res.reverse()
        res = ''.join(res)

        return res == num

assert Solution().isStrobogrammatic('69')
assert Solution().isStrobogrammatic('88')
assert Solution().isStrobogrammatic('1691')
assert not Solution().isStrobogrammatic('121')