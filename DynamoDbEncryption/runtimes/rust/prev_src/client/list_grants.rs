// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListGrants`](crate::operation::list_grants::builders::ListGrantsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`grant_id(impl Into<Option<::std::string::String>>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::grant_id) / [`set_grant_id(Option<::std::string::String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_grant_id): (undocumented)<br>
    ///   - [`grantee_principal(impl Into<Option<::std::string::String>>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::grantee_principal) / [`set_grantee_principal(Option<::std::string::String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_grantee_principal): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`marker(impl Into<Option<::std::string::String>>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::marker) / [`set_marker(Option<::std::string::String>)`](crate::operation::list_grants::builders::ListGrantsFluentBuilder::set_marker): (undocumented)<br>
    /// - On success, responds with [`ListGrantsResponse`](crate::operation::list_grants::ListGrantsResponse) with field(s):
    ///   - [`grants(Option<::std::vec::Vec<kms::types::GrantListEntry>>)`](crate::operation::list_grants::ListGrantsResponse::grants): (undocumented)
    ///   - [`next_marker(Option<::std::string::String>)`](crate::operation::list_grants::ListGrantsResponse::next_marker): (undocumented)
    ///   - [`truncated(Option<::std::primitive::bool>)`](crate::operation::list_grants::ListGrantsResponse::truncated): (undocumented)
    /// - On failure, responds with [`SdkError<ListGrantsError>`](crate::operation::list_grants::ListGrantsError)
    pub fn list_grants(&self) -> crate::operation::list_grants::builders::ListGrantsFluentBuilder {
        crate::operation::list_grants::builders::ListGrantsFluentBuilder::new(self.clone())
    }
}
