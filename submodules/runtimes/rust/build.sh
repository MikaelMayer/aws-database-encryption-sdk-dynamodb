#!/bin/sh

echo "Start";

if [ -z "$DAFNY" ]; then
  DAFNY=~/Documents/dafny/Binaries/Dafny.exe
fi
#CURRENT_ABSOLUTE_PATH=`cd ../..; pwd`
MPL=C:/Users/mimayere/Documents/aws-database-encryption-sdk-dynamodb-java/submodules/MaterialProviders/

IMPL=runtimes/rust/dafny_impl/src
# List all the files in the folder $IMPL with extension .rs except ImplementationFromDafny.rs
# Then prefix all of them with $IMPL and join them with a space
ALL_EXTERNS=$(ls dafny_impl/src | grep .rs | grep -v ImplementationFromDafny.rs | xargs -I{} echo $IMPL/{})

(cd ../../../submodules;
$DAFNY translate rs --no-verify --emit-uncompilable-code:true \
--allow-warnings --optimize-erasable-datatype-wrapper:false --allow-external-contracts \
--quantifier-syntax:3 --unicode-char:false --function-syntax:3 \
--include-runtime:true --output runtimes/rust/ImplementationFromDafny \
$MPL/StandardLibrary/src/Index.dfy \
$MPL/AwsCryptographyPrimitives/src/Index.dfy \
$MPL/ComAmazonawsKms/src/Index.dfy \
$MPL/ComAmazonawsDynamodb/src/Index.dfy \
$MPL/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders/src/Index.dfy \
$MPL/AwsCryptographicMaterialProviders/dafny/AwsCryptographyKeyStore/src/Index.dfy \
MaterialProvidersIndex.dfy $ALL_EXTERNS)

echo "Removing externs";
REMOVE_EXTERNS=$(ls ImplementationFromDafny-rust/src | grep .rs | grep -v ImplementationFromDafny.rs | xargs -I{} echo ImplementationFromDafny-rust/src/{})
# Now remove all *.rs files 
rm $REMOVE_EXTERNS
