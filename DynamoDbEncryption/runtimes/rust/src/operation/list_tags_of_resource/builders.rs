// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::list_tags_of_resource::_list_tags_of_resource_output::ListTagsOfResourceOutputBuilder;

pub use crate::operation::list_tags_of_resource::_list_tags_of_resource_input::ListTagsOfResourceInputBuilder;

impl ListTagsOfResourceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::list_tags_of_resource::ListTagsOfResourceOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.list_tags_of_resource();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTagsOfResource`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTagsOfResourceFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::list_tags_of_resource::builders::ListTagsOfResourceInputBuilder,
}
impl ListTagsOfResourceFluentBuilder {
    /// Creates a new `ListTagsOfResource`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ListTagsOfResource as a reference.
    pub fn as_input(&self) -> &crate::operation::list_tags_of_resource::builders::ListTagsOfResourceInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_tags_of_resource::ListTagsOfResourceOutput,
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
        crate::operation::list_tags_of_resource::ListTagsOfResource::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.next_token(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_next_token(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_next_token()
}
#[allow(missing_docs)] // documentation missing in model
pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.resource_arn(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_resource_arn(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_resource_arn()
}
}
