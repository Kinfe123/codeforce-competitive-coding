for _ in range(int(input())):
    n , x = map(int , input().split())
    lists = list(map(int , input().split()))
    max_ = lists[0]
    collect = []
    for i in range(1 , n):
        diff = lists[i] - lists[i-1]
        collect.append(diff)
        max_ = max(max_ , diff)
    
  
    mj =  (x - lists[-1])*2
    print(max(max_ , mj ))
    
