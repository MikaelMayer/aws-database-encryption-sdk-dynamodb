// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdatePrimaryRegion`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::set_key_id): (undocumented)<br>
    ///   - [`primary_region(impl Into<Option<::std::string::String>>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::primary_region) / [`set_primary_region(Option<::std::string::String>)`](crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::set_primary_region): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::update_primary_region::Unit) with field(s):

    /// - On failure, responds with [`SdkError<UpdatePrimaryRegionError>`](crate::operation::update_primary_region::UpdatePrimaryRegionError)
    pub fn update_primary_region(&self) -> crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder {
        crate::operation::update_primary_region::builders::UpdatePrimaryRegionFluentBuilder::new(self.clone())
    }
}
