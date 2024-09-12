// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`UpdateKeyDescription`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl Into<Option<::std::string::String>>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::description) / [`set_description(Option<::std::string::String>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::set_description): (undocumented)<br>
    ///   - [`key_id(impl Into<Option<::std::string::String>>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::key_id) / [`set_key_id(Option<::std::string::String>)`](crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::set_key_id): (undocumented)<br>
    /// - On success, responds with [`Unit`](crate::operation::update_key_description::Unit) with field(s):

    /// - On failure, responds with [`SdkError<UpdateKeyDescriptionError>`](crate::operation::update_key_description::UpdateKeyDescriptionError)
    pub fn update_key_description(&self) -> crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder {
        crate::operation::update_key_description::builders::UpdateKeyDescriptionFluentBuilder::new(self.clone())
    }
}
