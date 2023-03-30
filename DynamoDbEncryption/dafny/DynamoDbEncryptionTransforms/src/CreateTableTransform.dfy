// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "DdbMiddlewareConfig.dfy"

module CreateTableTransform {
  import opened DdbMiddlewareConfig
  import opened DynamoDbMiddlewareSupport
  import opened Wrappers
  import DDB = ComAmazonawsDynamodbTypes
  import opened AwsCryptographyDynamoDbEncryptionTransformsTypes
  import EncTypes = AwsCryptographyDynamoDbEncryptionItemEncryptorTypes
  import Seq

  method Input(config: Config, input: CreateTableInputTransformInput)
    returns (output: Result<CreateTableInputTransformOutput, Error>)
    requires ValidConfig?(config)
    ensures ValidConfig?(config)
    modifies ModifiesConfig(config)
  {
    if input.sdkInput.TableName !in config.tableEncryptionConfigs {
      return Success(CreateTableInputTransformOutput(transformedInput := input.sdkInput));
    } else {
      var tableConfig := config.tableEncryptionConfigs[input.sdkInput.TableName];
      var finalResult :- CreateTableInputForBeacons(tableConfig, input.sdkInput);
      return Success(CreateTableInputTransformOutput(transformedInput := finalResult));
    }
  }

  method Output(config: Config, input: CreateTableOutputTransformInput)
    returns (output: Result<CreateTableOutputTransformOutput, Error>)
    ensures output.Success? && output.value.transformedOutput == input.sdkOutput
  {
    return Success(CreateTableOutputTransformOutput(transformedOutput := input.sdkOutput));
  }
}