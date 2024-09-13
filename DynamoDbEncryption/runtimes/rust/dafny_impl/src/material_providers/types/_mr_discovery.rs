// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MrDiscovery {
    #[allow(missing_docs)] // documentation missing in model
pub region: ::std::option::Option<::std::string::String>,
}
impl MrDiscovery {
    #[allow(missing_docs)] // documentation missing in model
pub fn region(&self) -> &::std::option::Option<::std::string::String> {
    &self.region
}
}
impl MrDiscovery {
    /// Creates a new builder-style object to manufacture [`MrDiscovery`](crate::material_providers::types::MrDiscovery).
    pub fn builder() -> crate::material_providers::types::builders::MrDiscoveryBuilder {
        crate::material_providers::types::builders::MrDiscoveryBuilder::default()
    }
}

/// A builder for [`MrDiscovery`](crate::material_providers::types::MrDiscovery).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MrDiscoveryBuilder {
    pub(crate) region: ::std::option::Option<::std::string::String>,
}
impl MrDiscoveryBuilder {
    #[allow(missing_docs)] // documentation missing in model
pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
    self.region = ::std::option::Option::Some(input.into());
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
    self.region = input;
    self
}
#[allow(missing_docs)] // documentation missing in model
pub fn get_region(&self) -> &::std::option::Option<::std::string::String> {
    &self.region
}
    /// Consumes the builder and constructs a [`MrDiscovery`](crate::material_providers::types::MrDiscovery).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::material_providers::types::MrDiscovery,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::material_providers::types::MrDiscovery {
            region: self.region,
        })
    }
}
