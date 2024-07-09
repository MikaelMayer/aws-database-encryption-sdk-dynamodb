// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

// Extern methods with a foreign module name
pub mod RSAEncryption {
    pub mod RSA {
        use crate::software::amazon::cryptography::primitives::internaldafny::types::RSAPaddingMode;
        use crate::*;
        #[allow(non_snake_case)]
        pub fn GenerateKeyPairExtern(
            _lengthBits: i32,
        ) -> (::dafny_runtime::Sequence<u8>, ::dafny_runtime::Sequence<u8>) {
            todo!("RSAEncryption_dRSA::GenerateKeyPairExtern not implemented");
        }
        #[allow(non_snake_case)]
        pub fn GetRSAKeyModulusLengthExtern(
            _publicKey: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            Wrappers::Result<
                u32,
                ::std::rc::Rc<
                    software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("RSAEncryption_dRSA::GetRSAKeyModulusLengthExtern not implemented");
        }
        #[allow(non_snake_case)]
        pub fn DecryptExtern(
            _self: &RSAPaddingMode,
            _privateKey: &::dafny_runtime::Sequence<u8>,
            _cipher_text: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("RSAEncryption_dRSA::DecryptExtern not implemented");
        }
        #[allow(non_snake_case)]
        pub fn EncryptExtern(
            _self: &RSAPaddingMode,
            _publicKey: &::dafny_runtime::Sequence<u8>,
            _message: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("RSAEncryption_dRSA::EncryptExtern not implemented");
        }
    }
}
