#!/bin/sh
#Create a branch b1-from-impl off feat-rust-beneration 
git checkout -b b1-from-impl

# Copy the file 
cp -f dafny_impl/src/ImplementationFromDafny.rs \
  ImplementationFromDafny-rust/src/ImplementationFromDafny.rs
git commit -am "Changes still needed"

# Checkout feat-rust-beneration and create a branch b2-update-compiler
git checkout feat-rust-beneration
git checkout -b b2-update-compiler

# Run “make build” so that it updates
# ImplementationFromDafny-rust/src/ImplementationFromDafny.rs 
make build
git commit -am "New compiler changes"
git merge b1-from-impl

echo "Now resolve conflicts, and then run"
echo "./benerate2.sh"