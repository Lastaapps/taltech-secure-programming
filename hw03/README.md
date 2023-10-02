# RSA
For private key cracking don't forget to use the release variant:
```cargo r --release```

## Benchmark results
Used for the largest 32-bit integer (so n does not overflow in 64-bit int):

```
What do want to do?
(G)enerate a RSA key pair
(C)rack a RSA key
(E)ncrypt a text
(D)ecrypt a text
E(x)amples
(Q)uit the app
c
Enter modulo n:
4611686014132420609
Enter exponent (public key):
17
Let me think now...
Make sure you have compiled the program in the release variant
Found it, the prime numbers are:
2147483647
2147483647
Inverse (private key): 2441480828737475285
```

The whole process took about 25 seconds.

