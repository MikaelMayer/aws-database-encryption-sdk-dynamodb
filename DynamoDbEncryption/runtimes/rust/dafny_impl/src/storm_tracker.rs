// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod internal_StormTrackingCMC {
    use crate::*;
    pub struct StormTrackingCMC {}
    impl StormTrackingCMC {
        pub fn _allocate_object(
            _cmc: ::dafny_runtime::Object<StormTracker::StormTracker>,
        ) -> ::dafny_runtime::Object<Self> {
            // SAFETY: The Rc has not been shared before
            unsafe { ::dafny_runtime::Object::from_rc(::std::rc::Rc::new(StormTrackingCMC {})) }
        }
    }

    impl ::dafny_runtime::UpcastObject<dyn ::std::any::Any> for StormTrackingCMC {
        ::dafny_runtime::UpcastObjectFn!(dyn ::std::any::Any);
    }

    impl ::dafny_runtime::UpcastObject<dyn software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache>
    for StormTrackingCMC {
    ::dafny_runtime::UpcastObjectFn!(dyn software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache);
}

    impl crate::software::amazon::cryptography::materialproviders::internaldafny::types::ICryptographicMaterialsCache for StormTrackingCMC {
    fn r#_PutCacheEntry_k(&mut self, _input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::PutCacheEntryInput>) -> std::rc::Rc<crate::Wrappers::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
      todo!("r#_software_damazon_dcryptography_dinternaldafny_dStormTrackingCMC::_ctor not implemented");
    }
  
    fn r#_UpdateUsageMetadata_k(&mut self, _input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::UpdateUsageMetadataInput>) -> std::rc::Rc<crate::Wrappers::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
        todo!()
    }
  
    fn r#_GetCacheEntry_k(&mut self, _input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryInput>) -> std::rc::Rc<crate::Wrappers::Result<std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::GetCacheEntryOutput>, std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
        todo!()
    }
  
    fn r#_DeleteCacheEntry_k(&mut self, _input: &std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::DeleteCacheEntryInput>) -> std::rc::Rc<crate::Wrappers::Result<(), std::rc::Rc<crate::software::amazon::cryptography::materialproviders::internaldafny::types::Error>>> {
        todo!()
    }
  }
}
