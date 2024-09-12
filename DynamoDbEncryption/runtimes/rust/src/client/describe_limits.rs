// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeLimits`](crate::operation::describe_limits::builders::DescribeLimitsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:

    /// - On success, responds with [`DescribeLimitsOutput`](crate::operation::describe_limits::DescribeLimitsOutput) with field(s):
    ///   - [`account_max_read_capacity_units(Option<::std::primitive::i64>)`](crate::operation::describe_limits::DescribeLimitsOutput::account_max_read_capacity_units): (undocumented)
    ///   - [`account_max_write_capacity_units(Option<::std::primitive::i64>)`](crate::operation::describe_limits::DescribeLimitsOutput::account_max_write_capacity_units): (undocumented)
    ///   - [`table_max_read_capacity_units(Option<::std::primitive::i64>)`](crate::operation::describe_limits::DescribeLimitsOutput::table_max_read_capacity_units): (undocumented)
    ///   - [`table_max_write_capacity_units(Option<::std::primitive::i64>)`](crate::operation::describe_limits::DescribeLimitsOutput::table_max_write_capacity_units): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeLimitsError>`](crate::operation::describe_limits::DescribeLimitsError)
    pub fn describe_limits(&self) -> crate::operation::describe_limits::builders::DescribeLimitsFluentBuilder {
        crate::operation::describe_limits::builders::DescribeLimitsFluentBuilder::new(self.clone())
    }
}
