// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::*;

pub mod AESEncryption {
    pub use crate::_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::*;
}
impl crate::_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::AES_GCM {
    #[allow(non_snake_case)]
  pub fn AESEncryptExtern(
    &self,
    _iv: &::dafny_runtime::Sequence<u8>,
    _key: &::dafny_runtime::Sequence<u8>,
    _msg: &::dafny_runtime::Sequence<u8>,
    _aad: &::dafny_runtime::Sequence<u8>
    ) -> ::std::rc::Rc<Wrappers::Result<::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::AESEncryptOutput>, r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::OpaqueError>>{
        todo!("AESEncryptExtern not implemented");
    }
    #[allow(non_snake_case)]
    pub fn AESDecryptExtern(
        &self,
        _key: &::dafny_runtime::Sequence<u8>,
        _cipherTxt: &::dafny_runtime::Sequence<u8>,
        _authTag: &::dafny_runtime::Sequence<u8>,
        _iv: &::dafny_runtime::Sequence<u8>,
        _aad: &::dafny_runtime::Sequence<u8>,
    ) -> ::std::rc::Rc<
        Wrappers::Result<
            ::dafny_runtime::Sequence<u8>,
            r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::OpaqueError,
        >,
    > {
        todo!("AESDecryptExtern not implemented");
    }
}
