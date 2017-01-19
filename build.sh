#!/bin/sh

RUSTC=`which rustc`

if [ "${RUSTC}" = "" ];then
    echo "Build failed. Could not find rustc."
    exit
fi

rustc introduce.rs
