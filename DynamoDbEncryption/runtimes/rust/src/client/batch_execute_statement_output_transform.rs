// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`BatchExecuteStatementOutputTransform`](crate::operation::batch_execute_statement_output_transform::builders::BatchExecuteStatementOutputTransformFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`original_input(impl Into<Option<dynamodb::types::BatchExecuteStatementInput>>)`](crate::operation::batch_execute_statement_output_transform::builders::BatchExecuteStatementOutputTransformFluentBuilder::original_input) / [`set_original_input(Option<dynamodb::types::BatchExecuteStatementInput>)`](crate::operation::batch_execute_statement_output_transform::builders::BatchExecuteStatementOutputTransformFluentBuilder::set_original_input): (undocumented)<br>
    ///   - [`sdk_output(impl Into<Option<dynamodb::types::BatchExecuteStatementOutput>>)`](crate::operation::batch_execute_statement_output_transform::builders::BatchExecuteStatementOutputTransformFluentBuilder::sdk_output) / [`set_sdk_output(Option<dynamodb::types::BatchExecuteStatementOutput>)`](crate::operation::batch_execute_statement_output_transform::builders::BatchExecuteStatementOutputTransformFluentBuilder::set_sdk_output): (undocumented)<br>
    /// - On success, responds with [`BatchExecuteStatementOutputTransformOutput`](crate::operation::batch_execute_statement_output_transform::BatchExecuteStatementOutputTransformOutput) with field(s):
    ///   - [`transformed_output(Option<dynamodb::types::BatchExecuteStatementOutput>)`](crate::operation::batch_execute_statement_output_transform::BatchExecuteStatementOutputTransformOutput::transformed_output): (undocumented)
    /// - On failure, responds with [`SdkError<BatchExecuteStatementOutputTransformError>`](crate::operation::batch_execute_statement_output_transform::BatchExecuteStatementOutputTransformError)
    pub fn batch_execute_statement_output_transform(&self) -> crate::operation::batch_execute_statement_output_transform::builders::BatchExecuteStatementOutputTransformFluentBuilder {
        crate::operation::batch_execute_statement_output_transform::builders::BatchExecuteStatementOutputTransformFluentBuilder::new(self.clone())
    }
}
