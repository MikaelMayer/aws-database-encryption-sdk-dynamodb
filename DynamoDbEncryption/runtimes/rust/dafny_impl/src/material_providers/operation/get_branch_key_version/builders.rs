// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::material_providers::operation::get_branch_key_version::_get_branch_key_version_output::GetBranchKeyVersionOutputBuilder;

pub use crate::material_providers::operation::get_branch_key_version::_get_branch_key_version_input::GetBranchKeyVersionInputBuilder;

impl GetBranchKeyVersionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::material_providers::client::Client,
    ) -> ::std::result::Result<
        crate::material_providers::operation::get_branch_key_version::GetBranchKeyVersionOutput,
        crate::material_providers::types::error::Error,
    > {
        let mut fluent_builder = client.get_branch_key_version();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetBranchKeyVersion`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBranchKeyVersionFluentBuilder {
    client: crate::material_providers::client::Client,
    pub(crate) inner: crate::material_providers::operation::get_branch_key_version::builders::GetBranchKeyVersionInputBuilder,
}
impl GetBranchKeyVersionFluentBuilder {
    /// Creates a new `GetBranchKeyVersion`.
    pub(crate) fn new(client: crate::material_providers::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetBranchKeyVersion as a reference.
    pub fn as_input(&self) -> &crate::material_providers::operation::get_branch_key_version::builders::GetBranchKeyVersionInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::material_providers::operation::get_branch_key_version::GetBranchKeyVersionOutput,
        crate::material_providers::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::material_providers::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::material_providers::operation::get_branch_key_version::GetBranchKeyVersion::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn branch_key_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.branch_key_identifier(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_branch_key_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_branch_key_identifier(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_branch_key_identifier()
}
#[allow(missing_docs)] // documentation missing in model
pub fn branch_key_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.branch_key_version(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_branch_key_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_branch_key_version(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_branch_key_version(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_branch_key_version()
}
}
