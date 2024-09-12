// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`EnableKey`](crate::operation::enable_key::builders::EnableKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::enable_key::builders::EnableKeyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::enable_key::builders::EnableKeyFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::enable_key::Unit) with field(s):

    /// - On failure, responds with [`SdkError<EnableKeyError>`](crate::operation::enable_key::EnableKeyError)
    pub fn enable_key(&self) -> crate::operation::enable_key::builders::EnableKeyFluentBuilder {
        crate::operation::enable_key::builders::EnableKeyFluentBuilder::new(self.clone())
    }
}
