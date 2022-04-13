from lib.types import TreeNode

# LC 199

class Solution:
    def rightSideView(self, root):
        stack = [(root, 0)]
        res = []
        while stack:
            # left, then visit right
            cur, depth = stack.pop(0)
            # ignore null node
            if cur is None:
                continue
            if len(res) == depth:
                # add new value
                res.append(cur.val)
            else:
                # replace cur depth value
                res[depth] = cur.val

            stack.append((cur.left, depth + 1))
            stack.append((cur.right, depth + 1))

        return res
