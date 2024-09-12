// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ListAliases`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`limit(impl Into<Option<::std::primitive::i32>>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::limit) / [`set_limit(Option<::std::primitive::i32>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_limit): (undocumented)<br>
    ///   - [`marker(impl Into<Option<::std::string::String>>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::marker) / [`set_marker(Option<::std::string::String>)`](crate::operation::list_aliases::builders::ListAliasesFluentBuilder::set_marker): (undocumented)<br>
    /// - On success, responds with [`ListAliasesResponse`](crate::operation::list_aliases::ListAliasesResponse) with field(s):
    ///   - [`aliases(Option<::std::vec::Vec<kms::types::AliasListEntry>>)`](crate::operation::list_aliases::ListAliasesResponse::aliases): (undocumented)
    ///   - [`next_marker(Option<::std::string::String>)`](crate::operation::list_aliases::ListAliasesResponse::next_marker): (undocumented)
    ///   - [`truncated(Option<::std::primitive::bool>)`](crate::operation::list_aliases::ListAliasesResponse::truncated): (undocumented)
    /// - On failure, responds with [`SdkError<ListAliasesError>`](crate::operation::list_aliases::ListAliasesError)
    pub fn list_aliases(&self) -> crate::operation::list_aliases::builders::ListAliasesFluentBuilder {
        crate::operation::list_aliases::builders::ListAliasesFluentBuilder::new(self.clone())
    }
}
