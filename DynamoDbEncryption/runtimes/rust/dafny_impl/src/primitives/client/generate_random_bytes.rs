// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::primitives::client::Client {
    /// Constructs a fluent builder for the [`GenerateRandomBytes`](crate::primitives::operation::generate_random_bytes::builders::GenerateRandomBytesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`length(impl Into<Option<::std::primitive::i32>>)`](crate::primitives::operation::generate_random_bytes::builders::GenerateRandomBytesFluentBuilder::length) / [`set_length(Option<::std::primitive::i32>)`](crate::operation::generate_random_bytes::builders::GenerateRandomBytesFluentBuilder::set_length): (undocumented)<br>
    /// - On success, responds with [`GenerateRandomBytesOutput`](crate::primitives::operation::generate_random_bytes::GenerateRandomBytesOutput) with field(s):
    ///   - [`data(Option<::aws_smithy_types::Blob>)`](crate::primitives::operation::generate_random_bytes::GenerateRandomBytesOutput::data): (undocumented)
    /// - On failure, responds with [`SdkError<GenerateRandomBytesError>`](crate::primitives::operation::generate_random_bytes::GenerateRandomBytesError)
    pub fn generate_random_bytes(&self) -> crate::primitives::operation::generate_random_bytes::builders::GenerateRandomBytesFluentBuilder {
        crate::primitives::operation::generate_random_bytes::builders::GenerateRandomBytesFluentBuilder::new(self.clone())
    }
}
