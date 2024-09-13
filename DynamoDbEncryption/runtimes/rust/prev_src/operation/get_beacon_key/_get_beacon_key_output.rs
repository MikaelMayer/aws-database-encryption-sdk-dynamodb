// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBeaconKeyOutput {
    #[allow(missing_docs)] // documentation missing in model
pub beacon_key_materials: ::std::option::Option<key_store::types::BeaconKeyMaterials>,
}
impl GetBeaconKeyOutput {
    #[allow(missing_docs)] // documentation missing in model
pub fn beacon_key_materials(&self) -> &::std::option::Option<key_store::types::BeaconKeyMaterials> {
    &self.beacon_key_materials
}
}
impl GetBeaconKeyOutput {
    /// Creates a new builder-style object to manufacture [`GetBeaconKeyOutput`](crate::operation::get_beacon_key::builders::GetBeaconKeyOutput).
    pub fn builder() -> crate::operation::get_beacon_key::builders::GetBeaconKeyOutputBuilder {
        crate::operation::get_beacon_key::builders::GetBeaconKeyOutputBuilder::default()
    }
}

/// A builder for [`GetBeaconKeyOutput`](crate::operation::operation::GetBeaconKeyOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetBeaconKeyOutputBuilder {
    pub(crate) beacon_key_materials: ::std::option::Option<key_store::types::BeaconKeyMaterials>,
}
impl GetBeaconKeyOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn beacon_key_materials(mut self, input: impl ::std::convert::Into<key_store::types::BeaconKeyMaterials>) -> Self {
    self.beacon_key_materials = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_beacon_key_materials(mut self, input: ::std::option::Option<key_store::types::BeaconKeyMaterials>) -> Self {
    self.beacon_key_materials = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_beacon_key_materials(&self) -> &::std::option::Option<key_store::types::BeaconKeyMaterials> {
    &self.beacon_key_materials
}
    /// Consumes the builder and constructs a [`GetBeaconKeyOutput`](crate::operation::operation::GetBeaconKeyOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_beacon_key::GetBeaconKeyOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_beacon_key::GetBeaconKeyOutput {
            beacon_key_materials: self.beacon_key_materials,
        })
    }
}