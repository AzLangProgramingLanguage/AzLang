import time

def factorial(n):
    if n <= 1:
        return 1
    return n * factorial(n - 1)

def hesabla():
    cemi = 0
    for i in range(1, 5_000_001):
        cemi += factorial(15) + i
    return cemi

baslat = int(time.time() * 1000)  # vaxt_al()
netice = hesabla()
bitir = int(time.time() * 1000)

print(f"Cəmi: {netice}")
print(f"Vaxt: {bitir - baslat} ms")
