// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DecryptStructureOutput {
    #[allow(missing_docs)] // documentation missing in model
pub crypto_schema: ::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>>,
#[allow(missing_docs)] // documentation missing in model
pub parsed_header: ::std::option::Option<structured_encryption::types::ParsedHeader>,
#[allow(missing_docs)] // documentation missing in model
pub plaintext_structure: ::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>,
}
impl DecryptStructureOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn crypto_schema(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>> {
    &self.crypto_schema
}
#[allow(missing_docs)] // documentation missing in model
pub fn parsed_header(&self) -> &::std::option::Option<structured_encryption::types::ParsedHeader> {
    &self.parsed_header
}
#[allow(missing_docs)] // documentation missing in model
pub fn plaintext_structure(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>> {
    &self.plaintext_structure
}
}
impl DecryptStructureOutput {
    /// Creates a new builder-style object to manufacture [`DecryptStructureOutput`](crate::operation::decrypt_structure::builders::DecryptStructureOutput).
    pub fn builder() -> crate::operation::decrypt_structure::builders::DecryptStructureOutputBuilder {
        crate::operation::decrypt_structure::builders::DecryptStructureOutputBuilder::default()
    }
}

/// A builder for [`DecryptStructureOutput`](crate::operation::operation::DecryptStructureOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptStructureOutputBuilder {
    pub(crate) crypto_schema: ::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>>,
pub(crate) parsed_header: ::std::option::Option<structured_encryption::types::ParsedHeader>,
pub(crate) plaintext_structure: ::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>,
}
impl DecryptStructureOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn crypto_schema(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>>) -> Self {
    self.crypto_schema = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_crypto_schema(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>>) -> Self {
    self.crypto_schema = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_crypto_schema(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::CryptoAction>> {
    &self.crypto_schema
}
#[allow(missing_docs)] // documentation missing in model
pub fn parsed_header(mut self, input: impl ::std::convert::Into<structured_encryption::types::ParsedHeader>) -> Self {
    self.parsed_header = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_parsed_header(mut self, input: ::std::option::Option<structured_encryption::types::ParsedHeader>) -> Self {
    self.parsed_header = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_parsed_header(&self) -> &::std::option::Option<structured_encryption::types::ParsedHeader> {
    &self.parsed_header
}
#[allow(missing_docs)] // documentation missing in model
pub fn plaintext_structure(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>) -> Self {
    self.plaintext_structure = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_plaintext_structure(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>>) -> Self {
    self.plaintext_structure = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_plaintext_structure(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, structured_encryption::types::StructuredDataTerminal>> {
    &self.plaintext_structure
}
    /// Consumes the builder and constructs a [`DecryptStructureOutput`](crate::operation::operation::DecryptStructureOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::decrypt_structure::DecryptStructureOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::decrypt_structure::DecryptStructureOutput {
            crypto_schema: self.crypto_schema,
parsed_header: self.parsed_header,
plaintext_structure: self.plaintext_structure,
        })
    }
}