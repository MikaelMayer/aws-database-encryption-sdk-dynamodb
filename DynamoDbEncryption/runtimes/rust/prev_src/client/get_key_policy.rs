// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GetKeyPolicy`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`policy_name(impl Into<Option<::std::string::String>>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<::std::string::String>)`](crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::set_policy_name): (undocumented)<br>
    /// - On success, responds with [`GetKeyPolicyResponse`](crate::operation::get_key_policy::GetKeyPolicyResponse) with field(s):
    ///   - [`policy(Option<::std::string::String>)`](crate::operation::get_key_policy::GetKeyPolicyResponse::policy): (undocumented)
    ///   - [`policy_name(Option<::std::string::String>)`](crate::operation::get_key_policy::GetKeyPolicyResponse::policy_name): (undocumented)
    /// - On failure, responds with [`SdkError<GetKeyPolicyError>`](crate::operation::get_key_policy::GetKeyPolicyError)
    pub fn get_key_policy(&self) -> crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder {
        crate::operation::get_key_policy::builders::GetKeyPolicyFluentBuilder::new(self.clone())
    }
}
