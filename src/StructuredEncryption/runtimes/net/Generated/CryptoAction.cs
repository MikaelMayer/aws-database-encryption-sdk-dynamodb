// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
using System;
 using AWS.Cryptography.StructuredEncryption; namespace AWS.Cryptography.StructuredEncryption {
 using Amazon.Runtime; public class CryptoAction : ConstantClass {

 
 public static readonly CryptoAction ENCRYPT_AND_SIGN = new CryptoAction ("ENCRYPT_AND_SIGN");
 
 public static readonly CryptoAction SIGN_ONLY = new CryptoAction ("SIGN_ONLY");
 
 public static readonly CryptoAction IGNORE = new CryptoAction ("IGNORE");
 public static readonly  CryptoAction [] Values =  {
 ENCRYPT_AND_SIGN , IGNORE , SIGN_ONLY
} ;
 public CryptoAction (string value) : base(value) {}
}
}
