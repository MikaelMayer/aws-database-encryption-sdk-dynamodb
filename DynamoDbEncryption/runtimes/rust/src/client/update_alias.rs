// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdateAlias`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alias_name(impl Into<Option<::std::string::String>>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::alias_name) / [`set_alias_name(Option<::std::string::String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_alias_name): (undocumented)<br>
    ///   - [`target_key_id(impl Into<Option<::std::string::String>>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::target_key_id) / [`set_target_key_id(Option<::std::string::String>)`](crate::operation::update_alias::builders::UpdateAliasFluentBuilder::set_target_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::update_alias::Unit) with field(s):

    /// - On failure, responds with [`SdkError<UpdateAliasError>`](crate::operation::update_alias::UpdateAliasError)
    pub fn update_alias(&self) -> crate::operation::update_alias::builders::UpdateAliasFluentBuilder {
        crate::operation::update_alias::builders::UpdateAliasFluentBuilder::new(self.clone())
    }
}
