// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::material_providers::client::Client {
    /// Constructs a fluent builder for the [`GetBranchKeyVersion`](crate::material_providers::operation::get_branch_key_version::builders::GetBranchKeyVersionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`branch_key_identifier(impl Into<Option<::std::string::String>>)`](crate::material_providers::operation::get_branch_key_version::builders::GetBranchKeyVersionFluentBuilder::branch_key_identifier) / [`set_branch_key_identifier(Option<::std::string::String>)`](crate::operation::get_branch_key_version::builders::GetBranchKeyVersionFluentBuilder::set_branch_key_identifier): (undocumented)<br>
    ///   - [`branch_key_version(impl Into<Option<::std::string::String>>)`](crate::material_providers::operation::get_branch_key_version::builders::GetBranchKeyVersionFluentBuilder::branch_key_version) / [`set_branch_key_version(Option<::std::string::String>)`](crate::operation::get_branch_key_version::builders::GetBranchKeyVersionFluentBuilder::set_branch_key_version): (undocumented)<br>
    /// - On success, responds with [`GetBranchKeyVersionOutput`](crate::material_providers::operation::get_branch_key_version::GetBranchKeyVersionOutput) with field(s):
    ///   - [`branch_key_materials(Option<crate::material_providers::types::BranchKeyMaterials>)`](crate::material_providers::operation::get_branch_key_version::GetBranchKeyVersionOutput::branch_key_materials): (undocumented)
    /// - On failure, responds with [`SdkError<GetBranchKeyVersionError>`](crate::material_providers::operation::get_branch_key_version::GetBranchKeyVersionError)
    pub fn get_branch_key_version(&self) -> crate::material_providers::operation::get_branch_key_version::builders::GetBranchKeyVersionFluentBuilder {
        crate::material_providers::operation::get_branch_key_version::builders::GetBranchKeyVersionFluentBuilder::new(self.clone())
    }
}
