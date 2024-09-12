// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransactGetItemsInput {
    #[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
#[allow(missing_docs)] // documentation missing in model
pub transact_items: ::std::option::Option<::std::vec::Vec<dynamodb::types::TransactGetItem>>,
}
impl TransactGetItemsInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn transact_items(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::TransactGetItem>> {
    &self.transact_items
}
}
impl TransactGetItemsInput {
    /// Creates a new builder-style object to manufacture [`TransactGetItemsInput`](crate::operation::transact_get_items::builders::TransactGetItemsInput).
    pub fn builder() -> crate::operation::transact_get_items::builders::TransactGetItemsInputBuilder {
        crate::operation::transact_get_items::builders::TransactGetItemsInputBuilder::default()
    }
}

/// A builder for [`TransactGetItemsInput`](crate::operation::operation::TransactGetItemsInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TransactGetItemsInputBuilder {
    pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
pub(crate) transact_items: ::std::option::Option<::std::vec::Vec<dynamodb::types::TransactGetItem>>,
}
impl TransactGetItemsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(mut self, input: impl ::std::convert::Into<dynamodb::types::ReturnConsumedCapacity>) -> Self {
    self.return_consumed_capacity = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_return_consumed_capacity(mut self, input: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>) -> Self {
    self.return_consumed_capacity = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn transact_items(mut self, input: impl ::std::convert::Into<::std::vec::Vec<dynamodb::types::TransactGetItem>>) -> Self {
    self.transact_items = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_transact_items(mut self, input: ::std::option::Option<::std::vec::Vec<dynamodb::types::TransactGetItem>>) -> Self {
    self.transact_items = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_transact_items(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::TransactGetItem>> {
    &self.transact_items
}
    /// Consumes the builder and constructs a [`TransactGetItemsInput`](crate::operation::operation::TransactGetItemsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::transact_get_items::TransactGetItemsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::transact_get_items::TransactGetItemsInput {
            return_consumed_capacity: self.return_consumed_capacity,
transact_items: self.transact_items,
        })
    }
}
