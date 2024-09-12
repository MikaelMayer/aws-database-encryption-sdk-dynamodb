// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DisableKeyRotation`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::disable_key_rotation::Unit) with field(s):

    /// - On failure, responds with [`SdkError<DisableKeyRotationError>`](crate::operation::disable_key_rotation::DisableKeyRotationError)
    pub fn disable_key_rotation(&self) -> crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder {
        crate::operation::disable_key_rotation::builders::DisableKeyRotationFluentBuilder::new(self.clone())
    }
}
