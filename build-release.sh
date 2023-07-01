#!/bin/bash

# compile feature list
for arg
do
	if [ $arg == ${@: -1} ]; then
		features+="$arg"
		break
	fi
	features+="$arg,"
done

# build step
cargo build --release --features $features
cp target/release/$(basename $1) $1
