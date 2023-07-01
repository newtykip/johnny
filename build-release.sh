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
if [ -z $features ]; then
	cargo build --release
else
	cargo build --release --features $features
fi

cp target/release/$(basename $PWD) $(basename $PWD)
