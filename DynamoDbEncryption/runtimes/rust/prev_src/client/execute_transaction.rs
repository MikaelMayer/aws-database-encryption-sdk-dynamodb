// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ExecuteTransaction`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_request_token(impl Into<Option<::std::string::String>>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::client_request_token) / [`set_client_request_token(Option<::std::string::String>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::set_client_request_token): (undocumented)<br>
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    ///   - [`transact_statements(impl Into<Option<::std::vec::Vec<dynamodb::types::ParameterizedStatement>>>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::transact_statements) / [`set_transact_statements(Option<::std::vec::Vec<dynamodb::types::ParameterizedStatement>>)`](crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::set_transact_statements): (undocumented)<br>
    /// - On success, responds with [`ExecuteTransactionOutput`](crate::operation::execute_transaction::ExecuteTransactionOutput) with field(s):
    ///   - [`consumed_capacity(Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>)`](crate::operation::execute_transaction::ExecuteTransactionOutput::consumed_capacity): (undocumented)
    ///   - [`responses(Option<::std::vec::Vec<dynamodb::types::ItemResponse>>)`](crate::operation::execute_transaction::ExecuteTransactionOutput::responses): (undocumented)
    /// - On failure, responds with [`SdkError<ExecuteTransactionError>`](crate::operation::execute_transaction::ExecuteTransactionError)
    pub fn execute_transaction(&self) -> crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder {
        crate::operation::execute_transaction::builders::ExecuteTransactionFluentBuilder::new(self.clone())
    }
}
