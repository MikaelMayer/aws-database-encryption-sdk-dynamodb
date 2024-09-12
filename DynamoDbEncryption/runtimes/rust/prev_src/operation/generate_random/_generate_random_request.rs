// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateRandomRequest {
    #[allow(missing_docs)] // documentation missing in model
pub custom_key_store_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub number_of_bytes: ::std::option::Option<::std::primitive::i32>,
#[allow(missing_docs)] // documentation missing in model
pub recipient: ::std::option::Option<kms::types::RecipientInfo>,
}
impl GenerateRandomRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn number_of_bytes(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.number_of_bytes
}
#[allow(missing_docs)] // documentation missing in model
pub fn recipient(&self) -> &::std::option::Option<kms::types::RecipientInfo> {
    &self.recipient
}
}
impl GenerateRandomRequest {
    /// Creates a new builder-style object to manufacture [`GenerateRandomRequest`](crate::operation::generate_random::builders::GenerateRandomRequest).
    pub fn builder() -> crate::operation::generate_random::builders::GenerateRandomRequestBuilder {
        crate::operation::generate_random::builders::GenerateRandomRequestBuilder::default()
    }
}

/// A builder for [`GenerateRandomRequest`](crate::operation::operation::GenerateRandomRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateRandomRequestBuilder {
    pub(crate) custom_key_store_id: ::std::option::Option<::std::string::String>,
pub(crate) number_of_bytes: ::std::option::Option<::std::primitive::i32>,
pub(crate) recipient: ::std::option::Option<kms::types::RecipientInfo>,
}
impl GenerateRandomRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn custom_key_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.custom_key_store_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_custom_key_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.custom_key_store_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_custom_key_store_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.custom_key_store_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn number_of_bytes(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
    self.number_of_bytes = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_number_of_bytes(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
    self.number_of_bytes = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_number_of_bytes(&self) -> &::std::option::Option<::std::primitive::i32> {
    &self.number_of_bytes
}
#[allow(missing_docs)] // documentation missing in model
pub fn recipient(mut self, input: impl ::std::convert::Into<kms::types::RecipientInfo>) -> Self {
    self.recipient = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_recipient(mut self, input: ::std::option::Option<kms::types::RecipientInfo>) -> Self {
    self.recipient = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_recipient(&self) -> &::std::option::Option<kms::types::RecipientInfo> {
    &self.recipient
}
    /// Consumes the builder and constructs a [`GenerateRandomRequest`](crate::operation::operation::GenerateRandomRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_random::GenerateRandomRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_random::GenerateRandomRequest {
            custom_key_store_id: self.custom_key_store_id,
number_of_bytes: self.number_of_bytes,
recipient: self.recipient,
        })
    }
}
