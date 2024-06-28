// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::*;

// Let's implement HMAC::_default::Digest
impl crate::HMAC::_default {
  #[allow(non_snake_case)]
  pub fn Digest(_input: &::std::rc::Rc<crate::r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::HMacInput>)
    -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Sequence<u8>, ::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error>>> {
      todo!("HMAC::_default::Digest not implemented");
  }
}

pub mod HMAC {
  use crate::*;
  pub struct _default {}
  
  pub struct HMac {
  }
  impl HMac {
    pub fn Init(&mut self, _salt: &::dafny_runtime::Sequence<u8>) {
      todo!("HMAC::HMac::Init not implemented");
    }
    pub fn Build(_input: &::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::DigestAlgorithm>) 
    -> ::std::rc::Rc<Wrappers::Result<::dafny_runtime::Object<Self>, ::std::rc::Rc<r#_software_damazon_dcryptography_dprimitives_dinternaldafny_dtypes::Error>>> {
      todo!("HMAC::HMac::Build not implemented");
    }
    pub fn BlockUpdate(&mut self, _block: &::dafny_runtime::Sequence<u8>) {
      todo!("HMAC::HMac::BlockUpdate not implemented");
    }
    pub fn GetResult(&mut self) -> ::dafny_runtime::Sequence<u8> {
      todo!("HMAC::HMac::GetResult not implemented");
    }
  }
}
