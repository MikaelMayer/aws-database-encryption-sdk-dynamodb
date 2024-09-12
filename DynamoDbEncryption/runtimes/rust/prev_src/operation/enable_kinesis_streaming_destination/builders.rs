// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_output::EnableKinesisStreamingDestinationOutputBuilder;

pub use crate::operation::enable_kinesis_streaming_destination::_enable_kinesis_streaming_destination_input::EnableKinesisStreamingDestinationInputBuilder;

impl EnableKinesisStreamingDestinationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.enable_kinesis_streaming_destination();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EnableKinesisStreamingDestination`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableKinesisStreamingDestinationFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder,
}
impl EnableKinesisStreamingDestinationFluentBuilder {
    /// Creates a new `EnableKinesisStreamingDestination`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the EnableKinesisStreamingDestination as a reference.
    pub fn as_input(&self) -> &crate::operation::enable_kinesis_streaming_destination::builders::EnableKinesisStreamingDestinationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestinationOutput,
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
        crate::operation::enable_kinesis_streaming_destination::EnableKinesisStreamingDestination::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.stream_arn(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_stream_arn(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_stream_arn()
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_table_name()
}
}
