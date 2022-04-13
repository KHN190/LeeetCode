# LC 498
#
# Watch out for trivial cases

class Solution:

    def findDiagonalOrder(self, mat):
        m, n = len(mat), len(mat[0])
        # cur position
        i, j = 0, 0
        # cur dir
        dir = [-1, 1]
        # @notice doesn't work with range(0, m * n)
        ans = [mat[0][0]]
        for _ in range(1, n * m):
            # next pos
            i, j = (i + dir[0], j + dir[1])
            # just flip direction, fix pos
            if j > n - 1:
                # out of right
                i, j = i + 2, j - 1
                dir.reverse()
            elif i > m - 1:
                # out of bottom
                i, j = i - 1, j + 2
                dir.reverse()
            elif i < 0:
                # out of top
                i = 0 
                dir.reverse()
            elif j < 0:
                # out of left
                j = 0
                dir.reverse()

            ans.append(mat[i][j])

        return ans


mat = [[1,2,3],[4,5,6],[7,8,9]]

assert Solution().findDiagonalOrder(mat) == [1,2,4,7,5,3,6,8,9]