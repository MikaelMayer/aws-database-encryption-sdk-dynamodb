// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EncryptItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub encrypted_item: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
#[allow(missing_docs)] // documentation missing in model
pub parsed_header: ::std::option::Option<item_encryptor::types::ParsedHeader>,
}
impl EncryptItemOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn encrypted_item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.encrypted_item
}
#[allow(missing_docs)] // documentation missing in model
pub fn parsed_header(&self) -> &::std::option::Option<item_encryptor::types::ParsedHeader> {
    &self.parsed_header
}
}
impl EncryptItemOutput {
    /// Creates a new builder-style object to manufacture [`EncryptItemOutput`](crate::operation::encrypt_item::builders::EncryptItemOutput).
    pub fn builder() -> crate::operation::encrypt_item::builders::EncryptItemOutputBuilder {
        crate::operation::encrypt_item::builders::EncryptItemOutputBuilder::default()
    }
}

/// A builder for [`EncryptItemOutput`](crate::operation::operation::EncryptItemOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct EncryptItemOutputBuilder {
    pub(crate) encrypted_item: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>,
pub(crate) parsed_header: ::std::option::Option<item_encryptor::types::ParsedHeader>,
}
impl EncryptItemOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn encrypted_item(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.encrypted_item = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encrypted_item(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>) -> Self {
    self.encrypted_item = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encrypted_item(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>> {
    &self.encrypted_item
}
#[allow(missing_docs)] // documentation missing in model
pub fn parsed_header(mut self, input: impl ::std::convert::Into<item_encryptor::types::ParsedHeader>) -> Self {
    self.parsed_header = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_parsed_header(mut self, input: ::std::option::Option<item_encryptor::types::ParsedHeader>) -> Self {
    self.parsed_header = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_parsed_header(&self) -> &::std::option::Option<item_encryptor::types::ParsedHeader> {
    &self.parsed_header
}
    /// Consumes the builder and constructs a [`EncryptItemOutput`](crate::operation::operation::EncryptItemOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::encrypt_item::EncryptItemOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::encrypt_item::EncryptItemOutput {
            encrypted_item: self.encrypted_item,
parsed_header: self.parsed_header,
        })
    }
}