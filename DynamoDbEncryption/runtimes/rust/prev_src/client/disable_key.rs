// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DisableKey`](crate::operation::disable_key::builders::DisableKeyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::disable_key::builders::DisableKeyFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::disable_key::builders::DisableKeyFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::disable_key::Unit) with field(s):

    /// - On failure, responds with [`SdkError<DisableKeyError>`](crate::operation::disable_key::DisableKeyError)
    pub fn disable_key(&self) -> crate::operation::disable_key::builders::DisableKeyFluentBuilder {
        crate::operation::disable_key::builders::DisableKeyFluentBuilder::new(self.clone())
    }
}
