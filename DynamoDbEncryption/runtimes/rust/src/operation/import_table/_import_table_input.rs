// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub client_token: ::std::option::Option<::std::string::String>,
#[allow(missing_docs)] // documentation missing in model
pub input_compression_type: ::std::option::Option<dynamodb::types::InputCompressionType>,
#[allow(missing_docs)] // documentation missing in model
pub input_format: ::std::option::Option<dynamodb::types::InputFormat>,
#[allow(missing_docs)] // documentation missing in model
pub input_format_options: ::std::option::Option<dynamodb::types::InputFormatOptions>,
#[allow(missing_docs)] // documentation missing in model
pub s3_bucket_source: ::std::option::Option<dynamodb::types::S3BucketSource>,
#[allow(missing_docs)] // documentation missing in model
pub table_creation_parameters: ::std::option::Option<dynamodb::types::TableCreationParameters>,
}
impl ImportTableInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn client_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.client_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_compression_type(&self) -> &::std::option::Option<dynamodb::types::InputCompressionType> {
    &self.input_compression_type
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_format(&self) -> &::std::option::Option<dynamodb::types::InputFormat> {
    &self.input_format
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_format_options(&self) -> &::std::option::Option<dynamodb::types::InputFormatOptions> {
    &self.input_format_options
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket_source(&self) -> &::std::option::Option<dynamodb::types::S3BucketSource> {
    &self.s3_bucket_source
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_creation_parameters(&self) -> &::std::option::Option<dynamodb::types::TableCreationParameters> {
    &self.table_creation_parameters
}
}
impl ImportTableInput {
    /// Creates a new builder-style object to manufacture [`ImportTableInput`](crate::operation::import_table::builders::ImportTableInput).
    pub fn builder() -> crate::operation::import_table::builders::ImportTableInputBuilder {
        crate::operation::import_table::builders::ImportTableInputBuilder::default()
    }
}

/// A builder for [`ImportTableInput`](crate::operation::operation::ImportTableInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct ImportTableInputBuilder {
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
pub(crate) input_compression_type: ::std::option::Option<dynamodb::types::InputCompressionType>,
pub(crate) input_format: ::std::option::Option<dynamodb::types::InputFormat>,
pub(crate) input_format_options: ::std::option::Option<dynamodb::types::InputFormatOptions>,
pub(crate) s3_bucket_source: ::std::option::Option<dynamodb::types::S3BucketSource>,
pub(crate) table_creation_parameters: ::std::option::Option<dynamodb::types::TableCreationParameters>,
}
impl ImportTableInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.client_token = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.client_token = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
    &self.client_token
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_compression_type(mut self, input: impl ::std::convert::Into<dynamodb::types::InputCompressionType>) -> Self {
    self.input_compression_type = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_input_compression_type(mut self, input: ::std::option::Option<dynamodb::types::InputCompressionType>) -> Self {
    self.input_compression_type = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_input_compression_type(&self) -> &::std::option::Option<dynamodb::types::InputCompressionType> {
    &self.input_compression_type
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_format(mut self, input: impl ::std::convert::Into<dynamodb::types::InputFormat>) -> Self {
    self.input_format = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_input_format(mut self, input: ::std::option::Option<dynamodb::types::InputFormat>) -> Self {
    self.input_format = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_input_format(&self) -> &::std::option::Option<dynamodb::types::InputFormat> {
    &self.input_format
}
#[allow(missing_docs)] // documentation missing in model
pub fn input_format_options(mut self, input: impl ::std::convert::Into<dynamodb::types::InputFormatOptions>) -> Self {
    self.input_format_options = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_input_format_options(mut self, input: ::std::option::Option<dynamodb::types::InputFormatOptions>) -> Self {
    self.input_format_options = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_input_format_options(&self) -> &::std::option::Option<dynamodb::types::InputFormatOptions> {
    &self.input_format_options
}
#[allow(missing_docs)] // documentation missing in model
pub fn s3_bucket_source(mut self, input: impl ::std::convert::Into<dynamodb::types::S3BucketSource>) -> Self {
    self.s3_bucket_source = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_s3_bucket_source(mut self, input: ::std::option::Option<dynamodb::types::S3BucketSource>) -> Self {
    self.s3_bucket_source = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_s3_bucket_source(&self) -> &::std::option::Option<dynamodb::types::S3BucketSource> {
    &self.s3_bucket_source
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_creation_parameters(mut self, input: impl ::std::convert::Into<dynamodb::types::TableCreationParameters>) -> Self {
    self.table_creation_parameters = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_creation_parameters(mut self, input: ::std::option::Option<dynamodb::types::TableCreationParameters>) -> Self {
    self.table_creation_parameters = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_creation_parameters(&self) -> &::std::option::Option<dynamodb::types::TableCreationParameters> {
    &self.table_creation_parameters
}
    /// Consumes the builder and constructs a [`ImportTableInput`](crate::operation::operation::ImportTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::import_table::ImportTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::import_table::ImportTableInput {
            client_token: self.client_token,
input_compression_type: self.input_compression_type,
input_format: self.input_format,
input_format_options: self.input_format_options,
s3_bucket_source: self.s3_bucket_source,
table_creation_parameters: self.table_creation_parameters,
        })
    }
}
