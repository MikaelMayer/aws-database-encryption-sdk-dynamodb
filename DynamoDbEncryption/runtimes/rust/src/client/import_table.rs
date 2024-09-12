// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ImportTable`](crate::operation::import_table::builders::ImportTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<Option<::std::string::String>>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::client_token) / [`set_client_token(Option<::std::string::String>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::set_client_token): (undocumented)<br>
    ///   - [`input_compression_type(impl Into<Option<dynamodb::types::InputCompressionType>>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::input_compression_type) / [`set_input_compression_type(Option<dynamodb::types::InputCompressionType>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::set_input_compression_type): (undocumented)<br>
    ///   - [`input_format(impl Into<Option<dynamodb::types::InputFormat>>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::input_format) / [`set_input_format(Option<dynamodb::types::InputFormat>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::set_input_format): (undocumented)<br>
    ///   - [`input_format_options(impl Into<Option<dynamodb::types::InputFormatOptions>>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::input_format_options) / [`set_input_format_options(Option<dynamodb::types::InputFormatOptions>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::set_input_format_options): (undocumented)<br>
    ///   - [`s3_bucket_source(impl Into<Option<dynamodb::types::S3BucketSource>>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::s3_bucket_source) / [`set_s3_bucket_source(Option<dynamodb::types::S3BucketSource>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::set_s3_bucket_source): (undocumented)<br>
    ///   - [`table_creation_parameters(impl Into<Option<dynamodb::types::TableCreationParameters>>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::table_creation_parameters) / [`set_table_creation_parameters(Option<dynamodb::types::TableCreationParameters>)`](crate::operation::import_table::builders::ImportTableFluentBuilder::set_table_creation_parameters): (undocumented)<br>
    /// - On success, responds with [`ImportTableOutput`](crate::operation::import_table::ImportTableOutput) with field(s):
    ///   - [`import_table_description(Option<dynamodb::types::ImportTableDescription>)`](crate::operation::import_table::ImportTableOutput::import_table_description): (undocumented)
    /// - On failure, responds with [`SdkError<ImportTableError>`](crate::operation::import_table::ImportTableError)
    pub fn import_table(&self) -> crate::operation::import_table::builders::ImportTableFluentBuilder {
        crate::operation::import_table::builders::ImportTableFluentBuilder::new(self.clone())
    }
}
