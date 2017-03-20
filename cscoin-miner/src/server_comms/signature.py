import ssl
import sys

from Crypto.PublicKey import RSA
from Crypto.Hash import SHA256
from Crypto.Signature import PKCS1_v1_5

'''read key'''
fp          = open("private_key.pem", 'rb')
private_key = RSA.importKey(fp.read(4096))
fp.close()

fp = open("data.dat", 'rb')
data = fp.read(100000000)

'''Get args'''
hasher = SHA256.new()
hasher.update(data)
signer = PKCS1_v1_5.new(private_key)
sign_bytes = signer.sign(hasher)
signature = ""

for b in sign_bytes:
    signature += "{0:02x}".format(b)

print (signature)
