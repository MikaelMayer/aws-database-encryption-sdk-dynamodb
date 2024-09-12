// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ImportKeyMaterial`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`encrypted_key_material(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::encrypted_key_material) / [`set_encrypted_key_material(Option<::aws_smithy_types::Blob>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_encrypted_key_material): (undocumented)<br>
    ///   - [`expiration_model(impl Into<Option<kms::types::ExpirationModelType>>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::expiration_model) / [`set_expiration_model(Option<kms::types::ExpirationModelType>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_expiration_model): (undocumented)<br>
    ///   - [`import_token(impl Into<Option<::aws_smithy_types::Blob>>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::import_token) / [`set_import_token(Option<::aws_smithy_types::Blob>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_import_token): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`valid_to(impl Into<Option<::aws_smithy_types::DateTime>>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::valid_to) / [`set_valid_to(Option<::aws_smithy_types::DateTime>)`](crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::set_valid_to): (undocumented)<br>
    /// - On success, responds with [`ImportKeyMaterialResponse`](crate::operation::import_key_material::ImportKeyMaterialResponse) with field(s):

    /// - On failure, responds with [`SdkError<ImportKeyMaterialError>`](crate::operation::import_key_material::ImportKeyMaterialError)
    pub fn import_key_material(&self) -> crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder {
        crate::operation::import_key_material::builders::ImportKeyMaterialFluentBuilder::new(self.clone())
    }
}
