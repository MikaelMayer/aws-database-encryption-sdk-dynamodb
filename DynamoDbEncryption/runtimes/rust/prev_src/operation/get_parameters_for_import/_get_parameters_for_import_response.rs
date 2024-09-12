// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetParametersForImportResponse {
    #[allow(missing_docs)] // documentation missing in model
pub import_token: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub parameters_valid_to: ::std::option::Option<::aws_smithy_types::DateTime>,
#[allow(missing_docs)] // documentation missing in model
pub public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetParametersForImportResponse {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_token(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.import_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn parameters_valid_to(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.parameters_valid_to
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.public_key
}
}
impl GetParametersForImportResponse {
    /// Creates a new builder-style object to manufacture [`GetParametersForImportResponse`](crate::operation::get_parameters_for_import::builders::GetParametersForImportResponse).
    pub fn builder() -> crate::operation::get_parameters_for_import::builders::GetParametersForImportResponseBuilder {
        crate::operation::get_parameters_for_import::builders::GetParametersForImportResponseBuilder::default()
    }
}

/// A builder for [`GetParametersForImportResponse`](crate::operation::operation::GetParametersForImportResponse).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetParametersForImportResponseBuilder {
    pub(crate) import_token: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) parameters_valid_to: ::std::option::Option<::aws_smithy_types::DateTime>,
pub(crate) public_key: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl GetParametersForImportResponseBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn import_token(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.import_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_import_token(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.import_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_import_token(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.import_token
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
pub fn parameters_valid_to(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.parameters_valid_to = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_parameters_valid_to(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.parameters_valid_to = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_parameters_valid_to(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.parameters_valid_to
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
    /// Consumes the builder and constructs a [`GetParametersForImportResponse`](crate::operation::operation::GetParametersForImportResponse).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_parameters_for_import::GetParametersForImportResponse,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_parameters_for_import::GetParametersForImportResponse {
            import_token: self.import_token,
key_id: self.key_id,
parameters_valid_to: self.parameters_valid_to,
public_key: self.public_key,
        })
    }
}
