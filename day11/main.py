import sys

data = open(sys.argv[1]).read().strip()
lines = data.split('\n')

odysseus = [
    "Οδυσσευς", "Odysseus",
    "Οδυσσεως", "Odysseos",
    "Οδυσσει", "Odyssei",
    "Οδυσσεα", "Odyssea",
    "Οδυσσευ", "Odysseu"
]
uppercases = "ΑΒΓΔΕΖΗΘΙΚΛΜΝΞΟΠΡΣΤΥΦΧΨΩ"
lowercases = "αβγδεζηθικλμνξοπρστυφχψω"

m = {}

for idx, uc in enumerate(uppercases):
    for shift in range(len(uppercases)):
        m[(uc, shift)] = uppercases[(idx + shift)%len(uppercases)]

for idx, lc in enumerate(lowercases):
    for shift in range(len(lowercases)):
        m[(lc, shift)] = lowercases[(idx + shift)%len(lowercases)]

def rotations(line):
    for i in range(26):
        for o in odysseus:
            if o in ''.join(m[(c, i)] if (c, i) in m else c for c in line):
                return i
    return 0

print(sum([rotations(line) for line in lines]))
