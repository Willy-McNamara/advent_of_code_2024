file = open("input.txt")

list1 = []
list2 = []

for line in file:
    x = line.split()
    list1.append(int(x[0]))
    list2.append(int(x[1]))

list1.sort()
list2.sort()

diff = 0

for i, item in enumerate(list1):
    if item > list2[i]:
        diff += item - list2[i]
    else:
        diff += list2[i] - item 
    
print(diff)


