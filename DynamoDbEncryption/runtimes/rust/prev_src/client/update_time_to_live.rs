// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdateTimeToLive`](crate::operation::update_time_to_live::builders::UpdateTimeToLiveFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_name(impl Into<Option<::std::string::String>>)`](crate::operation::update_time_to_live::builders::UpdateTimeToLiveFluentBuilder::table_name) / [`set_table_name(Option<::std::string::String>)`](crate::operation::update_time_to_live::builders::UpdateTimeToLiveFluentBuilder::set_table_name): (undocumented)<br>
    ///   - [`time_to_live_specification(impl Into<Option<dynamodb::types::TimeToLiveSpecification>>)`](crate::operation::update_time_to_live::builders::UpdateTimeToLiveFluentBuilder::time_to_live_specification) / [`set_time_to_live_specification(Option<dynamodb::types::TimeToLiveSpecification>)`](crate::operation::update_time_to_live::builders::UpdateTimeToLiveFluentBuilder::set_time_to_live_specification): (undocumented)<br>
    /// - On success, responds with [`UpdateTimeToLiveOutput`](crate::operation::update_time_to_live::UpdateTimeToLiveOutput) with field(s):
    ///   - [`time_to_live_specification(Option<dynamodb::types::TimeToLiveSpecification>)`](crate::operation::update_time_to_live::UpdateTimeToLiveOutput::time_to_live_specification): (undocumented)
    /// - On failure, responds with [`SdkError<UpdateTimeToLiveError>`](crate::operation::update_time_to_live::UpdateTimeToLiveError)
    pub fn update_time_to_live(&self) -> crate::operation::update_time_to_live::builders::UpdateTimeToLiveFluentBuilder {
        crate::operation::update_time_to_live::builders::UpdateTimeToLiveFluentBuilder::new(self.clone())
    }
}
