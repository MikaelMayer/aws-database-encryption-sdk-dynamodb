// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CryptoConfig {

}
impl CryptoConfig {

}
impl CryptoConfig {
    /// Creates a new builder-style object to manufacture [`CryptoConfig`](crate::primitives::types::CryptoConfig).
    pub fn builder() -> crate::primitives::types::crypto_config::CryptoConfigBuilder {
        crate::primitives::types::crypto_config::CryptoConfigBuilder::default()
    }
}

/// A builder for [`CryptoConfig`](crate::primitives::types::CryptoConfig).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CryptoConfigBuilder {

}
impl CryptoConfigBuilder {

    /// Consumes the builder and constructs a [`CryptoConfig`](crate::primitives::types::CryptoConfig).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::primitives::types::crypto_config::CryptoConfig,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::primitives::types::crypto_config::CryptoConfig {

        })
    }
}
