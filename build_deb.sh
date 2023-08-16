#!/usr/bin/bash

cargo build --release

version=`cat Cargo.toml | grep "version = " | head -n 1 | cut -d\" -f2 -`
description=`cat Cargo.toml | grep "description = " | head -n 1 | cut -d\" -f2 -`
homepage=`cat Cargo.toml | grep "repository = " | head -n 1 | cut -d\" -f2 -`
installed_size=`du -ks target/release/onetime-cli |cut -f 1`

build_dir_name="onetime-cli-$version"
base_dir="target/release/bundle/$build_dir_name"
bin_dir="$base_dir/usr/bin"

mkdir -p $bin_dir

cp target/release/onetime-cli $bin_dir/.


mkdir -p $base_dir/DEBIAN
cp deb-src/control $base_dir/DEBIAN/control

sed -i "s|VERSION|${version}|" $base_dir/DEBIAN/control
sed -i "s|DESCRIPTION|${description}|" $base_dir/DEBIAN/control
sed -i "s|INSTALLED_SIZE|${installed_size}|" $base_dir/DEBIAN/control
sed -i "s|HOMEPAGE|${homepage}|" $base_dir/DEBIAN/control

cd $base_dir/..
dpkg-deb --build --root-owner-group $build_dir_name