// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`RotateKeyOnDemand`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`RotateKeyOnDemandResponse`](crate::operation::rotate_key_on_demand::RotateKeyOnDemandResponse) with field(s):
    ///   - [`key_id(Option<::std::string::String>)`](crate::operation::rotate_key_on_demand::RotateKeyOnDemandResponse::key_id): (undocumented)
    /// - On failure, responds with [`SdkError<RotateKeyOnDemandError>`](crate::operation::rotate_key_on_demand::RotateKeyOnDemandError)
    pub fn rotate_key_on_demand(&self) -> crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder {
        crate::operation::rotate_key_on_demand::builders::RotateKeyOnDemandFluentBuilder::new(self.clone())
    }
}
