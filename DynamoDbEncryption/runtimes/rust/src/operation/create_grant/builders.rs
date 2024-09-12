// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::create_grant::_create_grant_response::CreateGrantResponseBuilder;

pub use crate::operation::create_grant::_create_grant_request::CreateGrantRequestBuilder;

impl CreateGrantRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::create_grant::CreateGrantResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.create_grant();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateGrant`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateGrantFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::create_grant::builders::CreateGrantRequestBuilder,
}
impl CreateGrantFluentBuilder {
    /// Creates a new `CreateGrant`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateGrant as a reference.
    pub fn as_input(&self) -> &crate::operation::create_grant::builders::CreateGrantRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_grant::CreateGrantResponse,
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
        crate::operation::create_grant::CreateGrant::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn constraints(mut self, input: impl ::std::convert::Into<kms::types::GrantConstraints>) -> Self {
    self.inner = self.inner.constraints(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_constraints(mut self, input: ::std::option::Option<kms::types::GrantConstraints>) -> Self {
    self.inner = self.inner.set_constraints(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_constraints(&self) -> &::std::option::Option<kms::types::GrantConstraints> {
    self.inner.get_constraints()
}
#[allow(missing_docs)] // documentation missing in model
pub fn dry_run(mut self, input: impl ::std::convert::Into<::std::primitive::bool>) -> Self {
    self.inner = self.inner.dry_run(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_dry_run(mut self, input: ::std::option::Option<::std::primitive::bool>) -> Self {
    self.inner = self.inner.set_dry_run(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_dry_run(&self) -> &::std::option::Option<::std::primitive::bool> {
    self.inner.get_dry_run()
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
pub fn grantee_principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.grantee_principal(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_grantee_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_grantee_principal(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_grantee_principal(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_grantee_principal()
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
pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_name()
}
#[allow(missing_docs)] // documentation missing in model
pub fn operations(mut self, input: impl ::std::convert::Into<::std::vec::Vec<kms::types::GrantOperation>>) -> Self {
    self.inner = self.inner.operations(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_operations(mut self, input: ::std::option::Option<::std::vec::Vec<kms::types::GrantOperation>>) -> Self {
    self.inner = self.inner.set_operations(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_operations(&self) -> &::std::option::Option<::std::vec::Vec<kms::types::GrantOperation>> {
    self.inner.get_operations()
}
#[allow(missing_docs)] // documentation missing in model
pub fn retiring_principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.retiring_principal(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_retiring_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_retiring_principal(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_retiring_principal(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_retiring_principal()
}
}
