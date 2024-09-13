// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValidatePublicKeyOutput {
    #[allow(missing_docs)] // documentation missing in model
pub success: ::std::option::Option<::std::primitive::bool>,
}
impl ValidatePublicKeyOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn success(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.success
}
}
impl ValidatePublicKeyOutput {
    /// Creates a new builder-style object to manufacture [`ValidatePublicKeyOutput`](crate::primitives::operation::validate_public_key::builders::ValidatePublicKeyOutput).
    pub fn builder() -> crate::primitives::operation::validate_public_key::builders::ValidatePublicKeyOutputBuilder {
        crate::primitives::operation::validate_public_key::builders::ValidatePublicKeyOutputBuilder::default()
    }
}

/// A builder for [`ValidatePublicKeyOutput`](crate::primitives::operation::operation::ValidatePublicKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ValidatePublicKeyOutputBuilder {
    pub(crate) success: ::std::option::Option<::std::primitive::bool>,
}
impl ValidatePublicKeyOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn success(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.success = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_success(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.success = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_success(&self) -> &::std::option::Option<::std::primitive::bool> {
    &self.success
}
    /// Consumes the builder and constructs a [`ValidatePublicKeyOutput`](crate::primitives::operation::operation::ValidatePublicKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::primitives::operation::validate_public_key::ValidatePublicKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::primitives::operation::validate_public_key::ValidatePublicKeyOutput {
            success: self.success,
        })
    }
}