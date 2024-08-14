#!/bin/sh

# If DAFNY is not defined, use ~/Documents/dafny/Binaries/Dafny.exe

if [ -z "$DAFNY" ]; then
  DAFNY=~/Documents/dafny/Binaries/Dafny.exe
fi
MPL=runtimes/rust/dafny_impl/src
IMPL=runtimes/rust/dafny_impl/src
ALL_EXTERNS=$(ls dafny_impl/src | grep .rs | grep -v ImplementationFromDafny.rs | xargs -I{} echo $IMPL/{})

(cd ../../../DynamoDbEncryption;
$DAFNY  translate rs --no-verify --emit-uncompilable-code:true \
--allow-warnings --optimize-erasable-datatype-wrapper:false --allow-external-contracts \
--unicode-char:false --function-syntax:3 \
--include-runtime:true --output runtimes/rust/ImplementationFromDafny UniqueToBuildInRust.dfy $ALL_EXTERNS)

echo "Removing externs";
REMOVE_EXTERNS=$(ls ImplementationFromDafny-rust/src | grep .rs | grep -v ImplementationFromDafny.rs | xargs -I{} echo ImplementationFromDafny-rust/src/{})
# Now remove all *.rs files 
rm $REMOVE_EXTERNS

echo "Moving generated file";
mv ImplementationFromDafny-rust/src/ImplementationFromDafny.rs dafny_impl/src/ImplementationFromDafny.rs
mv ImplementationFromDafny-rust/src/ImplementationFromDafny-rs.dtr dafny_impl/src/ImplementationFromDafny-rs.dtr
rm -r ImplementationFromDafny-rust

echo "Formatting generated file"
(cd dafny_impl; cargo fmt)