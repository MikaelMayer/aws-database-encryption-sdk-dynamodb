// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_multi_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::operation::create_multi_keyring::_create_multi_keyring_input::CreateMultiKeyringInputBuilder;

impl CreateMultiKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::create_multi_keyring::CreateKeyringOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_multi_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateMultiKeyring`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateMultiKeyringFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_multi_keyring::builders::CreateMultiKeyringInputBuilder,
}
impl CreateMultiKeyringFluentBuilder {
    /// Creates a new `CreateMultiKeyring`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateMultiKeyring as a reference.
    pub fn as_input(&self) -> &crate::operation::create_multi_keyring::builders::CreateMultiKeyringInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_multi_keyring::CreateKeyringOutput,
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
        crate::operation::create_multi_keyring::CreateMultiKeyring::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn child_keyrings(mut self, input: impl ::std::convert::Into<::std::vec::Vec<material_providers::types::keyring::KeyringRef>>) -> Self {
    self.inner = self.inner.child_keyrings(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_child_keyrings(mut self, input: ::std::option::Option<::std::vec::Vec<material_providers::types::keyring::KeyringRef>>) -> Self {
    self.inner = self.inner.set_child_keyrings(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_child_keyrings(&self) -> &::std::option::Option<::std::vec::Vec<material_providers::types::keyring::KeyringRef>> {
    self.inner.get_child_keyrings()
}
#[allow(missing_docs)] // documentation missing in model
pub fn generator(mut self, input: impl ::std::convert::Into<material_providers::types::keyring::KeyringRef>) -> Self {
    self.inner = self.inner.generator(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_generator(mut self, input: ::std::option::Option<material_providers::types::keyring::KeyringRef>) -> Self {
    self.inner = self.inner.set_generator(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_generator(&self) -> &::std::option::Option<material_providers::types::keyring::KeyringRef> {
    self.inner.get_generator()
}
}