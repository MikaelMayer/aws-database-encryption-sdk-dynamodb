// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod internal_SynchronizedLocalCMC {
  use crate::*;

  pub struct SynchronizedLocalCMC {}

  impl SynchronizedLocalCMC {
    pub fn _allocate_object(_cmc: ::dafny_runtime::Object<LocalCMC::LocalCMC>) -> ::dafny_runtime::Object<Self> {
      // SAFETY: Rc is instantiated only here
      unsafe {
        ::dafny_runtime::Object::from_rc(::std::rc::Rc::new(SynchronizedLocalCMC {}))
      }
    }
  }

  
  impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any>
    for SynchronizedLocalCMC {
    ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
  }

  impl ::dafny_runtime::UpcastObject<dyn software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache>
    for SynchronizedLocalCMC {
    ::dafny_runtime::UpcastObjectFn!(dyn software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache);
  }

  impl software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache for SynchronizedLocalCMC {
    fn r#_PutCacheEntry_k(&mut self, _input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::PutCacheEntryInput>) -> std::rc::Rc<crate::Wrappers::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      todo!("r#_software_damazon_dcryptography_dinternaldafny_dSynchronizedLocalCMC::_ctor not implemented");
    }
  
    fn r#_UpdateUsageMetadata_k(&mut self, _input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::UpdateUsageMetadataInput>) -> std::rc::Rc<crate::Wrappers::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      todo!("r#_software_damazon_dcryptography_dinternaldafny_dSynchronizedLocalCMC::_ctor not implemented");
    }
  
    fn r#_GetCacheEntry_k(&mut self, _input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput>) -> std::rc::Rc<crate::Wrappers::Result<std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput>, std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      todo!("r#_software_damazon_dcryptography_dinternaldafny_dSynchronizedLocalCMC::_ctor not implemented");
    }
  
    fn r#_DeleteCacheEntry_k(&mut self, _input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::DeleteCacheEntryInput>) -> std::rc::Rc<crate::Wrappers::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      todo!("r#_software_damazon_dcryptography_dinternaldafny_dSynchronizedLocalCMC::_ctor not implemented");
    }
  }
}