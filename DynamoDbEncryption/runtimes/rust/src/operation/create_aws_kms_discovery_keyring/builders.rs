// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_aws_kms_discovery_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::operation::create_aws_kms_discovery_keyring::_create_aws_kms_discovery_keyring_input::CreateAwsKmsDiscoveryKeyringInputBuilder;

impl CreateAwsKmsDiscoveryKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        material_providers::types::keyring::KeyringRef,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_aws_kms_discovery_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAwsKmsDiscoveryKeyring`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAwsKmsDiscoveryKeyringFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringInputBuilder,
}
impl CreateAwsKmsDiscoveryKeyringFluentBuilder {
    /// Creates a new `CreateAwsKmsDiscoveryKeyring`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateAwsKmsDiscoveryKeyring as a reference.
    pub fn as_input(&self) -> &crate::operation::create_aws_kms_discovery_keyring::builders::CreateAwsKmsDiscoveryKeyringInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        material_providers::types::keyring::KeyringRef,
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
        crate::operation::create_aws_kms_discovery_keyring::CreateAwsKmsDiscoveryKeyring::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn discovery_filter(mut self, input: impl ::std::convert::Into<material_providers::types::DiscoveryFilter>) -> Self {
    self.inner = self.inner.discovery_filter(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_discovery_filter(mut self, input: ::std::option::Option<material_providers::types::DiscoveryFilter>) -> Self {
    self.inner = self.inner.set_discovery_filter(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_discovery_filter(&self) -> &::std::option::Option<material_providers::types::DiscoveryFilter> {
    self.inner.get_discovery_filter()
}
#[allow(missing_docs)] // documentation missing in model
pub fn grant_tokens(mut self, input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>) -> Self {
    self.inner = self.inner.grant_tokens(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grant_tokens(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
    self.inner = self.inner.set_grant_tokens(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
    self.inner.get_grant_tokens()
}
#[allow(missing_docs)] // documentation missing in model
pub fn kms_client(mut self, input: impl ::std::convert::Into<kms::client::Client>) -> Self {
    self.inner = self.inner.kms_client(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_kms_client(mut self, input: ::std::option::Option<kms::client::Client>) -> Self {
    self.inner = self.inner.set_kms_client(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_kms_client(&self) -> &::std::option::Option<kms::client::Client> {
    self.inner.get_kms_client()
}
}