cmake_minimum_required(VERSION 3.6)
project(cscoin_miner)

set(CMAKE_CXX_STANDARD 11)

set(SOURCE_FILES
        src/main.rs
        src/miner.rs
        target/debug/.fingerprint/libc-690db683275b166f/dep-lib-libc
        target/debug/.fingerprint/libc-690db683275b166f/lib-libc
        target/debug/.fingerprint/libc-690db683275b166f/lib-libc.json
        target/debug/.fingerprint/rand-00ce216d81d5bd28/dep-lib-rand
        target/debug/.fingerprint/rand-00ce216d81d5bd28/lib-rand
        target/debug/.fingerprint/rand-00ce216d81d5bd28/lib-rand.json
        target/debug/deps/liblibc-29ef97a68464c2b7.rlib
        target/debug/deps/librand-8ea7d489d4a383a0.rlib
        target/debug/.cargo-lock
        Cargo.lock
        Cargo.toml)

add_executable(cscoin_miner ${SOURCE_FILES})