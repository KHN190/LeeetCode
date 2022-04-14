# LC 451. Meta

def frequencySort(s):
    ns = collections.Counter(s)
    tmp = sorted([x for x in ns.items()], key=lambda x: x[1], reverse=True)
    res = ''
    for x in tmp:
        res += x[0] * x[1]
    return res

assert frequencySort('abbabc') == 'bbbaac'
