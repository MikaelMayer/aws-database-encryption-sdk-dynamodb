// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`EnableKeyRotation`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`rotation_period_in_days(impl Into<Option<::std::primitive::i32>>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::rotation_period_in_days) / [`set_rotation_period_in_days(Option<::std::primitive::i32>)`](crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::set_rotation_period_in_days): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::enable_key_rotation::Unit) with field(s):

    /// - On failure, responds with [`SdkError<EnableKeyRotationError>`](crate::operation::enable_key_rotation::EnableKeyRotationError)
    pub fn enable_key_rotation(&self) -> crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder {
        crate::operation::enable_key_rotation::builders::EnableKeyRotationFluentBuilder::new(self.clone())
    }
}
