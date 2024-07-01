// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod Signature {
  pub mod ECDSA {
    use crate::*;
    pub use crate::_software_damazon_dcryptography_dmaterialproviders_dinternaldafny_dtypes::ECDSA::*;

    pub fn ExternKeyGen(_self: &::std::rc::Rc<crate::r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::ECDSASignatureAlgorithm>)
      -> ::std::rc::Rc<Wrappers::Result<::std::rc::Rc<Signature::SignatureKeyPair>, ::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error>>>
    {
      todo!("Signature::ExternKeyGen not implemented");
    }
    pub fn Sign(_self: &::std::rc::Rc<crate::r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::ECDSASignatureAlgorithm>,
                _key: &::dafny_runtime::Sequence<u8>,
                _msg: &::dafny_runtime::Sequence<u8>
    ) -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, ::std::rc::Rc<crate::r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error>>> {
      todo!("Signature::Sign not implemented");
    }
    /*
      pub fn ECDSAVerify(config: &::std::rc::Rc<AwsCryptographyPrimitivesOperations::Config>, input: &::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::ECDSAVerifyInput>) -> ::std::rc::Rc<Wrappers::Result<bool, ::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error>>> {
        let mut output = ::dafny_runtime::MaybePlacebo::<::std::rc::Rc<Wrappers::Result<bool, ::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error>>>>::new();
        output = ::dafny_runtime::MaybePlacebo::from(crate::_externs::Signature_dECDSA::Verify(input.signatureAlgorithm(); input.verificationKey(), input.message(), input.signature()));
        return output.read(); */
    pub fn Verify(_self: &::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::ECDSASignatureAlgorithm>,
                  _key: &::dafny_runtime::Sequence<u8>,
                  _msg: &::dafny_runtime::Sequence<u8>,
                  _sig: &::dafny_runtime::Sequence<u8>
    ) -> ::std::rc::Rc<Wrappers::Result<bool, ::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error>>> {
      todo!("Signature::Verify not implemented");
    }
  }
}
