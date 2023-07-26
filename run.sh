#!/bin/bash

if [ "$#" -ne 1 ]; then
	features+=","
fi 

for arg
do
	if [ "$#" == 1 ]; then
		features+=",$arg"
	elif [ $arg == ${@: -1} ]; then
		features+="$arg"
	else
		features+="$arg,"
	fi
done

cargo r --features dev$features --no-default-features
