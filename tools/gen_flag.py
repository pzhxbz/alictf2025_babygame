import random
from pwn import u32, p8
from ctypes import c_uint32

# flag_len = 64
# flags = []
# for i in range(flag_len):
#     flags.append(random.randint(0, 99))

# print(flags)

flags = [
    93,
    34,
    19,
    34,
    55,
    54,
    77,
    77,
    69,
    96,
    42,
    94,
    71,
    91,
    38,
    1,
    50,
    54,
    60,
    81,
    62,
    31,
    27,
    68,
    9,
    96,
    29,
    72,
    75,
    90,
    14,
    9,
    91,
    22,
    93,
    95,
    67,
    16,
    82,
    21,
    79,
    42,
    96,
    85,
    0,
    95,
    52,
    11,
    44,
    85,
    79,
    86,
    21,
    52,
    51,
    31,
    54,
    20,
    26,
    11,
    97,
    49,
    23,
    65,
]


def next_random(i):
    j = 0
    j = i * 1664525 + 1013904223
    j ^= j >> 16
    j = j * 1664525 + 1013904223
    j ^= j >> 16

    return j & 0xFFFFFFFF  # 返回u32范围内的值，即取低32位


def calc_res(input, i):
    xor1 = i & 0xFF
    xor2 = (i >> 8) & 0xFF
    xor3 = (i >> 16) & 0xFF
    xor4 = (i >> 24) & 0xFF

    input ^= xor1
    input += xor2
    input ^= xor3
    input += xor4

    return input & 0xFF  # 返回u8范围内的值，即取低8位


def de_calc_res(input, i):
    xor1 = i & 0xFF
    xor2 = (i >> 8) & 0xFF
    xor3 = (i >> 16) & 0xFF
    xor4 = (i >> 24) & 0xFF

    input -= xor4
    input ^= xor3
    input -= xor2
    input ^= xor1

    return input & 0xFF  # 返回u8范围内的值，即取低8位


def enc1(flags):
    status = 0
    res = []
    for i in range(len(flags)):
        status = next_random(status)
        # print(status)
        res.append(calc_res(flags[i], status))
    return res


def dec1(flags):
    status = 0
    res = []
    for i in range(len(flags)):
        status = next_random(status)
        res.append(de_calc_res(flags[i], status))
    return res


en1 = enc1(flags)
print("enc1", en1)
assert dec1(en1) == flags

en1 = b"".join(p8(i) for i in en1)

en1_b = []
for i in range(16):
    en1_b.append(u32(en1[i * 4 : i * 4 + 4]))

print("en1_b", en1_b)


def MX(z, y, total, key, p, e):
    temp1 = (z.value >> 5 ^ y.value << 2) + (y.value >> 3 ^ z.value << 4)
    temp2 = (total.value ^ y.value) + (key[(p & 3) ^ e.value] ^ z.value)

    return c_uint32(temp1 ^ temp2)


def encrypt(n, v, key, delta=0x9E3779B9):
    delta = delta
    rounds = 6 + 52 // n

    total = c_uint32(0)
    z = c_uint32(v[n - 1])
    e = c_uint32(0)

    while rounds > 0:
        total.value += delta
        e.value = (total.value >> 2) & 3
        for p in range(n - 1):
            y = c_uint32(v[p + 1])
            v[p] = c_uint32(v[p] + MX(z, y, total, key, p, e).value).value
            z.value = v[p]
        y = c_uint32(v[0])
        v[n - 1] = c_uint32(v[n - 1] + MX(z, y, total, key, n - 1, e).value).value
        z.value = v[n - 1]
        rounds -= 1

    return v


def decrypt(n, v, key, delta=0x9E3779B9):
    delta = delta
    rounds = 6 + 52 // n

    total = c_uint32(rounds * delta)
    y = c_uint32(v[0])
    e = c_uint32(0)

    while rounds > 0:
        e.value = (total.value >> 2) & 3
        for p in range(n - 1, 0, -1):
            z = c_uint32(v[p - 1])
            v[p] = c_uint32((v[p] - MX(z, y, total, key, p, e).value)).value
            y.value = v[p]
        z = c_uint32(v[n - 1])
        v[0] = c_uint32(v[0] - MX(z, y, total, key, 0, e).value).value
        y.value = v[0]
        total.value -= delta
        rounds -= 1

    return v


def encrypt_xor(n, v, key, delta=0x98D846DC):
    delta = delta

    for i in range(len(v)):
        v[i] ^= 0x42E2B468

    rounds = 6 + 52 // n

    total = c_uint32(0)
    z = c_uint32(v[n - 1])
    e = c_uint32(0)

    while rounds > 0:
        total.value += delta
        e.value = (total.value >> 2) & 3
        for p in range(n - 1):
            y = c_uint32(v[p + 1])
            v[p] = c_uint32(v[p] + MX(z, y, total, key, p, e).value).value
            z.value = v[p]
        y = c_uint32(v[0])
        v[n - 1] = c_uint32(v[n - 1] + MX(z, y, total, key, n - 1, e).value).value
        z.value = v[n - 1]
        rounds -= 1
    for i in range(len(v)):
        v[i] ^= 0x71F28B88

    return v


def decrypt_xor(n, v, key, delta=0x98D846DC):
    delta = delta

    for i in range(len(v)):
        v[i] ^= 0x71F28B88

    rounds = 6 + 52 // n

    total = c_uint32(0)
    z = c_uint32(v[n - 1])
    e = c_uint32(0)

    while rounds > 0:
        total.value += delta
        e.value = (total.value >> 2) & 3
        for p in range(n - 1):
            y = c_uint32(v[p + 1])
            v[p] = c_uint32(v[p] + MX(z, y, total, key, p, e).value).value
            z.value = v[p]
        y = c_uint32(v[0])
        v[n - 1] = c_uint32(v[n - 1] + MX(z, y, total, key, n - 1, e).value).value
        z.value = v[n - 1]
        rounds -= 1

    for i in range(len(v)):
        v[i] ^= 0x42E2B468
    return v


d1 = 0x6BC6121D
k1 = [0xAF657662, 0xFC6F144B, 0x22AB2B6C, 0x367D2DCB]

en2 = encrypt(len(en1_b), en1_b, k1, d1)
# assert decrypt(len(en2) , en2, k1, d1) == en1_b
print("en2", en2)

d2 = 0xB72908F9
k2 = [0x9E51E580, 0xF4496000, 0x64168EED, 0x496E55BF]
en3 = encrypt(len(en2), en2, k2, d2)
# assert decrypt(len(en3) , en3, k2, d2) == en2
print("en3", en3)


k3 = [0x41661F49, 0xDFC12FCF, 0x1FE0F1A2, 0x71168786]
k4 = encrypt_xor(len(en3), en3, k3)
print(k4)
assert decrypt_xor(len(k4), k4, k3) == en3
