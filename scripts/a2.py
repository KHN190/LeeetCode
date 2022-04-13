# Google
#
# union find, cluster

# with a graph, does 1 go to 5?
#
graph = [[1,2],[2,3],[3,4],[5,6],[7,2]]

class Cluster:

	def __init__(self):
		self.data = {} # { node: adjacent nodes }

	# find n's cluster parent node
	def find_parent(self, n):
		for (i, ns) in self.data.items():
			if i == n or n in ns:
				return i
		return None

	# find if n, m are in same cluster
	def is_connected(self, n, m):
		c1 = self.find_parent(n)
		c2 = self.find_parent(m)
		return c1 and c1 == c2

cluster = Cluster()

# build clusters
for ns in graph:
	node = cluster.find_parent(ns[0]) or cluster.find_parent(ns[1])
	# create new cluster
	if not node:
		cluster.data[ns[0]] = set([ns[0], ns[1]])
		continue
	# we have a cluster, add the nodes to the cluster
	cluster.data[node].add(ns[0])
	cluster.data[node].add(ns[1])


assert not cluster.is_connected(1, 5) # False
assert cluster.is_connected(1, 7) # True
assert cluster.is_connected(2, 7) # True
