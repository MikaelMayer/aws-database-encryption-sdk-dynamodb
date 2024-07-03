#!/bin/sh

cp ImplementationFromDafny-rust/src/ImplementationFromDafny.rs \
  dafny_impl/src/ImplementationFromDafny-updated.rs

git merge --abort || git reset --merge ORIG_HEAD

mv dafny_impl/src/ImplementationFromDafny-updated.rs \
  dafny_impl/src/ImplementationFromDafny.rs

(cargo check && ./benerate3.sh) || (echo "Please run 'cargo check' and then manually run ./benerate3.sh when it does");