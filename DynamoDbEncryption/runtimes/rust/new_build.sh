#!/bin/sh -eu


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

# TEST_FILES_TESTVECTORS=../TestVectors/dafny/DDBEncryption/test/RunMain.dfy
# TEST_FILES_TESTVECTORS=../submodules/MaterialProviders/TestVectorsAwsCryptographicMaterialProviders/dafny/TestVectorsAwsCryptographicMaterialProviders/src/Index.dfy ../submodules/MaterialProviders/TestVectorsAwsCryptographicMaterialProviders/dafny/TestVectorsAwsCryptographicMaterialProviders/test/RunMain.dfy 
TEST_FILES_TESTVECTORS=../submodules/MaterialProviders/TestVectorsAwsCryptographicMaterialProviders/dafny/TestVectorsAwsCryptographicMaterialProviders/test/RunMain.dfy 

(cd ../..;
dafny translate rs --no-verify --emit-uncompilable-code:true \
--allow-warnings --optimize-erasable-datatype-wrapper:false --allow-external-contracts \
--unicode-char:false --function-syntax:3 \
--include-runtime:true --output runtimes/rust/ImplementationFromDafny UniqueToBuildInRust.dfy \
 $TEST_FILES_TESTVECTORS \
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

#echo "Formatting generated file"
#sed -i -e 's/[[:space:]]*$$//' dafny_impl/src/ImplementationFromDafny.rs
#(cd dafny_impl; cargo fmt)