from collections import Counter, defaultdict
from math import log2, gcd


for _ in range(int(input())):
    cords = []
    for _ in range(4):
        a , b = map(int , input().split())
        cords.append((a , b))

    sorted_one= sorted(cords)
    w , h = abs(sorted_one[1][1] - sorted_one[0][1]) , abs(sorted_one[-1][1] - sorted_one[-2][1])
    print(w*h)    

