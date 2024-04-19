#!/usr/bin/env bash

set -ex

rm -rfv src
mkdir -p src
pushd src

cargo run --manifest-path ../zbus/zbus_xmlgen/Cargo.toml --release -- \
    ../gvfs/common/org.gtk.vfs.xml

for file in *.rs
do
    name="$(basename $file .rs)"
    echo "pub mod ${name};" >> lib.rs
done
echo "pub use zbus;" >> lib.rs

popd
