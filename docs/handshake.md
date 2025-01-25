# handshake
The protocol would begin as follows:

1. Both robots exchange their Globally Unique Wallet Address.
2. Both robots issue an encrypted challenge using the other robot’s public key. They generate a random challenge string, and encrypt it with the other robot’s public key.
3. Both robots decrypt the challenge with their private key, and send back the randomly generated challenge string.
4. Both robots ensure that the string they received matches their generated challenge.
