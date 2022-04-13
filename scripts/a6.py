# LC 818
# https://leetcode.com/problems/race-car/submissions/
#
from collections import defaultdict, deque

# this is wrong
class Solution:
    def racecar(self, target):
        # newton method
        # once it's over the target, then R
        # otherwise, A
        self.seq = ''
        self.pos = 0
        self.vel = 1

        while target != self.pos:
            if not self.overshoot(target):
                self.acc()
            else:
                self.rev()
                self.acc()

        print(self.seq)
        return len(self.seq)

    def acc(self):
        self.seq += 'A'
        self.pos += self.vel
        self.vel *= 2

    def rev(self):
        self.seq += 'R'
        self.vel = -1 if self.vel > 0 else 1

    def overshoot(self, target):
        if self.vel > 0:
            return self.pos > target
        return self.pos < target


# correct
class Solution:
    def racecar(self, target: int) -> int:
        dq = deque([(0,1,0)]) # (pos, speed, step)
        while dq:
            pos,speed,step = dq.popleft()
            if pos == target:
                return step
            #accelerate
            dq.append((pos+speed, speed*2, step+1))
            #reverse
            if speed > 0:
                dq.append((pos,-1,step+1))
            else:
                dq.append((pos,1, step+1))


assert Solution().racecar(4) == 5 # AARRA, not AAARAA
assert Solution().racecar(6) == 5
assert Solution().racecar(1) == 1
