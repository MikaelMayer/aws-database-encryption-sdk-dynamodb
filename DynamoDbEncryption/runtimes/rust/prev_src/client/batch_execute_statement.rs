// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`BatchExecuteStatement`](crate::operation::batch_execute_statement::builders::BatchExecuteStatementFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::batch_execute_statement::builders::BatchExecuteStatementFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::batch_execute_statement::builders::BatchExecuteStatementFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    ///   - [`statements(impl Into<Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>>>)`](crate::operation::batch_execute_statement::builders::BatchExecuteStatementFluentBuilder::statements) / [`set_statements(Option<::std::vec::Vec<dynamodb::types::BatchStatementRequest>>)`](crate::operation::batch_execute_statement::builders::BatchExecuteStatementFluentBuilder::set_statements): (undocumented)<br>
    /// - On success, responds with [`BatchExecuteStatementOutput`](crate::operation::batch_execute_statement::BatchExecuteStatementOutput) with field(s):
    ///   - [`consumed_capacity(Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>)`](crate::operation::batch_execute_statement::BatchExecuteStatementOutput::consumed_capacity): (undocumented)
    ///   - [`responses(Option<::std::vec::Vec<dynamodb::types::BatchStatementResponse>>)`](crate::operation::batch_execute_statement::BatchExecuteStatementOutput::responses): (undocumented)
    /// - On failure, responds with [`SdkError<BatchExecuteStatementError>`](crate::operation::batch_execute_statement::BatchExecuteStatementError)
    pub fn batch_execute_statement(&self) -> crate::operation::batch_execute_statement::builders::BatchExecuteStatementFluentBuilder {
        crate::operation::batch_execute_statement::builders::BatchExecuteStatementFluentBuilder::new(self.clone())
    }
}
