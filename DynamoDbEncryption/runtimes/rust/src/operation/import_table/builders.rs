// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::import_table::_import_table_output::ImportTableOutputBuilder;

pub use crate::operation::import_table::_import_table_input::ImportTableInputBuilder;

impl ImportTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::import_table::ImportTableOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.import_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ImportTable`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ImportTableFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::import_table::builders::ImportTableInputBuilder,
}
impl ImportTableFluentBuilder {
    /// Creates a new `ImportTable`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the ImportTable as a reference.
    pub fn as_input(&self) -> &crate::operation::import_table::builders::ImportTableInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_table::ImportTableOutput,
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
        crate::operation::import_table::ImportTable::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.client_token(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_client_token(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_client_token()
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_compression_type(mut self, input: impl ::std::convert::Into<dynamodb::types::InputCompressionType>) -> Self {
    self.inner = self.inner.input_compression_type(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_input_compression_type(mut self, input: ::std::option::Option<dynamodb::types::InputCompressionType>) -> Self {
    self.inner = self.inner.set_input_compression_type(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_input_compression_type(&self) -> &::std::option::Option<dynamodb::types::InputCompressionType> {
    self.inner.get_input_compression_type()
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_format(mut self, input: impl ::std::convert::Into<dynamodb::types::InputFormat>) -> Self {
    self.inner = self.inner.input_format(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_input_format(mut self, input: ::std::option::Option<dynamodb::types::InputFormat>) -> Self {
    self.inner = self.inner.set_input_format(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_input_format(&self) -> &::std::option::Option<dynamodb::types::InputFormat> {
    self.inner.get_input_format()
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_format_options(mut self, input: impl ::std::convert::Into<dynamodb::types::InputFormatOptions>) -> Self {
    self.inner = self.inner.input_format_options(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_input_format_options(mut self, input: ::std::option::Option<dynamodb::types::InputFormatOptions>) -> Self {
    self.inner = self.inner.set_input_format_options(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_input_format_options(&self) -> &::std::option::Option<dynamodb::types::InputFormatOptions> {
    self.inner.get_input_format_options()
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket_source(mut self, input: impl ::std::convert::Into<dynamodb::types::S3BucketSource>) -> Self {
    self.inner = self.inner.s3_bucket_source(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_bucket_source(mut self, input: ::std::option::Option<dynamodb::types::S3BucketSource>) -> Self {
    self.inner = self.inner.set_s3_bucket_source(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_bucket_source(&self) -> &::std::option::Option<dynamodb::types::S3BucketSource> {
    self.inner.get_s3_bucket_source()
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_creation_parameters(mut self, input: impl ::std::convert::Into<dynamodb::types::TableCreationParameters>) -> Self {
    self.inner = self.inner.table_creation_parameters(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_creation_parameters(mut self, input: ::std::option::Option<dynamodb::types::TableCreationParameters>) -> Self {
    self.inner = self.inner.set_table_creation_parameters(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_creation_parameters(&self) -> &::std::option::Option<dynamodb::types::TableCreationParameters> {
    self.inner.get_table_creation_parameters()
}
}
