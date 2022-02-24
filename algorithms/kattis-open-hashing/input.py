import random, string

n = 300000

s = ''.join(random.choice(string.ascii_lowercase) for _ in range(n))

def q():
    a = random.randint(0, n - 3)
    b = random.randint(a+2, n)
    return f"{a} {b}"

qs = [q() for _ in range(n)]

print(s)
print(n)
for q in qs:
    print(q)
