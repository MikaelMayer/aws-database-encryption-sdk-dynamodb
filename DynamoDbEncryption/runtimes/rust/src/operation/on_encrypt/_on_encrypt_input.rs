// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OnEncryptInput {
    #[allow(missing_docs)] // documentation missing in model
pub materials: ::std::option::Option<material_providers::types::EncryptionMaterials>,
}
impl OnEncryptInput {
    #[allow(missing_docs)] // documentation missing in model
pub fn materials(&self) -> &::std::option::Option<material_providers::types::EncryptionMaterials> {
    &self.materials
}
}
impl OnEncryptInput {
    /// Creates a new builder-style object to manufacture [`OnEncryptInput`](crate::operation::on_encrypt::builders::OnEncryptInput).
    pub fn builder() -> crate::operation::on_encrypt::builders::OnEncryptInputBuilder {
        crate::operation::on_encrypt::builders::OnEncryptInputBuilder::default()
    }
}

/// A builder for [`OnEncryptInput`](crate::operation::operation::OnEncryptInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct OnEncryptInputBuilder {
    pub(crate) materials: ::std::option::Option<material_providers::types::EncryptionMaterials>,
}
impl OnEncryptInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn materials(mut self, input: impl ::std::convert::Into<material_providers::types::EncryptionMaterials>) -> Self {
    self.materials = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_materials(mut self, input: ::std::option::Option<material_providers::types::EncryptionMaterials>) -> Self {
    self.materials = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_materials(&self) -> &::std::option::Option<material_providers::types::EncryptionMaterials> {
    &self.materials
}
    /// Consumes the builder and constructs a [`OnEncryptInput`](crate::operation::operation::OnEncryptInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::on_encrypt::OnEncryptInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::on_encrypt::OnEncryptInput {
            materials: self.materials,
        })
    }
}
