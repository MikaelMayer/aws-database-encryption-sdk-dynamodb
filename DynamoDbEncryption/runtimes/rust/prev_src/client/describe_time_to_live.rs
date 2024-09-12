// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DescribeTimeToLive`](crate::operation::describe_time_to_live::builders::DescribeTimeToLiveFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::describe_time_to_live::builders::DescribeTimeToLiveFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::describe_time_to_live::builders::DescribeTimeToLiveFluentBuilder::set_table_name): (undocumented)<br>
    /// - On success, responds with [`DescribeTimeToLiveOutput`](crate::operation::describe_time_to_live::DescribeTimeToLiveOutput) with field(s):
    ///   - [`time_to_live_description(Option<dynamodb::types::TimeToLiveDescription>)`](crate::operation::describe_time_to_live::DescribeTimeToLiveOutput::time_to_live_description): (undocumented)
    /// - On failure, responds with [`SdkError<DescribeTimeToLiveError>`](crate::operation::describe_time_to_live::DescribeTimeToLiveError)
    pub fn describe_time_to_live(&self) -> crate::operation::describe_time_to_live::builders::DescribeTimeToLiveFluentBuilder {
        crate::operation::describe_time_to_live::builders::DescribeTimeToLiveFluentBuilder::new(self.clone())
    }
}
