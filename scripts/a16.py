# Meta. Maximize min nums in array.

import collections

def maximizeMin(nums, k):
	# num freq
	d = collections.defaultdict(lambda: 0)
	for i, n in enumerate(nums):
		d[n] += 1
	
	cur = min(d.keys())
	while k > 0:
		k -= 1
		# update min and min+1 freq
		d[cur] -= 1
		d[cur + 1] += 1

		# update min, if freq[min] == 0
		if d[cur] == 0:
			cur += 1

	return cur

assert maximizeMin([1, 100], 1) == 2
assert maximizeMin([1, 2, 1, 3], 3) == 2
assert maximizeMin([1, 2, 3], 4) == 3