// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportKeyMaterialRequest {
    #[allow(missing_docs)] // documentation missing in model
pub encrypted_key_material: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub expiration_model: ::std::option::Option<kms::types::ExpirationModelType>,
#[allow(missing_docs)] // documentation missing in model
pub import_token: ::std::option::Option<::aws_smithy_types::Blob>,
#[allow(missing_docs)] // documentation missing in model
pub key_id: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub valid_to: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ImportKeyMaterialRequest {
    #[allow(missing_docs)] // documentation missing in model
pub fn encrypted_key_material(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.encrypted_key_material
}
#[allow(missing_docs)] // documentation missing in model
pub fn expiration_model(&self) -> &::std::option::Option<kms::types::ExpirationModelType> {
    &self.expiration_model
}
#[allow(missing_docs)] // documentation missing in model
pub fn import_token(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.import_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(&self) -> &::std::option::Option<::std::string::String> {
    &self.key_id
}
#[allow(missing_docs)] // documentation missing in model
pub fn valid_to(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.valid_to
}
}
impl ImportKeyMaterialRequest {
    /// Creates a new builder-style object to manufacture [`ImportKeyMaterialRequest`](crate::operation::import_key_material::builders::ImportKeyMaterialRequest).
    pub fn builder() -> crate::operation::import_key_material::builders::ImportKeyMaterialRequestBuilder {
        crate::operation::import_key_material::builders::ImportKeyMaterialRequestBuilder::default()
    }
}

/// A builder for [`ImportKeyMaterialRequest`](crate::operation::operation::ImportKeyMaterialRequest).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportKeyMaterialRequestBuilder {
    pub(crate) encrypted_key_material: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) expiration_model: ::std::option::Option<kms::types::ExpirationModelType>,
pub(crate) import_token: ::std::option::Option<::aws_smithy_types::Blob>,
pub(crate) key_id: ::std::option::Option<::std::string::String>,
pub(crate) valid_to: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl ImportKeyMaterialRequestBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn encrypted_key_material(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.encrypted_key_material = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encrypted_key_material(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.encrypted_key_material = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encrypted_key_material(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    &self.encrypted_key_material
}
#[allow(missing_docs)] // documentation missing in model
pub fn expiration_model(mut self, input: impl ::std::convert::Into<kms::types::ExpirationModelType>) -> Self {
    self.expiration_model = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expiration_model(mut self, input: ::std::option::Option<kms::types::ExpirationModelType>) -> Self {
    self.expiration_model = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expiration_model(&self) -> &::std::option::Option<kms::types::ExpirationModelType> {
    &self.expiration_model
}
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
pub fn valid_to(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.valid_to = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_valid_to(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.valid_to = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_valid_to(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    &self.valid_to
}
    /// Consumes the builder and constructs a [`ImportKeyMaterialRequest`](crate::operation::operation::ImportKeyMaterialRequest).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_key_material::ImportKeyMaterialRequest,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::import_key_material::ImportKeyMaterialRequest {
            encrypted_key_material: self.encrypted_key_material,
expiration_model: self.expiration_model,
import_token: self.import_token,
key_id: self.key_id,
valid_to: self.valid_to,
        })
    }
}
