// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::primitives::operation::validate_public_key::_validate_public_key_output::ValidatePublicKeyOutputBuilder;

pub use crate::primitives::operation::validate_public_key::_validate_public_key_input::ValidatePublicKeyInputBuilder;

impl ValidatePublicKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::primitives::client::Client,
    ) -> ::std::result::Result<
        crate::primitives::operation::validate_public_key::ValidatePublicKeyOutput,
        crate::primitives::types::error::Error,
    > {
        let mut fluent_builder = client.validate_public_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ValidatePublicKey`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ValidatePublicKeyFluentBuilder {
    client: crate::primitives::client::Client,
    pub(crate) inner: crate::primitives::operation::validate_public_key::builders::ValidatePublicKeyInputBuilder,
}
impl ValidatePublicKeyFluentBuilder {
    /// Creates a new `ValidatePublicKey`.
    pub(crate) fn new(client: crate::primitives::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ValidatePublicKey as a reference.
    pub fn as_input(&self) -> &crate::primitives::operation::validate_public_key::builders::ValidatePublicKeyInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::primitives::operation::validate_public_key::ValidatePublicKeyOutput,
        crate::primitives::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::primitives::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::primitives::operation::validate_public_key::ValidatePublicKey::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn ecc_curve(mut self, input: impl ::std::convert::Into<crate::primitives::types::EcdhCurveSpec>) -> Self {
    self.inner = self.inner.ecc_curve(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_ecc_curve(mut self, input: ::std::option::Option<crate::primitives::types::EcdhCurveSpec>) -> Self {
    self.inner = self.inner.set_ecc_curve(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_ecc_curve(&self) -> &::std::option::Option<crate::primitives::types::EcdhCurveSpec> {
    self.inner.get_ecc_curve()
}
#[allow(missing_docs)] // documentation missing in model
pub fn public_key(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.public_key(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_public_key(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_public_key(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_public_key(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_public_key()
}
}