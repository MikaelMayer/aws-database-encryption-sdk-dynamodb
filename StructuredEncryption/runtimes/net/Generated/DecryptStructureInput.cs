// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
using System;
 using AWS.Cryptography.StructuredEncryption; namespace AWS.Cryptography.StructuredEncryption {
 public class DecryptStructureInput {
 private string _tableName ;
 private AWS.Cryptography.StructuredEncryption.StructuredData _encryptedStructure ;
 private AWS.Cryptography.StructuredEncryption.AuthenticateSchema _authenticateSchema ;
 private AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager _cmm ;
 private System.Collections.Generic.Dictionary<string, string> _encryptionContext ;
 public string TableName {
 get { return this._tableName; }
 set { this._tableName = value; }
}
 public bool IsSetTableName () {
 return this._tableName != null;
}
 public AWS.Cryptography.StructuredEncryption.StructuredData EncryptedStructure {
 get { return this._encryptedStructure; }
 set { this._encryptedStructure = value; }
}
 public bool IsSetEncryptedStructure () {
 return this._encryptedStructure != null;
}
 public AWS.Cryptography.StructuredEncryption.AuthenticateSchema AuthenticateSchema {
 get { return this._authenticateSchema; }
 set { this._authenticateSchema = value; }
}
 public bool IsSetAuthenticateSchema () {
 return this._authenticateSchema != null;
}
 public AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager Cmm {
 get { return this._cmm; }
 set { this._cmm = value; }
}
 public bool IsSetCmm () {
 return this._cmm != null;
}
 public System.Collections.Generic.Dictionary<string, string> EncryptionContext {
 get { return this._encryptionContext; }
 set { this._encryptionContext = value; }
}
 public bool IsSetEncryptionContext () {
 return this._encryptionContext != null;
}
 public void Validate() {
 if (!IsSetTableName()) throw new System.ArgumentException("Missing value for required property 'TableName'");
 if (!IsSetEncryptedStructure()) throw new System.ArgumentException("Missing value for required property 'EncryptedStructure'");
 if (!IsSetAuthenticateSchema()) throw new System.ArgumentException("Missing value for required property 'AuthenticateSchema'");
 if (!IsSetCmm()) throw new System.ArgumentException("Missing value for required property 'Cmm'");

}
}
}
