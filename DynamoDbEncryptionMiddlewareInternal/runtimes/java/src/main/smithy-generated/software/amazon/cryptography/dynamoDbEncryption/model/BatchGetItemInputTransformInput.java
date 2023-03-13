// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.dynamoDbEncryption.model;

import java.util.Objects;
import software.amazon.awssdk.services.dynamodb.model.BatchGetItemRequest;

public class BatchGetItemInputTransformInput {
  private final BatchGetItemRequest sdkInput;

  protected BatchGetItemInputTransformInput(BuilderImpl builder) {
    this.sdkInput = builder.sdkInput();
  }

  public BatchGetItemRequest sdkInput() {
    return this.sdkInput;
  }

  public Builder toBuilder() {
    return new BuilderImpl(this);
  }

  public static Builder builder() {
    return new BuilderImpl();
  }

  public interface Builder {
    Builder sdkInput(BatchGetItemRequest sdkInput);

    BatchGetItemRequest sdkInput();

    BatchGetItemInputTransformInput build();
  }

  static class BuilderImpl implements Builder {
    protected BatchGetItemRequest sdkInput;

    protected BuilderImpl() {
    }

    protected BuilderImpl(BatchGetItemInputTransformInput model) {
      this.sdkInput = model.sdkInput();
    }

    public Builder sdkInput(BatchGetItemRequest sdkInput) {
      this.sdkInput = sdkInput;
      return this;
    }

    public BatchGetItemRequest sdkInput() {
      return this.sdkInput;
    }

    public BatchGetItemInputTransformInput build() {
      if (Objects.isNull(this.sdkInput()))  {
        throw new IllegalArgumentException("Missing value for required field `sdkInput`");
      }
      return new BatchGetItemInputTransformInput(this);
    }
  }
}
