#!/bin/sh

git commit -am "Fixed beneration"
# Refer to the diff with the original file if needed.

# Commit and squash all your commits into one
git reset --soft HEAD~2
git commit -am "Updated beneration with latest feat-rust compiler"

git checkout feat-rust-beneration && git merge b2-update-compiler

## Cleanup
git branch -D b2-update-compiler
git branch -D b1-from-impl

# Compute the new number of diffs
make diff

git push
