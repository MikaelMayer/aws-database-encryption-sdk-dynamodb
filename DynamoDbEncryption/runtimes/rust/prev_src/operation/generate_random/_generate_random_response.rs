// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GenerateRandomResponse {
    #[allow(missing_docs)] // documentation missing in model
pub ciphertext_for_recipient: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateRandomResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn ciphertext_for_recipient(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.ciphertext_for_recipient
}
#[allow(missing_docs)] // documentation missing in model
pub fn plaintext(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.plaintext
}
}
impl GenerateRandomResponse {
    /// Creates a new builder-style object to manufacture [`GenerateRandomResponse`](crate::operation::generate_random::builders::GenerateRandomResponse).
    pub fn builder() -> crate::operation::generate_random::builders::GenerateRandomResponseBuilder {
        crate::operation::generate_random::builders::GenerateRandomResponseBuilder::default()
    }
}

/// A builder for [`GenerateRandomResponse`](crate::operation::operation::GenerateRandomResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GenerateRandomResponseBuilder {
    pub(crate) ciphertext_for_recipient: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) plaintext: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GenerateRandomResponseBuilder {
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
    /// Consumes the builder and constructs a [`GenerateRandomResponse`](crate::operation::operation::GenerateRandomResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_random::GenerateRandomResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::generate_random::GenerateRandomResponse {
            ciphertext_for_recipient: self.ciphertext_for_recipient,
plaintext: self.plaintext,
        })
    }
}