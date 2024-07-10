// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod AesKdfCtr {
    use crate::software::amazon::cryptography::primitives::internaldafny::types::Error as DafnyError;
    use crate::*;
    use ::dafny_runtime::Sequence;
    use aws_lc_rs::cipher::{DecryptingKey, EncryptingKey, UnboundCipherKey, AES_256};
    use aws_lc_rs::error::Unspecified;
    use std::rc::Rc;

    pub struct _default {}

    fn error(s: &str) -> Rc<DafnyError> {
        Rc::new(DafnyError::AwsCryptographicPrimitivesError {
            message:
                dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(s),
        })
    }

    fn ctr_stream(nonce: &[u8], key: &[u8], length: u32) -> Result<Vec<u8>, Unspecified> {
        let mut in_out_buffer = Vec::new();
        in_out_buffer.resize(length as usize, 0);

        let key = UnboundCipherKey::new(&AES_256, key)?;
        let mut encrypting_key = EncryptingKey::ctr(key)?;
        let context = encrypting_key.encrypt(&mut in_out_buffer)?;
        Ok(in_out_buffer)
    }

    impl _default {
        pub fn AesKdfCtrStream(
            nonce: &Sequence<u8>,
            key: &Sequence<u8>,
            length: u32,
        ) -> Rc<
            Wrappers::Result<
                Sequence<u8>,
                Rc<crate::software::amazon::cryptography::primitives::internaldafny::types::Error>,
            >,
        > {
            let nonce: Vec<u8> = nonce.iter().collect();
            let key: Vec<u8> = key.iter().collect();
            match ctr_stream(&nonce, &key, length) {
                Ok(x) => Rc::new(Wrappers::Result::Success {
                    value: x.iter().cloned().collect(),
                }),
                Err(e) => {
                    let msg = format!("{}", e);
                    Rc::new(Wrappers::Result::Failure { error: error(&msg) })
                }
            }
        }
    }
}
