// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::generate_mac::_generate_mac_response::GenerateMacResponseBuilder;

pub use crate::operation::generate_mac::_generate_mac_request::GenerateMacRequestBuilder;

impl GenerateMacRequestBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::generate_mac::GenerateMacResponse,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.generate_mac();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GenerateMac`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GenerateMacFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::generate_mac::builders::GenerateMacRequestBuilder,
}
impl GenerateMacFluentBuilder {
    /// Creates a new `GenerateMac`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GenerateMac as a reference.
    pub fn as_input(&self) -> &crate::operation::generate_mac::builders::GenerateMacRequestBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::generate_mac::GenerateMacResponse,
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
        crate::operation::generate_mac::GenerateMac::send(&self.client, input).await
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
pub fn mac_algorithm(mut self, input: impl ::std::convert::Into<kms::types::MacAlgorithmSpec>) -> Self {
    self.inner = self.inner.mac_algorithm(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_mac_algorithm(mut self, input: ::std::option::Option<kms::types::MacAlgorithmSpec>) -> Self {
    self.inner = self.inner.set_mac_algorithm(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_mac_algorithm(&self) -> &::std::option::Option<kms::types::MacAlgorithmSpec> {
    self.inner.get_mac_algorithm()
}
#[allow(missing_docs)] // documentation missing in model
pub fn message(mut self, input: impl ::std::convert::Into<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.message(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_message(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
    self.inner = self.inner.set_message(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_message(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
    self.inner.get_message()
}
}
