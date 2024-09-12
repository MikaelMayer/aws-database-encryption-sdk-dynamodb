// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransactWriteItemsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
#[allow(missing_docs)] // documentation missing in model
pub item_collection_metrics: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>,
}
impl TransactWriteItemsOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn item_collection_metrics(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>> {
    &self.item_collection_metrics
}
}
impl TransactWriteItemsOutput {
    /// Creates a new builder-style object to manufacture [`TransactWriteItemsOutput`](crate::operation::transact_write_items::builders::TransactWriteItemsOutput).
    pub fn builder() -> crate::operation::transact_write_items::builders::TransactWriteItemsOutputBuilder {
        crate::operation::transact_write_items::builders::TransactWriteItemsOutputBuilder::default()
    }
}

/// A builder for [`TransactWriteItemsOutput`](crate::operation::operation::TransactWriteItemsOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct TransactWriteItemsOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
pub(crate) item_collection_metrics: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>,
}
impl TransactWriteItemsOutputBuilder {
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
pub fn item_collection_metrics(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>) -> Self {
    self.item_collection_metrics = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_item_collection_metrics(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>>) -> Self {
    self.item_collection_metrics = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_item_collection_metrics(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<dynamodb::types::ItemCollectionMetrics>>> {
    &self.item_collection_metrics
}
    /// Consumes the builder and constructs a [`TransactWriteItemsOutput`](crate::operation::operation::TransactWriteItemsOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::transact_write_items::TransactWriteItemsOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::transact_write_items::TransactWriteItemsOutput {
            consumed_capacity: self.consumed_capacity,
item_collection_metrics: self.item_collection_metrics,
        })
    }
}
