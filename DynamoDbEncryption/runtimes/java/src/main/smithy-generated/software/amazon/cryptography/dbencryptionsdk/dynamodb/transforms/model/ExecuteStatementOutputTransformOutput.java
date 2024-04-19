// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.dbencryptionsdk.dynamodb.transforms.model;

import java.util.Objects;
import software.amazon.awssdk.services.dynamodb.model.ExecuteStatementResponse;

public class ExecuteStatementOutputTransformOutput {

  private final ExecuteStatementResponse transformedOutput;

  protected ExecuteStatementOutputTransformOutput(BuilderImpl builder) {
    this.transformedOutput = builder.transformedOutput();
  }

  public ExecuteStatementResponse transformedOutput() {
    return this.transformedOutput;
  }

  public Builder toBuilder() {
    return new BuilderImpl(this);
  }

  public static Builder builder() {
    return new BuilderImpl();
  }

  public interface Builder {
    Builder transformedOutput(ExecuteStatementResponse transformedOutput);

    ExecuteStatementResponse transformedOutput();

    ExecuteStatementOutputTransformOutput build();
  }

  static class BuilderImpl implements Builder {

    protected ExecuteStatementResponse transformedOutput;

    protected BuilderImpl() {}

    protected BuilderImpl(ExecuteStatementOutputTransformOutput model) {
      this.transformedOutput = model.transformedOutput();
    }

    public Builder transformedOutput(
      ExecuteStatementResponse transformedOutput
    ) {
      this.transformedOutput = transformedOutput;
      return this;
    }

    public ExecuteStatementResponse transformedOutput() {
      return this.transformedOutput;
    }

    public ExecuteStatementOutputTransformOutput build() {
      if (Objects.isNull(this.transformedOutput())) {
        throw new IllegalArgumentException(
          "Missing value for required field `transformedOutput`"
        );
      }
      return new ExecuteStatementOutputTransformOutput(this);
    }
  }
}
