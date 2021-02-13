#!/usr/bin/env bash


# install to $1

cargo build --release
[ ! -d $1 ] && mkdir -p $1
cp target/release/bintools $1
cd $1
ln -sf bintools ag
ln -sf bintools rg
ln -sf bintools fs
ln -sf bintools ff
ln -sf bintools vd
ln -sf bintools fspreview
ln -sf bintools c
