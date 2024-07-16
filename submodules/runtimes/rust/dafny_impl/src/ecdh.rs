// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod ECDH {
    pub mod ECCUtils {
        pub fn GetPublicKey(
            curveAlgorithm: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
            privateKey: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::GetPublicKey not implemented.");
        }
        pub fn ValidatePublicKey(
            curveAlgorithm: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
            publicKey: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                bool,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::ValidatePublicKey not implemented.");
        }
        pub fn CompressPublicKey(
            publicKey: &::dafny_runtime::Sequence<u8>,
            curveAlgorithm: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::CompressPublicKey not implemented.");
        }
        pub fn DecompressPublicKey(
            publicKey: &::dafny_runtime::Sequence<u8>,
            curveAlgorithm: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::DecompressPublicKey not implemented.");
        }
        pub fn ParsePublicKey(
            publicKey: &::dafny_runtime::Sequence<u8>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::ECCUtils::ParsePublicKey not implemented.");
        }
    }
    pub mod DeriveSharedSecret {
        pub fn CalculateSharedSecret(
            curveAlgorithm: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
            privateKey: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPrivateKey>,
            publicKey: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECCPublicKey>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::dafny_runtime::Sequence<u8>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::DeriveSharedSecret::CalculateSharedSecret not implemented.");
        }
    }
    pub mod KeyGeneration {
        pub fn GenerateKeyPair(
            s: &::std::rc::Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::ECDHCurveSpec>,
        ) -> ::std::rc::Rc<
            crate::Wrappers::Result<
                ::std::rc::Rc<crate::ECDH::EccKeyPair>,
                ::std::rc::Rc<
                    crate::software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            todo!("ECDH::KeyGeneration::GenerateKeyPair not implemented.");
        }
    }
}
