// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateGrant`](crate::operation::create_grant::builders::CreateGrantFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`constraints(impl Into<Option<kms::types::GrantConstraints>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::constraints) / [`set_constraints(Option<kms::types::GrantConstraints>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_constraints): (undocumented)<br>
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_tokens(impl Into<Option<::std::vec::Vec<::std::string::String>>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::grant_tokens) / [`set_grant_tokens(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_grant_tokens): (undocumented)<br>
    ///   - [`grantee_principal(impl Into<Option<::std::string::String>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::grantee_principal) / [`set_grantee_principal(Option<::std::string::String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_grantee_principal): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`name(impl Into<Option<::std::string::String>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::name) / [`set_name(Option<::std::string::String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_name): (undocumented)<br>
    ///   - [`operations(impl Into<Option<::std::vec::Vec<kms::types::GrantOperation>>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::operations) / [`set_operations(Option<::std::vec::Vec<kms::types::GrantOperation>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_operations): (undocumented)<br>
    ///   - [`retiring_principal(impl Into<Option<::std::string::String>>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::retiring_principal) / [`set_retiring_principal(Option<::std::string::String>)`](crate::operation::create_grant::builders::CreateGrantFluentBuilder::set_retiring_principal): (undocumented)<br>
    /// - On success, responds with [`CreateGrantResponse`](crate::operation::create_grant::CreateGrantResponse) with field(s):
    ///   - [`grant_id(Option<::std::string::String>)`](crate::operation::create_grant::CreateGrantResponse::grant_id): (undocumented)
    ///   - [`grant_token(Option<::std::string::String>)`](crate::operation::create_grant::CreateGrantResponse::grant_token): (undocumented)
    /// - On failure, responds with [`SdkError<CreateGrantError>`](crate::operation::create_grant::CreateGrantError)
    pub fn create_grant(&self) -> crate::operation::create_grant::builders::CreateGrantFluentBuilder {
        crate::operation::create_grant::builders::CreateGrantFluentBuilder::new(self.clone())
    }
}
