// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`BatchWriteItemInputTransform`](crate::operation::batch_write_item_input_transform::builders::BatchWriteItemInputTransformFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`sdk_input(impl Into<Option<dynamodb::types::BatchWriteItemInput>>)`](crate::operation::batch_write_item_input_transform::builders::BatchWriteItemInputTransformFluentBuilder::sdk_input) / [`set_sdk_input(Option<dynamodb::types::BatchWriteItemInput>)`](crate::operation::batch_write_item_input_transform::builders::BatchWriteItemInputTransformFluentBuilder::set_sdk_input): (undocumented)<br>
    /// - On success, responds with [`BatchWriteItemInputTransformOutput`](crate::operation::batch_write_item_input_transform::BatchWriteItemInputTransformOutput) with field(s):
    ///   - [`transformed_input(Option<dynamodb::types::BatchWriteItemInput>)`](crate::operation::batch_write_item_input_transform::BatchWriteItemInputTransformOutput::transformed_input): (undocumented)
    /// - On failure, responds with [`SdkError<BatchWriteItemInputTransformError>`](crate::operation::batch_write_item_input_transform::BatchWriteItemInputTransformError)
    pub fn batch_write_item_input_transform(&self) -> crate::operation::batch_write_item_input_transform::builders::BatchWriteItemInputTransformFluentBuilder {
        crate::operation::batch_write_item_input_transform::builders::BatchWriteItemInputTransformFluentBuilder::new(self.clone())
    }
}
