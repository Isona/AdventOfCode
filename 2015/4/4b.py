import hashlib
secretKey = "yzbqklnj"
#secretKey = "pqrstuv"
i = 0

while True:
    m = hashlib.md5()
    m.update((secretKey+str(i)).encode('utf-8'))
    if m.hexdigest()[:6] == "000000":
        print(secretKey+str(i))
        print(m.hexdigest())
        exit()
    i += 1