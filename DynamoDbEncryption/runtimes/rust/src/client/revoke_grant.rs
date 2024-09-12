// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`RevokeGrant`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(impl Into<Option<::std::primitive::bool>>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::dry_run) / [`set_dry_run(Option<::std::primitive::bool>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::set_dry_run): (undocumented)<br>
    ///   - [`grant_id(impl Into<Option<::std::string::String>>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::grant_id) / [`set_grant_id(Option<::std::string::String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::set_grant_id): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::revoke_grant::Unit) with field(s):

    /// - On failure, responds with [`SdkError<RevokeGrantError>`](crate::operation::revoke_grant::RevokeGrantError)
    pub fn revoke_grant(&self) -> crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder {
        crate::operation::revoke_grant::builders::RevokeGrantFluentBuilder::new(self.clone())
    }
}
