#!/usr/bin/env bash

set -eu

out_info() {
    echo -e "\e[0;34mINFO\e[0m: $*"
}

distclean() {
    local -r ROOT_DIR=$1

    rm -v "$ROOT_DIR"/index.* || true
    rm -v "$ROOT_DIR/serial" || true
    rm -v "$ROOT_DIR/serial.old" || true
    rm -v "$ROOT_DIR"/newcerts/*.pem || true
    rm -v "$ROOT_DIR"/root.* || true
    rm -v "$ROOT_DIR"/server.* || true

    return 0
}

setup() {
    local -r ROOT_DIR=$1

    touch "$ROOT_DIR/index.txt"
    echo 00 > "$ROOT_DIR/serial"
}
cd $(dirname $0)/assets

out_info distclean
distclean "$PWD"
setup "$PWD"

out_info Generate CA private key.
openssl genrsa 2048 > root.key

out_info Generate CA certificate signing request.
openssl req -new -key root.key -subj "/O=root/CN=root" > root.csr

out_info Generate CA certification.
openssl ca \
    -config openssl.cnf \
    -name grpc_sample_ca \
    -keyfile root.key \
    -selfsign \
    -md sha256 \
    -days 36500 \
    -extensions grpc_sample_root_ext \
    -batch \
    -in root.csr \
    -out root.crt

out_info Generate server private key.
openssl genrsa 2048 > server.key

out_info Generate server certificate signing request.
openssl req -new -key server.key -subj "/O=localhost" > server.csr

out_info generate server certification.
openssl ca \
    -config openssl.cnf \
    -name grpc_sample_ca \
    -keyfile root.key \
    -cert root.crt \
    -md sha256 \
    -days 36500 \
    -extensions grpc_sample_server_ext \
    -batch \
    -in server.csr \
    -out server.crt
