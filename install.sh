#!/usr/bin/env bash


if [[ -z $1 ]]; then
    installdir=~/.bin
else
    installdir=$1
fi

cargo build --release
[[ ! -d $installdir ]] && mkdir -p $installdir
cp target/release/bintools $installdir
[[ ! -d ~/.config ]] && mkdir -p ~/.config

[[ ! -f ~/.config/c.config.toml ]] &&
    cp c.config.toml ~/.config

cd $installdir
ln -sf bintools ag
ln -sf bintools rg
ln -sf bintools rgignore
ln -sf bintools fs
ln -sf bintools ff
ln -sf bintools vd
ln -sf bintools fspreview
ln -sf bintools c

echo "Now, add $installdir to your PATH if not:"
echo "export PATH=$installdir:\$PATH"
