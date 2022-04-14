# LC 339. Nested list weight sum

from lib.types import NestedInteger

class Solution:
    def depthSum(self, nestedList):
        return self.recursive(nestedList, 1)

    def recursive(self, nestedList, level):
        res = 0
        for e in nestedList:
            if e.isInteger():
                res += level * e.getInteger()
            else:
                res += self.recursive(e.getList(), level + 1)
        return res
