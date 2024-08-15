// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod ECDH {
    pub mod KeyGeneration {
        pub fn GenerateKeyPair(
            _input: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::std::rc::Rc<crate::ECDH::EccKeyPair>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::KeyGeneration::GenerateKeyPair");
        }
    }
    pub mod ECCUtils {
        pub fn GetPublicKey(
            _input: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
            _key: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::GetPublicKey");
        }
        pub fn ParsePublicKey(
            _key: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::ParsePublicKey");
        }
        pub fn ValidatePublicKey(
            _input: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
            _key: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                bool,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::ValidatePublicKey");
        }
        pub fn CompressPublicKey(
            _input: &::dafny_runtime::Sequence<u8>,
            _curve: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::CompressPublicKey");
        }

        pub fn DecompressPublicKey(
            _input: &::dafny_runtime::Sequence<u8>,
            _curve: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::DecompressPublicKey");
        }
    }
    pub mod DeriveSharedSecret {
        pub fn CalculateSharedSecret(
            _input: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
            _private_key: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
            _public_key: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPublicKey>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::DeriveSharedSecret::CalculateSharedSecret");
        }
    }
}
