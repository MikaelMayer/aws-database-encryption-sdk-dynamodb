// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::material_providers::client::Client {
    /// Constructs a fluent builder for the [`CreateKey`](crate::material_providers::operation::create_key::builders::CreateKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`branch_key_identifier(impl Into<Option<::std::string::String>>)`](crate::material_providers::operation::create_key::builders::CreateKeyFluentBuilder::branch_key_identifier) / [`set_branch_key_identifier(Option<::std::string::String>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_branch_key_identifier): (undocumented)<br>
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::material_providers::operation::create_key::builders::CreateKeyFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::create_key::builders::CreateKeyFluentBuilder::set_encryption_context): (undocumented)<br>
    /// - On success, responds with [`CreateKeyOutput`](crate::material_providers::operation::create_key::CreateKeyOutput) with field(s):
    ///   - [`branch_key_identifier(Option<::std::string::String>)`](crate::material_providers::operation::create_key::CreateKeyOutput::branch_key_identifier): (undocumented)
    /// - On failure, responds with [`SdkError<CreateKeyError>`](crate::material_providers::operation::create_key::CreateKeyError)
    pub fn create_key(&self) -> crate::material_providers::operation::create_key::builders::CreateKeyFluentBuilder {
        crate::material_providers::operation::create_key::builders::CreateKeyFluentBuilder::new(self.clone())
    }
}
