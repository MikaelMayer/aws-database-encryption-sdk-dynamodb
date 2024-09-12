// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub request_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>,
#[allow(missing_docs)] // documentation missing in model
pub return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
}
impl BatchGetItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn request_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>> {
    &self.request_items
}
#[allow(missing_docs)] // documentation missing in model
pub fn return_consumed_capacity(&self) -> &::std::option::Option<dynamodb::types::ReturnConsumedCapacity> {
    &self.return_consumed_capacity
}
}
impl BatchGetItemInput {
    /// Creates a new builder-style object to manufacture [`BatchGetItemInput`](crate::operation::batch_get_item::builders::BatchGetItemInput).
    pub fn builder() -> crate::operation::batch_get_item::builders::BatchGetItemInputBuilder {
        crate::operation::batch_get_item::builders::BatchGetItemInputBuilder::default()
    }
}

/// A builder for [`BatchGetItemInput`](crate::operation::operation::BatchGetItemInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetItemInputBuilder {
    pub(crate) request_items: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>,
pub(crate) return_consumed_capacity: ::std::option::Option<dynamodb::types::ReturnConsumedCapacity>,
}
impl BatchGetItemInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn request_items(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>) -> Self {
    self.request_items = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_request_items(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>) -> Self {
    self.request_items = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_request_items(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>> {
    &self.request_items
}
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
    /// Consumes the builder and constructs a [`BatchGetItemInput`](crate::operation::operation::BatchGetItemInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_get_item::BatchGetItemInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_get_item::BatchGetItemInput {
            request_items: self.request_items,
return_consumed_capacity: self.return_consumed_capacity,
        })
    }
}
