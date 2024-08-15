use crate::*;

impl crate::software::amazon::cryptography::services::kms::internaldafny::_default {
    pub fn KMSClient() -> ::std::rc::Rc<crate::Wrappers::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::Error>>>{
        todo!("amazon::cryptography::services::kms::internaldafny::_default::KMSClient()")
    }
}

impl crate::software::amazon::cryptography::services::dynamodb::internaldafny::_default {
    pub fn DynamoDBClient() -> ::std::rc::Rc<crate::Wrappers::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>, ::std::rc::Rc<crate::software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>>>{
        todo!("amazon::cryptography::services::dynamodb::internaldafny::_default::DynamoDBClient()")
    }
}

pub mod software {
    pub mod amazon {
        pub mod cryptography {
            pub mod internaldafny {
                pub mod StormTrackingCMC {
                    pub use crate::storm_tracker::internal_StormTrackingCMC::*;
                    use crate::*;
                }
                pub mod SynchronizedLocalCMC {
                    pub use crate::local_cmc::internal_SynchronizedLocalCMC::*;
                    use crate::*;
                }
            }
            pub mod dbencryptionsdk {
                pub mod dynamodb {
                    pub mod itemencryptor {
                        pub mod internaldafny {
                            pub mod legacy {
                                pub struct InternalLegacyOverride {
                  pub r#__i_policy: ::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::internaldafny::types::LegacyPolicy>
                }
                                impl InternalLegacyOverride {
                                    pub fn Build(config: &::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::DynamoDbItemEncryptorConfig>) -> ::std::rc::Rc<crate::Wrappers::Result<::std::rc::Rc<crate::Wrappers::Option<::dafny_runtime::Object<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::legacy::InternalLegacyOverride>>>, ::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::Error>>>{
                                        ::std::rc::Rc::new(crate::Wrappers::Result::Success {
                                            value: ::std::rc::Rc::new(
                                                crate::Wrappers::Option::Some {
                        value: 
                        // SAFETY: The Rc's count is 1
                        unsafe {
                          ::dafny_runtime::Object::from_rc(::std::rc::Rc::new(InternalLegacyOverride{
                            r#__i_policy: todo!("InternalLegacyOverride::__i_policy")
                          }))
                        }
                      },
                                            ),
                                        })
                                    }
                                    pub fn EncryptItem(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::EncryptItemInput>) -> ::std::rc::Rc<crate::Wrappers::Result<::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::EncryptItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::Error>>>{
                                        todo!("InternalLLegacyOverride::EncryptItem")
                                    }
                                    pub fn DecryptItem(&mut self, input: &::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::DecryptItemInput>) -> ::std::rc::Rc<crate::Wrappers::Result<::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::DecryptItemOutput>, ::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::Error>>>{
                                        todo!("InternalLLegacyOverride::DecryptItem")
                                    }
                                    pub fn IsLegacyInput(
                                        &self,
                                        input: &::std::rc::Rc<crate::software::amazon::cryptography::dbencryptionsdk::dynamodb::itemencryptor::internaldafny::types::DecryptItemInput>,
                                    ) -> bool {
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
