data = """00000013
00000073
"""

result = ""

for line in data.split("\n"):
    temp = ""

    for i, v in enumerate(line):
        temp += v
        if i % 2 == 1:
            temp += " "

    for v in [ v for v in reversed(temp.split(" ")) if v != '']:
        result += ("0x{},\n".format(v))


print(result)