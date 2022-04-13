# Amazon
#
# Given a list, find all sub arrays, with max - min <= k

# time, space O(n)
def countPossibleSegments(weights, k): 
    cnt = 0
    left = -1
    qmax, qmin = deque([]), deque([])
    for i in range(len(weights)):
        while qmax and weights[qmax[-1]] <= k: # mono descrease
            qmax.pop()
        qmax.append(i)
        while qmin and weights[qmin[-1]] >= k: # mono increase
            qmin.pop()
        qmin.append(i)

        while qmax and qmin and weights[qmax[0]] - weights[qmin[0]] > k: # qmax / qmin has i
            left = qmax.popleft() if qmax[0] < qmin[0] else qmin.popleft()
        cnt += i - left
    return cnt
