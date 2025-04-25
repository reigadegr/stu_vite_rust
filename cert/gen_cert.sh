#!/bin/sh
rm *.pem *.csr 2>/dev/null
#1. **生成私钥 (`key.pem`)**:
openssl genrsa -out key.pem 2048

#2. **生成证书签名请求 (CSR)**:
openssl req -new -key key.pem -out csr.csr -subj "/C=CN/ST=Beijing/O=school/CN=reigadegr/emailAddress=123456@qq.com" -batch

#3. **自签名证书 (`cert.pem`)**:
openssl req -x509 -days 36500 -key key.pem -in csr.csr -out cert.pem

