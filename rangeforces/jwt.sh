#!/bin/bash

# MESSAGE=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ
# SIGN=SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c

MESSAGE=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJKV1QgQ2hhbGxlbmdlIiwicm9sZSI6InVzZXIiLCJpYXQiOjE2OTc4MzgzMTl9
SIGN=s_FW9hh6wR0JuoMxnpRi2ZVrIY-Ki3xTHNn_dyktcaU

# for x in your-256-bit-secret
for x in {{a..z},{A..Z},{0..9}}{{a..z},{A..Z},{0..9}}{{a..z},{A..Z},{0..9}}{{a..z},{A..Z},{0..9}}
do
    IDK=$(echo -n "${MESSAGE}" | openssl dgst -sha256 -hmac "${x}" -binary | base64 -w 0 | tr '/' '_')
    IDK="${IDK:0:43}"
    echo $x
    # echo $IDK
    # echo $SIGN
    if [[ "$IDK" == "$SIGN" ]]; then
        echo "$x"
        exit 0
    fi
done

exit 1

