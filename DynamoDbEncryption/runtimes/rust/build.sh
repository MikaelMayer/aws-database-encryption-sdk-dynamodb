#!/bin/sh

# If DAFNY is not defined, use ~/Documents/dafny/Binaries/Dafny.exe

if [ -z "$DAFNY" ]; then
  DAFNY=~/Documents/dafny/Binaries/Dafny.exe
fi
MPL=runtimes/rust/dafny_impl/src
IMPL=runtimes/rust/dafny_impl/src
ALL_EXTERNS=$(ls dafny_impl/src | grep .rs | grep -v ImplementationFromDafny.rs | xargs -I{} echo $IMPL/{})

# List all the .dfy files in ../../DynamoDbEncryption/dafny/DynamoDbEncryptionTransforms/test suitable for passing to the command line below (NO NEWLINES, ONLY SEPARATED BY SPACES)
TEST_FILES_DYNAMODBENCRYPTION=$(ls ../../dafny/DynamoDbEncryption/test | grep .dfy | xargs -I{} echo dafny/DynamoDbEncryption/test/{})
# Same but for DynamoDbEncryptionTransforms
TEST_FILES_DYNAMODBENCRYPTIONTRANSFORMS=$(ls ../../dafny/DynamoDbEncryptionTransforms/test | grep .dfy | xargs -I{} echo dafny/DynamoDbEncryptionTransforms/test/{})
# Same but for DynamoDbItemEncryptor
TEST_FILES_DYNAMODBITEMENCRYPTOR=$(ls ../../dafny/DynamoDbItemEncryptor/test | grep .dfy | xargs -I{} echo dafny/DynamoDbItemEncryptor/test/{})
# Same but for StructuredEncryption
TEST_FILES_STRUCTUREDENCRYPTION=$(ls ../../dafny/StructuredEncryption/test | grep .dfy | xargs -I{} echo dafny/StructuredEncryption/test/{})

(cd ../..;
$DAFNY  translate rs --no-verify --emit-uncompilable-code:true \
--allow-warnings --optimize-erasable-datatype-wrapper:false --allow-external-contracts \
--unicode-char:false --function-syntax:3 \
--include-runtime:true --output runtimes/rust/ImplementationFromDafny UniqueToBuildInRust.dfy \
 $TEST_FILES_DYNAMODBENCRYPTION \
 $TEST_FILES_DYNAMODBENCRYPTIONTRANSFORMS \
 $TEST_FILES_DYNAMODBITEMENCRYPTOR \
 $TEST_FILES_STRUCTUREDENCRYPTION \
 $ALL_EXTERNS
)

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