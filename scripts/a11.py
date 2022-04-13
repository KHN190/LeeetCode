# LC 249

class Solution:
    def groupStrings(self, strings):
        # same group: 
        #   len(s) and [c0-c1, c1-c2, ..]
        # dict of the traits
        # gen_traits(s)
        d = {}
        def gen_trait(s):
            l = []
            for i in range(1, len(s)):
                n = ord(s[i - 1]) - ord(s[i])
                while n < 0:
                    n += 26
                l.append(n)
            return tuple(l)
        
        for s in strings:
            trait = gen_trait(s)

            if d.get(trait):
                d[trait].append(s)
            else:
                d[trait] = [s]
        
        res = list(sorted(d.values()))
        return res

lst = list(sorted(["abc","bcd","acef","xyz","az","ba","a","z"]))
ans = list(sorted([["acef"],["a","z"],["abc","bcd","xyz"],["az","ba"]]))
assert Solution().groupStrings(lst) == ans

lst = ["a","b","ac","zb"]
ans = list(sorted([["a", "b"], ["ac", "zb"]]))
assert Solution().groupStrings(lst) == ans
