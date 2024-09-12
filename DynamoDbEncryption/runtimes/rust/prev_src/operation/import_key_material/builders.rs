// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::import_key_material::_import_key_material_response::ImportKeyMaterialResponseBuilder;

pub use crate::operation::import_key_material::_import_key_material_request::ImportKeyMaterialRequestBuilder;

impl ImportKeyMaterialRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::import_key_material::ImportKeyMaterialResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.import_key_material();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportKeyMaterial`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportKeyMaterialFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::import_key_material::builders::ImportKeyMaterialRequestBuilder,
}
impl ImportKeyMaterialFluentBuilder {
    /// Creates a new `ImportKeyMaterial`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ImportKeyMaterial as a reference.
    pub fn as_input(&self) -> &crate::operation::import_key_material::builders::ImportKeyMaterialRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_key_material::ImportKeyMaterialResponse,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::operation::import_key_material::ImportKeyMaterial::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn encrypted_key_material(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.encrypted_key_material(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encrypted_key_material(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_encrypted_key_material(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encrypted_key_material(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_encrypted_key_material()
}
#[allow(missing_docs)] // documentation missing in model
pub fn expiration_model(mut self, input: impl ::std::convert::Into<kms::types::ExpirationModelType>) -> Self {
    self.inner = self.inner.expiration_model(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_expiration_model(mut self, input: ::std::option::Option<kms::types::ExpirationModelType>) -> Self {
    self.inner = self.inner.set_expiration_model(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_expiration_model(&self) -> &::std::option::Option<kms::types::ExpirationModelType> {
    self.inner.get_expiration_model()
}
#[allow(missing_docs)] // documentation missing in model
pub fn import_token(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.import_token(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_import_token(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_import_token(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_import_token(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_import_token()
}
#[allow(missing_docs)] // documentation missing in model
pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.key_id(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_key_id(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_key_id()
}
#[allow(missing_docs)] // documentation missing in model
pub fn valid_to(mut self, input: impl ::std::convert::Into<::aws_smithy_types::DateTime>) -> Self {
    self.inner = self.inner.valid_to(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_valid_to(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
    self.inner = self.inner.set_valid_to(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_valid_to(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
    self.inner.get_valid_to()
}
}
