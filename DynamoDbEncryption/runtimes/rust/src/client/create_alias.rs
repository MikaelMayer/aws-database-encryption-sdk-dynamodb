// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`CreateAlias`](crate::operation::create_alias::builders::CreateAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alias_name(impl Into<Option<::std::string::String>>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::alias_name) / [`set_alias_name(Option<::std::string::String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_alias_name): (undocumented)<br>
    ///   - [`target_key_id(impl Into<Option<::std::string::String>>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::target_key_id) / [`set_target_key_id(Option<::std::string::String>)`](crate::operation::create_alias::builders::CreateAliasFluentBuilder::set_target_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::create_alias::Unit) with field(s):

    /// - On failure, responds with [`SdkError<CreateAliasError>`](crate::operation::create_alias::CreateAliasError)
    pub fn create_alias(&self) -> crate::operation::create_alias::builders::CreateAliasFluentBuilder {
        crate::operation::create_alias::builders::CreateAliasFluentBuilder::new(self.clone())
    }
}
