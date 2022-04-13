# Google
#
# Given a string, find longest substring, where the substring's chars occur same times.
#
# https://www.geeksforgeeks.org/longest-sub-array-with-equal-number-of-alphabets-and-numeric-characters/

s = 'abcdc' # 'abcd'
s = 'aaccb' # 'aacc'
s = 'aacbbca' # 'acbbca' or 'aacbbc'

# start with ''
#
# a substr can extend to longer,
#
# the substr also has: 
# 	occur for all chars is n
#   set of chars
#
# merge 2 adjacent substr s1, s2 if
#   s1.chars() == s2.chars() and
#   s1.n == s2.n
# 

class Substr:
	def __init__(self, s):
		self.s = s
		self.freq = count_freq(s)

	def clone(self):
		import copy
		return copy.deepcopy(self)

	def append(self, s):
		self.s += s
		for c in s:
			self.freq[c] += 1

	def is_valid(self):
		ns = set(self.freq.values())
		return len(ns) == 1 or (len(ns) == 2 and 0 in ns)

	def chars(self):
		return set([x for x in self.freq if self.freq[x] > 0])

	def __repr__(self):
		return self.s + ' ' + str(self.is_valid())[0]


def count_freq(s):
	freq = {}
	for _ in 'abcdefghijklmnopqrstuvwxyz':
		freq[_] = 0
	for c in s:
		freq[c] += 1
	return freq


def find_longest_substr(s):
	ss = []
	for i, c in enumerate(s):
		# all new sub strings
		for x in range(i):
			ss.append(Substr(s[x:i]))
		ss.append(Substr(c))

	# print(len(ss))
	# find longest substring
	res = ''

	for x in ss:
		n = len(x.chars())
		if x.is_valid() and len(x.s) > len(res):
			res = x.s
	return res

assert find_longest_substr('abcdc') == 'abcd' # 'abcd'
assert find_longest_substr('aaccb') == 'aacc' # 'aacc'
assert find_longest_substr('aacbbca') == 'aacbbc' # 'acbbca' or 'aacbbc'
assert find_longest_substr('aaaaaaaaaaacbbcacbcbcbcbcbcbcbcbcb') == 'aaaaaaaaacbbcacbcbcbcbcbcbcbcb'
