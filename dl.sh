#!/bin/bash

prefix=https://raw.githubusercontent.com/rust-lang/rustlings/main/exercises/

if [[ $# -eq 0 ]]; then
    echo "Usage: $0 enums/enums{1..3}.rs"
    exit
fi

curl --remote-name-all "${@/#/$prefix}"
