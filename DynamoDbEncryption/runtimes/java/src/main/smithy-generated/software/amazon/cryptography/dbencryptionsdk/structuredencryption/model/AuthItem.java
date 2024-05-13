// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.dbencryptionsdk.structuredencryption.model;

import java.util.List;
import java.util.Objects;

public class AuthItem {

  private final List<PathSegment> key;

  private final StructuredDataTerminal data;

  private final AuthenticateAction action;

  protected AuthItem(BuilderImpl builder) {
    this.key = builder.key();
    this.data = builder.data();
    this.action = builder.action();
  }

  public List<PathSegment> key() {
    return this.key;
  }

  public StructuredDataTerminal data() {
    return this.data;
  }

  public AuthenticateAction action() {
    return this.action;
  }

  public Builder toBuilder() {
    return new BuilderImpl(this);
  }

  public static Builder builder() {
    return new BuilderImpl();
  }

  public interface Builder {
    Builder key(List<PathSegment> key);

    List<PathSegment> key();

    Builder data(StructuredDataTerminal data);

    StructuredDataTerminal data();

    Builder action(AuthenticateAction action);

    AuthenticateAction action();

    AuthItem build();
  }

  static class BuilderImpl implements Builder {

    protected List<PathSegment> key;

    protected StructuredDataTerminal data;

    protected AuthenticateAction action;

    protected BuilderImpl() {}

    protected BuilderImpl(AuthItem model) {
      this.key = model.key();
      this.data = model.data();
      this.action = model.action();
    }

    public Builder key(List<PathSegment> key) {
      this.key = key;
      return this;
    }

    public List<PathSegment> key() {
      return this.key;
    }

    public Builder data(StructuredDataTerminal data) {
      this.data = data;
      return this;
    }

    public StructuredDataTerminal data() {
      return this.data;
    }

    public Builder action(AuthenticateAction action) {
      this.action = action;
      return this;
    }

    public AuthenticateAction action() {
      return this.action;
    }

    public AuthItem build() {
      if (Objects.isNull(this.key())) {
        throw new IllegalArgumentException(
          "Missing value for required field `key`"
        );
      }
      if (Objects.isNull(this.data())) {
        throw new IllegalArgumentException(
          "Missing value for required field `data`"
        );
      }
      if (Objects.isNull(this.action())) {
        throw new IllegalArgumentException(
          "Missing value for required field `action`"
        );
      }
      return new AuthItem(this);
    }
  }
}
