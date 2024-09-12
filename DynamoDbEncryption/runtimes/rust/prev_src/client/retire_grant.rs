// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`RetireGrant`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_id(impl Into<Option<::std::string::String>>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::grant_id) / [`set_grant_id(Option<::std::string::String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_grant_id): (undocumented)<br>
    ///   - [`grant_token(impl Into<Option<::std::string::String>>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::grant_token) / [`set_grant_token(Option<::std::string::String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_grant_token): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::retire_grant::builders::RetireGrantFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::retire_grant::Unit) with field(s):

    /// - On failure, responds with [`SdkError<RetireGrantError>`](crate::operation::retire_grant::RetireGrantError)
    pub fn retire_grant(&self) -> crate::operation::retire_grant::builders::RetireGrantFluentBuilder {
        crate::operation::retire_grant::builders::RetireGrantFluentBuilder::new(self.clone())
    }
}
