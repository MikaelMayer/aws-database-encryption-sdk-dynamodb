// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`ConnectCustomKeyStore`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl Into<Option<::std::string::String>>)`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<::std::string::String>)`](crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder::set_custom_key_store_id): (undocumented)<br>
    /// - On success, responds with [`ConnectCustomKeyStoreResponse`](crate::operation::connect_custom_key_store::ConnectCustomKeyStoreResponse) with field(s):

    /// - On failure, responds with [`SdkError<ConnectCustomKeyStoreError>`](crate::operation::connect_custom_key_store::ConnectCustomKeyStoreError)
    pub fn connect_custom_key_store(&self) -> crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder {
        crate::operation::connect_custom_key_store::builders::ConnectCustomKeyStoreFluentBuilder::new(self.clone())
    }
}
