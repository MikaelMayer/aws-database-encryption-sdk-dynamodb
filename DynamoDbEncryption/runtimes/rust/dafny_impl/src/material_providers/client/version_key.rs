// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::material_providers::client::Client {
    /// Constructs a fluent builder for the [`VersionKey`](crate::material_providers::operation::version_key::builders::VersionKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`branch_key_identifier(impl Into<Option<::std::string::String>>)`](crate::material_providers::operation::version_key::builders::VersionKeyFluentBuilder::branch_key_identifier) / [`set_branch_key_identifier(Option<::std::string::String>)`](crate::operation::version_key::builders::VersionKeyFluentBuilder::set_branch_key_identifier): (undocumented)<br>
    /// - On success, responds with [`VersionKeyOutput`](crate::material_providers::operation::version_key::VersionKeyOutput) with field(s):

    /// - On failure, responds with [`SdkError<VersionKeyError>`](crate::material_providers::operation::version_key::VersionKeyError)
    pub fn version_key(&self) -> crate::material_providers::operation::version_key::builders::VersionKeyFluentBuilder {
        crate::material_providers::operation::version_key::builders::VersionKeyFluentBuilder::new(self.clone())
    }
}
