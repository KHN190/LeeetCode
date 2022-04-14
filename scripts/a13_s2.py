class Solution(object):
    def shortestDistance(self, grid):
        if not grid or not grid[0]:
            return -1

        self.grid = grid
        self.row = len(grid)
        self.col = len(grid[0])
        # 
        self.matrix = [[[0,0] for i in range(self.col)] for j in range(self.row)]

        # count building
        n = 0
        for i in range(self.row):
            for j in range(self.col):
                if grid[i][j] == 1:
                    self.bfs([i,j], n)
                    n += 1

        res = float('inf')
        for i in range(self.row):
            for j in range(self.col):
                if self.matrix[i][j][1]==n:
                    res = min(res, self.matrix[i][j][0])

        return res if res < float('inf') else -1

    def bfs(self, start, n):
        q = [(start, 0)]
        while q:
            cur = q.pop(0)
            pos, step = cur[0], cur[1]
            for dx, dy in [(-1,0), (1,0), (0,1), (0,-1)]:
                # next pos
                i, j = pos[0] + dx, pos[1] + dy
                # if in bound, reachable, and is 0
                if 0 <= i < self.row and 0 <= j < self.col \
                and self.matrix[i][j][1] == n \
                and self.grid[i][j] == 0:

                    self.matrix[i][j][0] += step + 1
                    self.matrix[i][j][1] = n + 1
                    q.append(([i, j], step + 1))


grid = [[0,1,1,0,1],
        [1,0,1,0,1],
        [1,0,0,0,1],
        [0,1,1,1,0]]
assert Solution().shortestDistance(grid) == 27

grid = [[1,1,1,1,1,0],
        [0,0,0,0,0,1],
        [0,1,1,0,0,1],
        [1,0,0,1,0,1],
        [1,0,1,0,0,1],
        [1,0,0,0,0,1],
        [0,1,1,1,1,0]]
assert Solution().shortestDistance(grid) == 88
