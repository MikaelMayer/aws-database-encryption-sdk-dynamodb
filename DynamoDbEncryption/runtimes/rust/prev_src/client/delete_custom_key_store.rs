// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DeleteCustomKeyStore`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`custom_key_store_id(impl Into<Option<::std::string::String>>)`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::custom_key_store_id) / [`set_custom_key_store_id(Option<::std::string::String>)`](crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::set_custom_key_store_id): (undocumented)<br>
    /// - On success, responds with [`DeleteCustomKeyStoreResponse`](crate::operation::delete_custom_key_store::DeleteCustomKeyStoreResponse) with field(s):

    /// - On failure, responds with [`SdkError<DeleteCustomKeyStoreError>`](crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError)
    pub fn delete_custom_key_store(&self) -> crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder {
        crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreFluentBuilder::new(self.clone())
    }
}
