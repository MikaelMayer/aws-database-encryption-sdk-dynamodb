// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::operation::update_continuous_backups::_update_continuous_backups_output::UpdateContinuousBackupsOutputBuilder;

pub use crate::operation::update_continuous_backups::_update_continuous_backups_input::UpdateContinuousBackupsInputBuilder;

impl UpdateContinuousBackupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::client::Client,
    ) -> ::std::result::Result<
        crate::operation::update_continuous_backups::UpdateContinuousBackupsOutput,
        crate::types::error::Error,
    > {
        let mut fluent_builder = client.update_continuous_backups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateContinuousBackups`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateContinuousBackupsFluentBuilder {
    client: crate::client::Client,
    pub(crate) inner: crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsInputBuilder,
}
impl UpdateContinuousBackupsFluentBuilder {
    /// Creates a new `UpdateContinuousBackups`.
    pub(crate) fn new(client: crate::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the UpdateContinuousBackups as a reference.
    pub fn as_input(&self) -> &crate::operation::update_continuous_backups::builders::UpdateContinuousBackupsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_continuous_backups::UpdateContinuousBackupsOutput,
        crate::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::operation::update_continuous_backups::UpdateContinuousBackups::send(&self.client, input).await
    }

    #[allow(missing_docs)] // documentation missing in model
pub fn point_in_time_recovery_specification(mut self, input: impl ::std::convert::Into<dynamodb::types::PointInTimeRecoverySpecification>) -> Self {
    self.inner = self.inner.point_in_time_recovery_specification(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_point_in_time_recovery_specification(mut self, input: ::std::option::Option<dynamodb::types::PointInTimeRecoverySpecification>) -> Self {
    self.inner = self.inner.set_point_in_time_recovery_specification(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_point_in_time_recovery_specification(&self) -> &::std::option::Option<dynamodb::types::PointInTimeRecoverySpecification> {
    self.inner.get_point_in_time_recovery_specification()
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.inner = self.inner.table_name(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.inner = self.inner.set_table_name(input);
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    self.inner.get_table_name()
}
}
