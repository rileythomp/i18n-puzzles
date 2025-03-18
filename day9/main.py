from collections import defaultdict
import sys

data = open(sys.argv[1]).read().strip()
lines = data.split('\n')

name_dates = defaultdict(list)
for i, line in enumerate(lines):
    date, names = line.split(': ')
    date_parts = [int(x) for x in date.split('-')]
    names = names.split(', ')
    for name in names:
        name_dates[name].append(date_parts)

month_days = {
    1: 31,
    2: 28,
    3: 31,
    4: 30,
    5: 31,
    6: 30,
    7: 31,
    8: 31,
    9: 30,
    10: 31,
    11: 30,
    12: 31,
}

def in_format(date, format, name):
    a, b, c = date
    if format == 'dmy' and (b not in month_days or a > month_days[b] + (1 if b == 2 and c%4 == 0 else 0) or a < 1):
        return False
    elif format == 'mdy' and (a not in month_days or b > month_days[a] + (1 if a == 2 and c%4 == 0 else 0) or b < 1):
        return False
    elif format == 'ymd' and (b not in month_days or c > month_days[b] + (1 if b == 2 and a%4 == 0 else 0) or c < 1):
        return False
    elif format == 'ydm' and (c not in month_days or b > month_days[c] + (1 if c == 2 and a%4 == 0 else 0) or b < 1):
        return False
    return True

formats = ['dmy', 'mdy', 'ymd', 'ydm']

ans_names = []

for name, dates in name_dates.items():
    for format in formats:
        for date in dates:
            if not in_format(date, format, name):
                break
        else:
            for date in dates:
                a, b, c = date
                if format == 'dmy' and a == 11 and b == 9 and c == 1:
                    ans_names.append(name)
                elif format == 'mdy' and a == 9 and b == 11 and c == 1:
                    ans_names.append(name)
                elif format == 'ymd' and a == 1 and b == 9 and c == 11:
                    ans_names.append(name)
                elif format == 'ydm' and a == 1 and b == 11 and c == 9:
                    ans_names.append(name)

print(' '.join(sorted(ans_names)))
