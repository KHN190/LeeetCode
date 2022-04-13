# Google
#
# Given a number N, find all even pairs sum up to it
# e.g. 
#  6 -> [2, 4], [4, 2]

# if n % 2 != 0: []
# 

def pairs(n):
	res = []
	if n % 2 != 0:
		return res
	# pivot
	mid = n // 2
	for i in range(1, mid // 2 + 1):
		res.append([i * 2, n - i * 2])
	return res

assert pairs(6) == [[2,4]] # [2,4]
assert pairs(8) == [[2,6], [4,4]] # [2,6], [4,4]
assert pairs(10) == [[2,8], [4,6]] # [2,8], [4,6]
# print(pairs(12))
