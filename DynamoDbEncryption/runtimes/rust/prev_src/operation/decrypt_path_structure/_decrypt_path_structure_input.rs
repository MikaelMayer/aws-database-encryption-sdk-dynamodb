// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DecryptPathStructureInput {
    #[allow(missing_docs)] // documentation missing in model
pub cmm: ::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
#[allow(missing_docs)] // documentation missing in model
pub encrypted_structure: ::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>>,
#[allow(missing_docs)] // documentation missing in model
pub encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
#[allow(missing_docs)] // documentation missing in model
pub table_name: ::std::option::Option<::std::string::String>,
}
impl DecryptPathStructureInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn cmm(&self) -> &::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef> {
    &self.cmm
}
#[allow(missing_docs)] // documentation missing in model
pub fn encrypted_structure(&self) -> &::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>> {
    &self.encrypted_structure
}
#[allow(missing_docs)] // documentation missing in model
pub fn encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
}
impl DecryptPathStructureInput {
    /// Creates a new builder-style object to manufacture [`DecryptPathStructureInput`](crate::operation::decrypt_path_structure::builders::DecryptPathStructureInput).
    pub fn builder() -> crate::operation::decrypt_path_structure::builders::DecryptPathStructureInputBuilder {
        crate::operation::decrypt_path_structure::builders::DecryptPathStructureInputBuilder::default()
    }
}

/// A builder for [`DecryptPathStructureInput`](crate::operation::operation::DecryptPathStructureInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct DecryptPathStructureInputBuilder {
    pub(crate) cmm: ::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>,
pub(crate) encrypted_structure: ::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>>,
pub(crate) encryption_context: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
pub(crate) table_name: ::std::option::Option<::std::string::String>,
}
impl DecryptPathStructureInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn cmm(mut self, input: impl ::std::convert::Into<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.cmm = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_cmm(mut self, input: ::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef>) -> Self {
    self.cmm = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_cmm(&self) -> &::std::option::Option<material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef> {
    &self.cmm
}
#[allow(missing_docs)] // documentation missing in model
pub fn encrypted_structure(mut self, input: impl ::std::convert::Into<::std::vec::Vec<structured_encryption::types::AuthItem>>) -> Self {
    self.encrypted_structure = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encrypted_structure(mut self, input: ::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>>) -> Self {
    self.encrypted_structure = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encrypted_structure(&self) -> &::std::option::Option<::std::vec::Vec<structured_encryption::types::AuthItem>> {
    &self.encrypted_structure
}
#[allow(missing_docs)] // documentation missing in model
pub fn encryption_context(mut self, input: impl ::std::convert::Into<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.encryption_context = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_encryption_context(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
    self.encryption_context = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_encryption_context(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
    &self.encryption_context
}
#[allow(missing_docs)] // documentation missing in model
pub fn table_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.table_name = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_table_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.table_name = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_table_name(&self) -> &::std::option::Option<::std::string::String> {
    &self.table_name
}
    /// Consumes the builder and constructs a [`DecryptPathStructureInput`](crate::operation::operation::DecryptPathStructureInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::decrypt_path_structure::DecryptPathStructureInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::decrypt_path_structure::DecryptPathStructureInput {
            cmm: self.cmm,
encrypted_structure: self.encrypted_structure,
encryption_context: self.encryption_context,
table_name: self.table_name,
        })
    }
}