// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateDataKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub ciphertext_for_recipient: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateDataKeyResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_for_recipient(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_for_recipient
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
}
impl GenerateDataKeyResponse {
    /// Creates a new builder-style object to manufacture [`GenerateDataKeyResponse`](crate::operation::generate_data_key::builders::GenerateDataKeyResponse).
    pub fn builder() -> crate::operation::generate_data_key::builders::GenerateDataKeyResponseBuilder {
        crate::operation::generate_data_key::builders::GenerateDataKeyResponseBuilder::default()
    }
}

/// A builder for [`GenerateDataKeyResponse`](crate::operation::operation::GenerateDataKeyResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateDataKeyResponseBuilder {
    pub(crate) ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) ciphertext_for_recipient: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateDataKeyResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_blob(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext_blob = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ciphertext_blob(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext_blob = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_for_recipient(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext_for_recipient = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ciphertext_for_recipient(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.ciphertext_for_recipient = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ciphertext_for_recipient(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_for_recipient
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.key_id = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.key_id = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn plaintext(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.plaintext = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_plaintext(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.plaintext = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
    /// Consumes the builder and constructs a [`GenerateDataKeyResponse`](crate::operation::operation::GenerateDataKeyResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_data_key::GenerateDataKeyResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_data_key::GenerateDataKeyResponse {
            ciphertext_blob: self.ciphertext_blob,
ciphertext_for_recipient: self.ciphertext_for_recipient,
key_id: self.key_id,
plaintext: self.plaintext,
        })
    }
}
