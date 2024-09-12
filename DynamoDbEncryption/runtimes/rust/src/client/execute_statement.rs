// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ExecuteStatement`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`consistent_read(impl Into<Option<::std::primitive::bool>>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::consistent_read) / [`set_consistent_read(Option<::std::primitive::bool>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::set_consistent_read): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`next_token(impl Into<Option<::std::string::String>>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::next_token) / [`set_next_token(Option<::std::string::String>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::set_next_token): (undocumented)<br>
    ///   - [`parameters(impl Into<Option<::std::vec::Vec<dynamodb::types::AttributeValue>>>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::parameters) / [`set_parameters(Option<::std::vec::Vec<dynamodb::types::AttributeValue>>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::set_parameters): (undocumented)<br>
    ///   - [`return_consumed_capacity(impl Into<Option<dynamodb::types::ReturnConsumedCapacity>>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::return_consumed_capacity) / [`set_return_consumed_capacity(Option<dynamodb::types::ReturnConsumedCapacity>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::set_return_consumed_capacity): (undocumented)<br>
    ///   - [`statement(impl Into<Option<::std::string::String>>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::statement) / [`set_statement(Option<::std::string::String>)`](crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::set_statement): (undocumented)<br>
    /// - On success, responds with [`ExecuteStatementOutput`](crate::operation::execute_statement::ExecuteStatementOutput) with field(s):
    ///   - [`consumed_capacity(Option<dynamodb::types::ConsumedCapacity>)`](crate::operation::execute_statement::ExecuteStatementOutput::consumed_capacity): (undocumented)
    ///   - [`items(Option<::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>)`](crate::operation::execute_statement::ExecuteStatementOutput::items): (undocumented)
    ///   - [`last_evaluated_key(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>)`](crate::operation::execute_statement::ExecuteStatementOutput::last_evaluated_key): (undocumented)
    ///   - [`next_token(Option<::std::string::String>)`](crate::operation::execute_statement::ExecuteStatementOutput::next_token): (undocumented)
    /// - On failure, responds with [`SdkError<ExecuteStatementError>`](crate::operation::execute_statement::ExecuteStatementError)
    pub fn execute_statement(&self) -> crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder {
        crate::operation::execute_statement::builders::ExecuteStatementFluentBuilder::new(self.clone())
    }
}
