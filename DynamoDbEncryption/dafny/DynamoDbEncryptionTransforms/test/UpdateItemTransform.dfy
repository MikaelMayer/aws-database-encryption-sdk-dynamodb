// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
include "../../DynamoDbEncryptionTransforms/src/Index.dfy"
include "../../DynamoDbEncryption/test/TestFixtures.dfy"

module UpdateItemTransformTest {
  import opened Wrappers
  import opened DynamoDbEncryptionTransforms
  import opened TestFixtures
  import DDB = ComAmazonawsDynamodbTypes
  import AwsCryptographyDynamoDbEncryptionTransformsTypes

  method {:test} TestUpdateItemInputPassthrough() {
    var middlewareUnderTest := TestFixtures.GetDynamoDbEncryptionTransforms();
    var input := DDB.UpdateItemInput(
      TableName := "no_such_table",
      Key := map[],
      AttributeUpdates := None(),
      Expected := None(),
      ConditionalOperator := None(),
      ReturnValues := None(),
      ReturnConsumedCapacity := None(),
      ReturnItemCollectionMetrics := None(),
      UpdateExpression := None(),
      ConditionExpression := None(),
      ExpressionAttributeNames := None(),
      ExpressionAttributeValues := None()
    );
    var transformed := middlewareUnderTest.UpdateItemInputTransform(
      AwsCryptographyDynamoDbEncryptionTransformsTypes.UpdateItemInputTransformInput(
        sdkInput := input
      )
    );

    expect_ok("UpdateItemInput", transformed);
    expect_equal("UpdateItemInput", transformed.value.transformedInput, input);
  }

    method {:test} TestUpdateItemInputUpdateExpressionSigned() {
    var middlewareUnderTest := TestFixtures.GetDynamoDbEncryptionTransforms();
    var input := DDB.UpdateItemInput(
      TableName := "foo",
      Key := map[],
      AttributeUpdates := None(),
      Expected := None(),
      ConditionalOperator := None(),
      ReturnValues := None(),
      ReturnConsumedCapacity := None(),
      ReturnItemCollectionMetrics := None(),
      UpdateExpression := Some("SET sign = :p"),
      ConditionExpression := None(),
      ExpressionAttributeNames := None(),
      ExpressionAttributeValues := None()
    );
    var transformed := middlewareUnderTest.UpdateItemInputTransform(
      AwsCryptographyDynamoDbEncryptionTransformsTypes.UpdateItemInputTransformInput(
        sdkInput := input
      )
    );

    ExpectFailure(transformed, "Update Expressions forbidden on signed attributes : sign");
  }


    method {:test} TestUpdateItemInputUpdateExpressionEncrypted() {
    var middlewareUnderTest := TestFixtures.GetDynamoDbEncryptionTransforms();
    var input := DDB.UpdateItemInput(
      TableName := "foo",
      Key := map[],
      AttributeUpdates := None(),
      Expected := None(),
      ConditionalOperator := None(),
      ReturnValues := None(),
      ReturnConsumedCapacity := None(),
      ReturnItemCollectionMetrics := None(),
      UpdateExpression := Some("SET encrypt = :p"),
      ConditionExpression := None(),
      ExpressionAttributeNames := None(),
      ExpressionAttributeValues := None()
    );
    var transformed := middlewareUnderTest.UpdateItemInputTransform(
      AwsCryptographyDynamoDbEncryptionTransformsTypes.UpdateItemInputTransformInput(
        sdkInput := input
      )
    );

    ExpectFailure(transformed, "Update Expressions forbidden on signed attributes : encrypt");
  }


    method {:test} TestUpdateItemInputUpdateExpressionPlain() {
    var middlewareUnderTest := TestFixtures.GetDynamoDbEncryptionTransforms();
    var input := DDB.UpdateItemInput(
      TableName := "foo",
      Key := map[],
      AttributeUpdates := None(),
      Expected := None(),
      ConditionalOperator := None(),
      ReturnValues := None(),
      ReturnConsumedCapacity := None(),
      ReturnItemCollectionMetrics := None(),
      UpdateExpression := Some("SET plain = :p"),
      ConditionExpression := None(),
      ExpressionAttributeNames := None(),
      ExpressionAttributeValues := None()
    );
    var transformed := middlewareUnderTest.UpdateItemInputTransform(
      AwsCryptographyDynamoDbEncryptionTransformsTypes.UpdateItemInputTransformInput(
        sdkInput := input
      )
    );

    expect_ok("UpdateItemInput", transformed);
    expect_equal("UpdateItemInput", transformed.value.transformedInput, input);
  }

  method {:test} TestUpdateItemOutputPassthrough() {
    var middlewareUnderTest := TestFixtures.GetDynamoDbEncryptionTransforms();
    var output := DDB.UpdateItemOutput(
      Attributes := None(),
      ConsumedCapacity := None(),
      ItemCollectionMetrics := None()
    );
    var input := DDB.UpdateItemInput(
      TableName := "no_such_table",
      Key := map[],
      AttributeUpdates := None(),
      Expected := None(),
      ConditionalOperator := None(),
      ReturnValues := None(),
      ReturnConsumedCapacity := None(),
      ReturnItemCollectionMetrics := None(),
      UpdateExpression := None(),
      ConditionExpression := None(),
      ExpressionAttributeNames := None(),
      ExpressionAttributeValues := None()
    );
    var transformed := middlewareUnderTest.UpdateItemOutputTransform(
      AwsCryptographyDynamoDbEncryptionTransformsTypes.UpdateItemOutputTransformInput(
        sdkOutput := output,
        originalInput := input
      )
    );

    expect_ok("UpdateItemOutput", transformed);
    expect_equal("UpdateItemOutput", transformed.value.transformedOutput, output);
  }
}