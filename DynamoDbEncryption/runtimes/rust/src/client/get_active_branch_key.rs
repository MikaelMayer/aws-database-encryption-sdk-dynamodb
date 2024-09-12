// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GetActiveBranchKey`](crate::operation::get_active_branch_key::builders::GetActiveBranchKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`branch_key_identifier(impl Into<Option<::std::string::String>>)`](crate::operation::get_active_branch_key::builders::GetActiveBranchKeyFluentBuilder::branch_key_identifier) / [`set_branch_key_identifier(Option<::std::string::String>)`](crate::operation::get_active_branch_key::builders::GetActiveBranchKeyFluentBuilder::set_branch_key_identifier): (undocumented)<br>
    /// - On success, responds with [`GetActiveBranchKeyOutput`](crate::operation::get_active_branch_key::GetActiveBranchKeyOutput) with field(s):
    ///   - [`branch_key_materials(Option<key_store::types::BranchKeyMaterials>)`](crate::operation::get_active_branch_key::GetActiveBranchKeyOutput::branch_key_materials): (undocumented)
    /// - On failure, responds with [`SdkError<GetActiveBranchKeyError>`](crate::operation::get_active_branch_key::GetActiveBranchKeyError)
    pub fn get_active_branch_key(&self) -> crate::operation::get_active_branch_key::builders::GetActiveBranchKeyFluentBuilder {
        crate::operation::get_active_branch_key::builders::GetActiveBranchKeyFluentBuilder::new(self.clone())
    }
}
