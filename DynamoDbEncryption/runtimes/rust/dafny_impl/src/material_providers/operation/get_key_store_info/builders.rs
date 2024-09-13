// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::material_providers::operation::get_key_store_info::_get_key_store_info_output::GetKeyStoreInfoOutputBuilder;

pub use crate::material_providers::operation::get_key_store_info::_unit::UnitBuilder;

impl UnitBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::material_providers::client::Client,
    ) -> ::std::result::Result<
        crate::material_providers::operation::get_key_store_info::GetKeyStoreInfoOutput,
        crate::material_providers::types::error::Error,
    > {
        let mut fluent_builder = client.get_key_store_info();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetKeyStoreInfo`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetKeyStoreInfoFluentBuilder {
    client: crate::material_providers::client::Client,
    pub(crate) inner: crate::material_providers::operation::get_key_store_info::builders::UnitBuilder,
}
impl GetKeyStoreInfoFluentBuilder {
    /// Creates a new `GetKeyStoreInfo`.
    pub(crate) fn new(client: crate::material_providers::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetKeyStoreInfo as a reference.
    pub fn as_input(&self) -> &crate::material_providers::operation::get_key_store_info::builders::UnitBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::material_providers::operation::get_key_store_info::GetKeyStoreInfoOutput,
        crate::material_providers::types::error::Error,
    > {
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| crate::material_providers::types::error::Error::Opaque {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut dyn ::std::any::Any)
            })?;
        crate::material_providers::operation::get_key_store_info::GetKeyStoreInfo::send(&self.client, input).await
    }


}
