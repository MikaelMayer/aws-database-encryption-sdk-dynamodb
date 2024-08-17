#!/bin/bash

cd /Users/ajewell/test_mpl_rust/aws-database-encryption-sdk-dynamodb/submodules/MaterialProviders

git checkout ComAmazonawsKms/Model/ComAmazonawsKmsTypes.dfy
head -4 ComAmazonawsKms/Model/ComAmazonawsKmsTypes.dfy > new_file
tail +5  /Users/ajewell/gzg/smithy-dafny/TestModels/aws-sdks/kms-lite/model/ComAmazonawsKmsTypes.dfy >> new_file
mv new_file ComAmazonawsKms/Model/ComAmazonawsKmsTypes.dfy

git checkout ComAmazonawsDynamodb/Model/ComAmazonawsDynamodbTypes.dfy
head -4 ComAmazonawsDynamodb/Model/ComAmazonawsDynamodbTypes.dfy > new_file
tail +5  /Users/ajewell/gzg/smithy-dafny/TestModels/aws-sdks/ddb-lite/model/ComAmazonawsDynamodbTypes.dfy >> new_file
mv new_file ComAmazonawsDynamodb/Model/ComAmazonawsDynamodbTypes.dfy
