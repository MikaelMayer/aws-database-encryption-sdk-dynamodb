// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GetParametersForImport`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`wrapping_algorithm(impl Into<Option<kms::types::AlgorithmSpec>>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::wrapping_algorithm) / [`set_wrapping_algorithm(Option<kms::types::AlgorithmSpec>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::set_wrapping_algorithm): (undocumented)<br>
    ///   - [`wrapping_key_spec(impl Into<Option<kms::types::WrappingKeySpec>>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::wrapping_key_spec) / [`set_wrapping_key_spec(Option<kms::types::WrappingKeySpec>)`](crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::set_wrapping_key_spec): (undocumented)<br>
    /// - On success, responds with [`GetParametersForImportResponse`](crate::operation::get_parameters_for_import::GetParametersForImportResponse) with field(s):
    ///   - [`import_token(Option<::aws_smithy_types::Blob>)`](crate::operation::get_parameters_for_import::GetParametersForImportResponse::import_token): (undocumented)
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::get_parameters_for_import::GetParametersForImportResponse::key_id): (undocumented)
    ///   - [`parameters_valid_to(Option<::aws_smithy_types::DateTime>)`](crate::operation::get_parameters_for_import::GetParametersForImportResponse::parameters_valid_to): (undocumented)
    ///   - [`public_key(Option<::aws_smithy_types::Blob>)`](crate::operation::get_parameters_for_import::GetParametersForImportResponse::public_key): (undocumented)
    /// - On failure, responds with [`SdkError<GetParametersForImportError>`](crate::operation::get_parameters_for_import::GetParametersForImportError)
    pub fn get_parameters_for_import(&self) -> crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder {
        crate::operation::get_parameters_for_import::builders::GetParametersForImportFluentBuilder::new(self.clone())
    }
}
