// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::client::Client {
    /// Constructs a fluent builder for the [`DecryptItem`](crate::operation::decrypt_item::builders::DecryptItemFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`encrypted_item(impl Into<Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>>)`](crate::operation::decrypt_item::builders::DecryptItemFluentBuilder::encrypted_item) / [`set_encrypted_item(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>)`](crate::operation::decrypt_item::builders::DecryptItemFluentBuilder::set_encrypted_item): (undocumented)<br>
    /// - On success, responds with [`DecryptItemOutput`](crate::operation::decrypt_item::DecryptItemOutput) with field(s):
    ///   - [`parsed_header(Option<item_encryptor::types::ParsedHeader>)`](crate::operation::decrypt_item::DecryptItemOutput::parsed_header): (undocumented)
    ///   - [`plaintext_item(Option<::std::collections::HashMap<::std::string::String, dynamodb::types::AttributeValue>>)`](crate::operation::decrypt_item::DecryptItemOutput::plaintext_item): (undocumented)
    /// - On failure, responds with [`SdkError<DecryptItemError>`](crate::operation::decrypt_item::DecryptItemError)
    pub fn decrypt_item(&self) -> crate::operation::decrypt_item::builders::DecryptItemFluentBuilder {
        crate::operation::decrypt_item::builders::DecryptItemFluentBuilder::new(self.clone())
    }
}
