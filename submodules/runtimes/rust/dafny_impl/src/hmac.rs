// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
use crate::*;
use aws_lc_rs::rand::SecureRandom;
use aws_lc_rs::{digest, hmac, rand};

fn convert_algorithm(
    input: &crate::software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm,
) -> hmac::Algorithm {
    match input {
        SHA_512 => hmac::HMAC_SHA512,
        SHA_384 => hmac::HMAC_SHA384,
        SHA_256 => hmac::HMAC_SHA256,
    }
}

// Let's implement HMAC::_default::Digest
impl crate::HMAC::_default {
    #[allow(non_snake_case)]
    pub fn Digest(
        input: &::std::rc::Rc<
            crate::software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
        >,
    ) -> ::std::rc::Rc<
        Wrappers::Result<
            ::dafny_runtime::Sequence<u8>,
            ::std::rc::Rc<
            software::amazon::cryptography::primitives::internaldafny::types::Error,
            >,
        >,
    > {
        let key_vec: Vec<u8> = input.key().iter().collect();
        let the_key = hmac::Key::new(convert_algorithm(&*input.digestAlgorithm()), &key_vec);
        let message_vec: Vec<u8> = input.message().iter().collect();
        let result = hmac::sign(&the_key, &message_vec);
        ::std::rc::Rc::new(Wrappers::Result::Success {
            value: result.as_ref().iter().cloned().collect(),
        })
    }
}

pub mod HMAC {
    use crate::*;
    use aws_lc_rs::rand::SecureRandom;
    use aws_lc_rs::{digest, hmac, rand};
    pub struct _default {}

    pub struct HMac {
        algorithm: hmac::Algorithm,
        context: Option<hmac::Context>,
    }
    impl HMac {
        pub fn Init(&mut self, salt: &::dafny_runtime::Sequence<u8>) {
            let key: Vec<u8> = salt.iter().collect();
            let s_key = hmac::Key::new(self.algorithm, &key);
            self.context = Some(hmac::Context::with_key(&s_key));
        }
        pub fn Build(
            input: &::std::rc::Rc<software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm>,
        ) -> ::std::rc::Rc<
            Wrappers::Result<
                ::dafny_runtime::Object<Self>,
                ::std::rc::Rc<
                software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            let inner = unsafe {
                dafny_runtime::Object::from_rc(std::rc::Rc::new(Self {
                    algorithm: super::convert_algorithm(&*input),
                    context: None,
                }))
            };

            ::std::rc::Rc::new(Wrappers::Result::Success { value: inner })
        }
        pub fn BlockUpdate(&mut self, block: &::dafny_runtime::Sequence<u8>) {
            let part: Vec<u8> = block.iter().collect();
            self.context.as_mut().unwrap().update(&part);
        }
        pub fn GetResult(&mut self) -> ::dafny_runtime::Sequence<u8> {
            let mut inner = self.context.take();
            match inner {
                Some(x) => {
                    let tag = x.sign();
                    tag.as_ref().iter().cloned().collect()
                }
                None => [].iter().cloned().collect(),
            }
        }
    }
}
