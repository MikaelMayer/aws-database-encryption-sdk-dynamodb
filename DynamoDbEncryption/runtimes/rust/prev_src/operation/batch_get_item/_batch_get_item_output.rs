// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BatchGetItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
#[allow(missing_docs)] // documentation missing in model
pub responses: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>>,
#[allow(missing_docs)] // documentation missing in model
pub unprocessed_keys: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>,
}
impl BatchGetItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn consumed_capacity(&self) -> &::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>> {
    &self.consumed_capacity
}
#[allow(missing_docs)] // documentation missing in model
pub fn responses(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>> {
    &self.responses
}
#[allow(missing_docs)] // documentation missing in model
pub fn unprocessed_keys(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>> {
    &self.unprocessed_keys
}
}
impl BatchGetItemOutput {
    /// Creates a new builder-style object to manufacture [`BatchGetItemOutput`](crate::operation::batch_get_item::builders::BatchGetItemOutput).
    pub fn builder() -> crate::operation::batch_get_item::builders::BatchGetItemOutputBuilder {
        crate::operation::batch_get_item::builders::BatchGetItemOutputBuilder::default()
    }
}

/// A builder for [`BatchGetItemOutput`](crate::operation::operation::BatchGetItemOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct BatchGetItemOutputBuilder {
    pub(crate) consumed_capacity: ::std::option::Option<::std::vec::Vec<dynamodb::types::ConsumedCapacity>>,
pub(crate) responses: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>>,
pub(crate) unprocessed_keys: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>,
}
impl BatchGetItemOutputBuilder {
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
pub fn responses(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>>) -> Self {
    self.responses = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_responses(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>>) -> Self {
    self.responses = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_responses(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>> {
    &self.responses
}
#[allow(missing_docs)] // documentation missing in model
pub fn unprocessed_keys(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>) -> Self {
    self.unprocessed_keys = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_unprocessed_keys(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>>) -> Self {
    self.unprocessed_keys = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_unprocessed_keys(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::KeysAndAttributes>> {
    &self.unprocessed_keys
}
    /// Consumes the builder and constructs a [`BatchGetItemOutput`](crate::operation::operation::BatchGetItemOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_get_item::BatchGetItemOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::batch_get_item::BatchGetItemOutput {
            consumed_capacity: self.consumed_capacity,
responses: self.responses,
unprocessed_keys: self.unprocessed_keys,
        })
    }
}
