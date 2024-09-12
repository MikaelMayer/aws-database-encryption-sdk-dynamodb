// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateDataKeyPairWithoutPlaintextResponse {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub key_pair_spec: ::std::option::Option<kms::types::DataKeyPairSpec>,
#[allow(missing_docs)] // documentation missing in model
pub private_key_ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateDataKeyPairWithoutPlaintextResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_pair_spec(&self) -> &::std::option::Option<kms::types::DataKeyPairSpec> {
    &self.key_pair_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn private_key_ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.private_key_ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.public_key
}
}
impl GenerateDataKeyPairWithoutPlaintextResponse {
    /// Creates a new builder-style object to manufacture [`GenerateDataKeyPairWithoutPlaintextResponse`](crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextResponse).
    pub fn builder() -> crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextResponseBuilder {
        crate::operation::generate_data_key_pair_without_plaintext::builders::GenerateDataKeyPairWithoutPlaintextResponseBuilder::default()
    }
}

/// A builder for [`GenerateDataKeyPairWithoutPlaintextResponse`](crate::operation::operation::GenerateDataKeyPairWithoutPlaintextResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateDataKeyPairWithoutPlaintextResponseBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) key_pair_spec: ::std::option::Option<kms::types::DataKeyPairSpec>,
pub(crate) private_key_ciphertext_blob: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateDataKeyPairWithoutPlaintextResponseBuilder {
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
pub fn key_pair_spec(mut self, input: impl ::std::convert::Into<kms::types::DataKeyPairSpec>) -> Self {
    self.key_pair_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_pair_spec(mut self, input: ::std::option::Option<kms::types::DataKeyPairSpec>) -> Self {
    self.key_pair_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_pair_spec(&self) -> &::std::option::Option<kms::types::DataKeyPairSpec> {
    &self.key_pair_spec
}
#[allow(missing_docs)] // documentation missing in model
pub fn private_key_ciphertext_blob(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.private_key_ciphertext_blob = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_private_key_ciphertext_blob(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.private_key_ciphertext_blob = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_private_key_ciphertext_blob(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.private_key_ciphertext_blob
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.public_key = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_public_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.public_key = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.public_key
}
    /// Consumes the builder and constructs a [`GenerateDataKeyPairWithoutPlaintextResponse`](crate::operation::operation::GenerateDataKeyPairWithoutPlaintextResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_data_key_pair_without_plaintext::GenerateDataKeyPairWithoutPlaintextResponse {
            key_id: self.key_id,
key_pair_spec: self.key_pair_spec,
private_key_ciphertext_blob: self.private_key_ciphertext_blob,
public_key: self.public_key,
        })
    }
}
