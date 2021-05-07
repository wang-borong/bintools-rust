#!/usr/bin/env bash


# install to $1
if [[ $1 == "" ]]; then
    echo "$0 <install dir>"
fi

cargo build --release
[[ ! -d $1 ]] && mkdir -p $1
cp target/release/bintools $1
[[ ! -d ~/.config ]] && mkdir -p ~/.config

[[ ! -f ~/.config/c.config.toml ]] &&
    cp c.config.toml ~/.config

cd $1
ln -sf bintools ag
ln -sf bintools rg
ln -sf bintools fs
ln -sf bintools ff
ln -sf bintools vd
ln -sf bintools fspreview
ln -sf bintools c

echo "Now, add $1 to your PATH if not:"
echo "export PATH=$1:\$PATH"
