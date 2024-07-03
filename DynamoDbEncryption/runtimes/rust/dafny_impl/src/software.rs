

pub mod software {
  pub mod amazon {
    pub mod cryptography {
      pub mod internaldafny {
        pub mod StormTrackingCMC {
          use crate::*;
          pub use crate::storm_tracker::internal_StormTrackingCMC::*;
        }
        pub mod SynchronizedLocalCMC {
          use crate::*;
          pub use crate::local_cmc::internal_SynchronizedLocalCMC::*;
        }
      }
      pub mod dbencryptionsdk {
        pub mod dynamodb {
          pub mod itemencryptor {
            pub mod internaldafny {
              pub mod legacy {
                pub struct InternalLegacyOverride {
                  pub r#__i_policy: ::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_dinternaldafny_dtypes::LegacyPolicy>
                }
                impl InternalLegacyOverride {
                  pub fn Build(config: &::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::DynamoDbItemEncryptorConfig>) -> ::std::rc::Rc<crate::Wrappers::Result<::std::rc::Rc<crate::Wrappers::Option<::dafny_runtime::Object<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dlegacy::InternalLegacyOverride>>>, ::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::Error>>> {
                    ::std::rc::Rc::new(crate::Wrappers::Result::Success {
                      value: ::std::rc::Rc::new(crate::Wrappers::Option::Some {
                        value: 
                        // SAFETY: The Rc's count is 1
                        unsafe {
                          ::dafny_runtime::Object::from_rc(::std::rc::Rc::new(InternalLegacyOverride{
                            r#__i_policy: todo!("InternalLegacyOverride::__i_policy")
                          }))
                        }
                      })
                    })
                  }
                  pub fn EncryptItem(&mut self, input: &::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::EncryptItemInput>) -> ::std::rc::Rc<crate::Wrappers::Result<::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::EncryptItemOutput>, ::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::Error>>> {
                    todo!("InternalLLegacyOverride::EncryptItem")
                  }
                  pub fn DecryptItem(&mut self, input: &::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::DecryptItemInput>) -> ::std::rc::Rc<crate::Wrappers::Result<::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::DecryptItemOutput>, ::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::Error>>> {
                    todo!("InternalLLegacyOverride::DecryptItem")
                  }
                  pub fn IsLegacyInput(&self, input: &::std::rc::Rc<crate::r#_software_damazon_dcryptography_ddbencryptionsdk_ddynamodb_ditemencryptor_dinternaldafny_dtypes::DecryptItemInput>) -> bool {
                    todo!("InternalLLegacyOverride::IsLegacyInput")
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}