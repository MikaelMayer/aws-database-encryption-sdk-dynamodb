// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`GetKeyRotationStatus`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`GetKeyRotationStatusResponse`](crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse) with field(s):
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse::key_id): (undocumented)
    ///   - [`key_rotation_enabled(Option<::std::primitive::bool>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse::key_rotation_enabled): (undocumented)
    ///   - [`next_rotation_date(Option<::aws_smithy_types::DateTime>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse::next_rotation_date): (undocumented)
    ///   - [`on_demand_rotation_start_date(Option<::aws_smithy_types::DateTime>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse::on_demand_rotation_start_date): (undocumented)
    ///   - [`rotation_period_in_days(Option<::std::primitive::i32>)`](crate::operation::get_key_rotation_status::GetKeyRotationStatusResponse::rotation_period_in_days): (undocumented)
    /// - On failure, responds with [`SdkError<GetKeyRotationStatusError>`](crate::operation::get_key_rotation_status::GetKeyRotationStatusError)
    pub fn get_key_rotation_status(&self) -> crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder {
        crate::operation::get_key_rotation_status::builders::GetKeyRotationStatusFluentBuilder::new(self.clone())
    }
}
