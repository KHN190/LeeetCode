# LC 317

class Solution:
    def shortestDistance(self, grid):
        row = len(grid)
        col = len(grid[0])

        self.grid = grid
        self.reach = [[0] * col for _ in range(row)]
        self.dist = [[0] * col for _ in range(row)]
        self.n = 0 # num of buildings
        self.ans = -1
        
        # flooding from each 1
        for r in range(row):
            for c in range(col):
                # 
                cur = self.grid[r][c]
                # 
                if cur == 1:
                    # start flooding
                    self.dist[r][c] = -1
                    self.n += 1
                    # 
                    self.flood(r, c)

                elif cur == 2:
                    self.dist[r][c] = -1


        flatten = []
        for r in range(row):
            for c in range(col):
                if self.dist[r][c] > 0 and self.reach[r][c] == self.n:
                    flatten.append(self.dist[r][c])

        if len(flatten) > 0:
            self.ans = min(flatten)

        return self.ans

    # bfs
    def flood(self, x, y):
        row = len(self.dist)
        col = len(self.dist[0])

        visited = set((x, y))
        q = [(x, y)]
        # 
        while q:
            cur = q.pop()
            # visit dirs
            for dy, dx in (-1, 0), (1, 0), (0, -1), (0, 1):
                r = cur[0] + dx
                c = cur[1] + dy
                pos = (r, c)

                # if in bound
                if 0 <= r < row and 0 <= c < col and pos not in visited:
                    # visit
                    visited.add(pos)
                    # dist
                    if self.grid[r][c] == 0:
                        #
                        self.dist[r][c] += abs(r - x) + abs(c - y) # WRONG, since we need bfs
                        self.reach[r][c] += 1
                        q.append(pos)
                    else:
                        # this is building / hill
                        self.dist[r][c] = -1
                        # no visit


grid = [[1,0,2,0,1],
        [0,0,0,0,0],
        [0,0,1,0,0]]
assert Solution().shortestDistance(grid) == 7

grid = [[1]]
assert Solution().shortestDistance(grid) == -1

grid = [[1,2,0]]
assert Solution().shortestDistance(grid) == -1

grid = [[1,2,0],
        [2,0,0]]
assert Solution().shortestDistance(grid) == -1

grid = [[1,2,0],
        [1,0,0]]
assert Solution().shortestDistance(grid) == -1 # since not all cells reachable
