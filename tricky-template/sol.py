from collections import Counter, defaultdict
from math import log2, gcd


for _ in range(int(input())):
    n = int(input())
    lists =[]
    for _ in range(3):
        lists.append(input())

    a, b , c = lists
    upper = c.upper()
    flag = True
    for i in range(n):
        curr_1 , curr_2 = a[i] , b[i]
        if c[i] != curr_1 and c[i] != curr_2:
            flag = False
            break

    print(['NO' , 'YES'][not flag])

