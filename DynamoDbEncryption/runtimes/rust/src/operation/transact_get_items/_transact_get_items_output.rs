// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransactGetItemsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
#[allow(missing_docs)] // documentation missing in model
pub responses: ::std::option::Option<::std::vec::Vec<dynamodb::types::ItemResponse>>,
}
impl TransactGetItemsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn responses(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ItemResponse>> {
    &self.responses
}
}
impl TransactGetItemsOutput {
    /// Creates a new builder-style object to manufacture [`TransactGetItemsOutput`](crate::operation::transact_get_items::builders::TransactGetItemsOutput).
    pub fn builder() -> crate::operation::transact_get_items::builders::TransactGetItemsOutputBuilder {
        crate::operation::transact_get_items::builders::TransactGetItemsOutputBuilder::default()
    }
}

/// A builder for [`TransactGetItemsOutput`](crate::operation::operation::TransactGetItemsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TransactGetItemsOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
pub(crate) responses: ::std::option::Option<::std::vec::Vec<dynamodb::types::ItemResponse>>,
}
impl TransactGetItemsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>) -> Self {
    self.consumed_capacity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_consumed_capacity(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>) -> Self {
    self.consumed_capacity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn responses(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::ItemResponse>>) -> Self {
    self.responses = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_responses(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::ItemResponse>>) -> Self {
    self.responses = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_responses(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ItemResponse>> {
    &self.responses
}
    /// Consumes the builder and constructs a [`TransactGetItemsOutput`](crate::operation::operation::TransactGetItemsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::transact_get_items::TransactGetItemsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::transact_get_items::TransactGetItemsOutput {
            consumed_capacity: self.consumed_capacity,
responses: self.responses,
        })
    }
}
