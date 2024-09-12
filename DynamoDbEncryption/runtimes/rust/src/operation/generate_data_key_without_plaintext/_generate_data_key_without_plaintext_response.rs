// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateDataKeyWithoutPlaintextResponse {
    #[allow(missing_docs)] // documentation missing in model
pub ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
}
impl GenerateDataKeyWithoutPlaintextResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
}
impl GenerateDataKeyWithoutPlaintextResponse {
    /// Creates a new builder-style object to manufacture [`GenerateDataKeyWithoutPlaintextResponse`](crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextResponse).
    pub fn builder() -> crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextResponseBuilder {
        crate::operation::generate_data_key_without_plaintext::builders::GenerateDataKeyWithoutPlaintextResponseBuilder::default()
    }
}

/// A builder for [`GenerateDataKeyWithoutPlaintextResponse`](crate::operation::operation::GenerateDataKeyWithoutPlaintextResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateDataKeyWithoutPlaintextResponseBuilder {
    pub(crate) ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl GenerateDataKeyWithoutPlaintextResponseBuilder {
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
    /// Consumes the builder and constructs a [`GenerateDataKeyWithoutPlaintextResponse`](crate::operation::operation::GenerateDataKeyWithoutPlaintextResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_data_key_without_plaintext::GenerateDataKeyWithoutPlaintextResponse {
            ciphertext_blob: self.ciphertext_blob,
key_id: self.key_id,
        })
    }
}
