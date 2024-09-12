// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DeleteImportedKeyMaterial`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::delete_imported_key_material::Unit) with field(s):

    /// - On failure, responds with [`SdkError<DeleteImportedKeyMaterialError>`](crate::operation::delete_imported_key_material::DeleteImportedKeyMaterialError)
    pub fn delete_imported_key_material(&self) -> crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder {
        crate::operation::delete_imported_key_material::builders::DeleteImportedKeyMaterialFluentBuilder::new(self.clone())
    }
}
