// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetParametersForImportRequest {
    #[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub wrapping_algorithm: ::std::option::Option<kms::types::AlgorithmSpec>,
#[allow(missing_docs)] // documentation missing in model
pub wrapping_key_spec: ::std::option::Option<kms::types::WrappingKeySpec>,
}
impl GetParametersForImportRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn wrapping_algorithm(&self) -> &::std::option::Option<kms::types::AlgorithmSpec> {
    &self.wrapping_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn wrapping_key_spec(&self) -> &::std::option::Option<kms::types::WrappingKeySpec> {
    &self.wrapping_key_spec
}
}
impl GetParametersForImportRequest {
    /// Creates a new builder-style object to manufacture [`GetParametersForImportRequest`](crate::operation::get_parameters_for_import::builders::GetParametersForImportRequest).
    pub fn builder() -> crate::operation::get_parameters_for_import::builders::GetParametersForImportRequestBuilder {
        crate::operation::get_parameters_for_import::builders::GetParametersForImportRequestBuilder::default()
    }
}

/// A builder for [`GetParametersForImportRequest`](crate::operation::operation::GetParametersForImportRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetParametersForImportRequestBuilder {
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) wrapping_algorithm: ::std::option::Option<kms::types::AlgorithmSpec>,
pub(crate) wrapping_key_spec: ::std::option::Option<kms::types::WrappingKeySpec>,
}
impl GetParametersForImportRequestBuilder {
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
pub fn wrapping_algorithm(mut self, input: impl ::std::convert::Into<kms::types::AlgorithmSpec>) -> Self {
    self.wrapping_algorithm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_wrapping_algorithm(mut self, input: ::std::option::Option<kms::types::AlgorithmSpec>) -> Self {
    self.wrapping_algorithm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_wrapping_algorithm(&self) -> &::std::option::Option<kms::types::AlgorithmSpec> {
    &self.wrapping_algorithm
}
#[allow(missing_docs)] // documentation missing in model
pub fn wrapping_key_spec(mut self, input: impl ::std::convert::Into<kms::types::WrappingKeySpec>) -> Self {
    self.wrapping_key_spec = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_wrapping_key_spec(mut self, input: ::std::option::Option<kms::types::WrappingKeySpec>) -> Self {
    self.wrapping_key_spec = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_wrapping_key_spec(&self) -> &::std::option::Option<kms::types::WrappingKeySpec> {
    &self.wrapping_key_spec
}
    /// Consumes the builder and constructs a [`GetParametersForImportRequest`](crate::operation::operation::GetParametersForImportRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_parameters_for_import::GetParametersForImportRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_parameters_for_import::GetParametersForImportRequest {
            key_id: self.key_id,
wrapping_algorithm: self.wrapping_algorithm,
wrapping_key_spec: self.wrapping_key_spec,
        })
    }
}
