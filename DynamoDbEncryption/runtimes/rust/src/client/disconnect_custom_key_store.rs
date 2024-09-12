// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DisconnectCustomKeyStore`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl Into<Option<::std::string::String>>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<::std::string::String>)`](crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::set_custom_key_store_id): (undocumented)<br>
    /// - On success, responds with [`DisconnectCustomKeyStoreResponse`](crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreResponse) with field(s):

    /// - On failure, responds with [`SdkError<DisconnectCustomKeyStoreError>`](crate::operation::disconnect_custom_key_store::DisconnectCustomKeyStoreError)
    pub fn disconnect_custom_key_store(&self) -> crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder {
        crate::operation::disconnect_custom_key_store::builders::DisconnectCustomKeyStoreFluentBuilder::new(self.clone())
    }
}
