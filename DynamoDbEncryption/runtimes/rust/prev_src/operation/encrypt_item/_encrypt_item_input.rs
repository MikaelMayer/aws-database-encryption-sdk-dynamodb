// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EncryptItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub plaintext_item: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
}
impl EncryptItemInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn plaintext_item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.plaintext_item
}
}
impl EncryptItemInput {
    /// Creates a new builder-style object to manufacture [`EncryptItemInput`](crate::operation::encrypt_item::builders::EncryptItemInput).
    pub fn builder() -> crate::operation::encrypt_item::builders::EncryptItemInputBuilder {
        crate::operation::encrypt_item::builders::EncryptItemInputBuilder::default()
    }
}

/// A builder for [`EncryptItemInput`](crate::operation::operation::EncryptItemInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EncryptItemInputBuilder {
    pub(crate) plaintext_item: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
}
impl EncryptItemInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn plaintext_item(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.plaintext_item = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_plaintext_item(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.plaintext_item = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_plaintext_item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.plaintext_item
}
    /// Consumes the builder and constructs a [`EncryptItemInput`](crate::operation::operation::EncryptItemInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::encrypt_item::EncryptItemInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::encrypt_item::EncryptItemInput {
            plaintext_item: self.plaintext_item,
        })
    }
}
