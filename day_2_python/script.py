file = open("input.txt")

valid_count = 0

for line in file:
    valid_line = True
    nums = line.split()
    if len(nums) < 2:
        continue
    direction = (int(nums[0]) - int(nums[1])) > 0
    for i in range(len(nums) - 1):
        curr_num = int(nums[i])
        next_num = int(nums[i + 1])

        jump = abs(curr_num - next_num)
        next_direction = (curr_num - next_num) > 0

        valid_jump = jump < 4 and jump != 0
        valid_direction = direction == next_direction

        if not valid_jump or not valid_direction:
            valid_line = False
            break
    
    if valid_line:
        valid_count += 1

print(valid_count)

