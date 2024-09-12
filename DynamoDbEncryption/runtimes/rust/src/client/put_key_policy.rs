// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`PutKeyPolicy`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bypass_policy_lockout_safety_check(impl Into<Option<::std::primitive::bool>>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::bypass_policy_lockout_safety_check) / [`set_bypass_policy_lockout_safety_check(Option<::std::primitive::bool>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_bypass_policy_lockout_safety_check): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`policy(impl Into<Option<::std::string::String>>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::policy) / [`set_policy(Option<::std::string::String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_policy): (undocumented)<br>
    ///   - [`policy_name(impl Into<Option<::std::string::String>>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::policy_name) / [`set_policy_name(Option<::std::string::String>)`](crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::set_policy_name): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::put_key_policy::Unit) with field(s):

    /// - On failure, responds with [`SdkError<PutKeyPolicyError>`](crate::operation::put_key_policy::PutKeyPolicyError)
    pub fn put_key_policy(&self) -> crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder {
        crate::operation::put_key_policy::builders::PutKeyPolicyFluentBuilder::new(self.clone())
    }
}
