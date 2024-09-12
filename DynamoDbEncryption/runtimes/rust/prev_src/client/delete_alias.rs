// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DeleteAlias`](crate::operation::delete_alias::builders::DeleteAliasFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`alias_name(impl Into<Option<::std::string::String>>)`](crate::operation::delete_alias::builders::DeleteAliasFluentBuilder::alias_name) / [`set_alias_name(Option<::std::string::String>)`](crate::operation::delete_alias::builders::DeleteAliasFluentBuilder::set_alias_name): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::delete_alias::Unit) with field(s):

    /// - On failure, responds with [`SdkError<DeleteAliasError>`](crate::operation::delete_alias::DeleteAliasError)
    pub fn delete_alias(&self) -> crate::operation::delete_alias::builders::DeleteAliasFluentBuilder {
        crate::operation::delete_alias::builders::DeleteAliasFluentBuilder::new(self.clone())
    }
}
