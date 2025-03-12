import sys

data = open(sys.argv[1]).read().strip()
lines = data.split('\n')

ans = 0
for line in lines:
    num_chars = len(line)
    num_bytes = len(line.encode('utf-8'))
    # print(num_chars, num_bytes)
    if num_chars <= 140 and num_bytes <= 160:
        ans += 13
    elif num_chars <= 140:
        ans += 7
    elif num_bytes <= 160:
        ans += 11
print(ans)
