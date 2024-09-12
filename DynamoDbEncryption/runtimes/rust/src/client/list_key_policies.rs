// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListKeyPolicies`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`marker(impl Into<Option<::std::string::String>>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::marker) / [`set_marker(Option<::std::string::String>)`](crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::set_marker): (undocumented)<br>
    /// - On success, responds with [`ListKeyPoliciesResponse`](crate::operation::list_key_policies::ListKeyPoliciesResponse) with field(s):
    ///   - [`next_marker(Option<::std::string::String>)`](crate::operation::list_key_policies::ListKeyPoliciesResponse::next_marker): (undocumented)
    ///   - [`policy_names(Option<::std::vec::Vec<::std::string::String>>)`](crate::operation::list_key_policies::ListKeyPoliciesResponse::policy_names): (undocumented)
    ///   - [`truncated(Option<::std::primitive::bool>)`](crate::operation::list_key_policies::ListKeyPoliciesResponse::truncated): (undocumented)
    /// - On failure, responds with [`SdkError<ListKeyPoliciesError>`](crate::operation::list_key_policies::ListKeyPoliciesError)
    pub fn list_key_policies(&self) -> crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder {
        crate::operation::list_key_policies::builders::ListKeyPoliciesFluentBuilder::new(self.clone())
    }
}
