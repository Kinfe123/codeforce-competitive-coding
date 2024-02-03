for _ in range(int(input())):
    n , x = map(int , input().split())
    lists = list(map(int , input().split()))
    max_ = (x - lists[-1]) * 2
    for i in range(1 , n):
        max_ = max(max_ , lists[i] - lists[i-1])
    print(max_)
    
