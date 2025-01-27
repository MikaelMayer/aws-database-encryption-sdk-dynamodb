/*
 * Copyright 2015 Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License"). You may not use this file except
 * in compliance with the License. A copy of the License is located at
 *
 * http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the
 * specific language governing permissions and limitations under the License.
 */
package com.amazonaws.services.dynamodbv2.mapper.integration;

import static org.testng.Assert.assertEquals;
import static org.testng.Assert.assertNotNull;
import static org.testng.Assert.assertNull;
import static org.testng.Assert.assertTrue;
import static org.testng.Assert.fail;

import com.amazonaws.services.dynamodbv2.datamodeling.DynamoDBMapper;
import com.amazonaws.services.dynamodbv2.datamodeling.DynamoDBMappingException;
import com.amazonaws.services.dynamodbv2.datamodeling.DynamoDBQueryExpression;
import com.amazonaws.services.dynamodbv2.datamodeling.encryption.DynamoDBEncryptor;
import com.amazonaws.services.dynamodbv2.datamodeling.encryption.EncryptionContext;
import com.amazonaws.services.dynamodbv2.mapper.encryption.IndexRangeKeyTestClass;
import com.amazonaws.services.dynamodbv2.mapper.encryption.TestDynamoDBMapperFactory;
import com.amazonaws.services.dynamodbv2.mapper.encryption.TestEncryptionMaterialsProvider;
import com.amazonaws.services.dynamodbv2.model.AttributeValue;
import com.amazonaws.services.dynamodbv2.model.ComparisonOperator;
import com.amazonaws.services.dynamodbv2.model.Condition;
import com.amazonaws.services.dynamodbv2.model.PutItemRequest;
import java.math.BigDecimal;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import org.testng.annotations.BeforeClass;
import org.testng.annotations.Test;

/**
 * Tests that index range keys are properly handled as common attribute when items are loaded,
 * saved/updated by using primary key. Also tests using index range keys for queries.
 */
public class IndexRangeKeyAttributesITCase
  extends DynamoDBMapperCryptoIntegrationTestBase {

  private static DynamoDBMapper mapper;
  private static final String RANGE_KEY = "rangeKey";
  private static final String INDEX_FOO_RANGE_KEY = "indexFooRangeKey";
  private static final String INDEX_BAR_RANGE_KEY = "indexBarRangeKey";
  private static final String MULTIPLE_INDEX_RANGE_KEY =
    "multipleIndexRangeKey";
  private static final String FOO_ATTRIBUTE = "fooAttribute";
  private static final String BAR_ATTRIBUTE = "barAttribute";
  private static final String VERSION_ATTRIBUTE = "version";

  // We don't start with the current system millis like other tests because
  // it's out of the range of some data types
  private static int start = 1;

  private static final List<Map<String, AttributeValue>> attrs = new LinkedList<
    Map<String, AttributeValue>
  >();
  private static final List<Long> hashKeyValues = new LinkedList<Long>();
  private static final int totalHash = 5;
  private static final int rangePerHash = 64;
  private static final int indexFooRangeStep = 2;
  private static final int indexBarRangeStep = 4;
  private static final int multipleIndexRangeStep = 8;

  // Test data
  static {
    for (int i = 0; i < totalHash; i++) {
      long hashKeyValue = startKey++;
      hashKeyValues.add(hashKeyValue);
      for (int j = 0; j < rangePerHash; j++) {
        Map<String, AttributeValue> attr = new HashMap<
          String,
          AttributeValue
        >();
        attr.put(KEY_NAME, new AttributeValue().withN("" + hashKeyValue));
        attr.put(RANGE_KEY, new AttributeValue().withN("" + j));
        if (j % indexFooRangeStep == 0) attr.put(
          INDEX_FOO_RANGE_KEY,
          new AttributeValue().withN("" + j)
        );
        if (j % indexBarRangeStep == 0) attr.put(
          INDEX_BAR_RANGE_KEY,
          new AttributeValue().withN("" + j)
        );
        if (j % multipleIndexRangeStep == 0) attr.put(
          MULTIPLE_INDEX_RANGE_KEY,
          new AttributeValue().withN("" + j)
        );
        attr.put(
          FOO_ATTRIBUTE,
          new AttributeValue().withS(UUID.randomUUID().toString())
        );
        attr.put(
          BAR_ATTRIBUTE,
          new AttributeValue().withS(UUID.randomUUID().toString())
        );
        attr.put(VERSION_ATTRIBUTE, new AttributeValue().withN("1"));

        attrs.add(attr);
      }
    }
  }

  @BeforeClass
  public static void setUp() throws Exception {
    boolean recreateTable = false;
    setUpTableWithIndexRangeAttribute(recreateTable);
    DynamoDBEncryptor encryptor = DynamoDBEncryptor.getInstance(
      new TestEncryptionMaterialsProvider()
    );
    EncryptionContext context = new EncryptionContext.Builder()
      .withHashKeyName(KEY_NAME)
      .withRangeKeyName(RANGE_KEY)
      .withTableName(TABLE_WITH_INDEX_RANGE_ATTRIBUTE)
      .build();
    // Insert the data
    for (Map<String, AttributeValue> attr : attrs) {
      attr =
        encryptor.encryptAllFieldsExcept(
          attr,
          context,
          KEY_NAME,
          RANGE_KEY,
          INDEX_FOO_RANGE_KEY,
          INDEX_BAR_RANGE_KEY,
          MULTIPLE_INDEX_RANGE_KEY,
          VERSION_ATTRIBUTE
        );
      dynamo.putItem(
        new PutItemRequest(TABLE_WITH_INDEX_RANGE_ATTRIBUTE, attr)
      );
    }
    mapper = TestDynamoDBMapperFactory.createDynamoDBMapper(dynamo);
  }

  /**
   * Tests that attribute annotated with @DynamoDBIndexRangeKey is properly set in the loaded
   * object.
   */
  @Test
  public void testLoad() throws Exception {
    for (Map<String, AttributeValue> attr : attrs) {
      IndexRangeKeyTestClass x = mapper.load(
        newIndexRangeKey(
          Long.parseLong(attr.get(KEY_NAME).getN()),
          Double.parseDouble(attr.get(RANGE_KEY).getN())
        )
      );

      // Convert all numbers to the most inclusive type for easy
      // comparison
      assertEquals(
        new BigDecimal(x.getKey()),
        new BigDecimal(attr.get(KEY_NAME).getN())
      );
      assertEquals(
        new BigDecimal(x.getRangeKey()),
        new BigDecimal(attr.get(RANGE_KEY).getN())
      );
      if (null == attr.get(INDEX_FOO_RANGE_KEY)) assertNull(
        x.getIndexFooRangeKeyWithFakeName()
      ); else assertEquals(
        new BigDecimal(x.getIndexFooRangeKeyWithFakeName()),
        new BigDecimal(attr.get(INDEX_FOO_RANGE_KEY).getN())
      );
      if (null == attr.get(INDEX_BAR_RANGE_KEY)) assertNull(
        x.getIndexBarRangeKey()
      ); else assertEquals(
        new BigDecimal(x.getIndexBarRangeKey()),
        new BigDecimal(attr.get(INDEX_BAR_RANGE_KEY).getN())
      );
      assertEquals(
        new BigDecimal(x.getVersion()),
        new BigDecimal(attr.get(VERSION_ATTRIBUTE).getN())
      );
      assertEquals(x.getFooAttribute(), attr.get(FOO_ATTRIBUTE).getS());
      assertEquals(x.getBarAttribute(), attr.get(BAR_ATTRIBUTE).getS());
    }
  }

  private IndexRangeKeyTestClass newIndexRangeKey(
    long hashKey,
    double rangeKey
  ) {
    IndexRangeKeyTestClass obj = new IndexRangeKeyTestClass();
    obj.setKey(hashKey);
    obj.setRangeKey(rangeKey);
    return obj;
  }

  /** Tests that attribute annotated with @DynamoDBIndexRangeKey is properly saved. */
  @Test
  public void testSave() throws Exception {
    List<IndexRangeKeyTestClass> objs = new ArrayList<IndexRangeKeyTestClass>();
    for (int i = 0; i < 5; i++) {
      IndexRangeKeyTestClass obj = getUniqueObject();
      objs.add(obj);
    }

    for (IndexRangeKeyTestClass obj : objs) {
      mapper.save(obj);
    }

    for (IndexRangeKeyTestClass obj : objs) {
      IndexRangeKeyTestClass loaded = mapper.load(
        IndexRangeKeyTestClass.class,
        obj.getKey(),
        obj.getRangeKey()
      );
      assertEquals(obj, loaded);
    }
  }

  /** Tests that version attribute is still working as expected. */
  @Test
  public void testUpdate() throws Exception {
    List<IndexRangeKeyTestClass> objs = new ArrayList<IndexRangeKeyTestClass>();
    for (int i = 0; i < 5; i++) {
      IndexRangeKeyTestClass obj = getUniqueObject();
      objs.add(obj);
    }

    for (IndexRangeKeyTestClass obj : objs) {
      mapper.save(obj);
    }

    for (IndexRangeKeyTestClass obj : objs) {
      IndexRangeKeyTestClass replacement = getUniqueObject();
      replacement.setKey(obj.getKey());
      replacement.setRangeKey(obj.getRangeKey());
      replacement.setVersion(obj.getVersion());
      mapper.save(replacement);

      IndexRangeKeyTestClass loadedObject = mapper.load(
        IndexRangeKeyTestClass.class,
        obj.getKey(),
        obj.getRangeKey()
      );
      assertEquals(replacement, loadedObject);

      // If we try to update the old version, we should get an error
      replacement.setVersion(replacement.getVersion() - 1);
      try {
        mapper.save(replacement);
        fail("Should have thrown an exception");
      } catch (Exception expected) {}
    }
  }

  /** Tests making queries on local secondary index */
  @Test
  public void testQueryWithIndexRangekey() {
    int indexFooRangePerHash = rangePerHash / indexFooRangeStep;
    int indexBarRangePerHash = rangePerHash / indexBarRangeStep;
    for (long hashKeyValue : hashKeyValues) {
      IndexRangeKeyTestClass hashKeyItem = new IndexRangeKeyTestClass();
      hashKeyItem.setKey(hashKeyValue);

      /** Query items by primary range key */
      List<IndexRangeKeyTestClass> result = mapper.query(
        IndexRangeKeyTestClass.class,
        new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
          .withHashKeyValues(hashKeyItem)
          .withRangeKeyCondition(
            RANGE_KEY,
            new Condition()
              .withAttributeValueList(new AttributeValue().withN("0"))
              .withComparisonOperator(ComparisonOperator.GE.toString())
          )
      );
      assertTrue(rangePerHash == result.size());
      // check that all attributes are retrieved
      for (IndexRangeKeyTestClass itemInFooIndex : result) {
        assertNotNull(itemInFooIndex.getFooAttribute());
        assertNotNull(itemInFooIndex.getBarAttribute());
      }

      /** Query items on index_foo */
      result =
        mapper.query(
          IndexRangeKeyTestClass.class,
          new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
            .withHashKeyValues(hashKeyItem)
            .withRangeKeyCondition(
              INDEX_FOO_RANGE_KEY,
              new Condition()
                .withAttributeValueList(new AttributeValue().withN("0"))
                .withComparisonOperator(ComparisonOperator.GE.toString())
            )
        );
      assertTrue(indexFooRangePerHash == result.size());
      // check that only the projected attributes are retrieved
      for (IndexRangeKeyTestClass itemInFooIndex : result) {
        assertNotNull(itemInFooIndex.getFooAttribute());
        assertNotNull(itemInFooIndex.getBarAttribute());
      }

      /** Query items on index_bar */
      result =
        mapper.query(
          IndexRangeKeyTestClass.class,
          new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
            .withHashKeyValues(hashKeyItem)
            .withRangeKeyCondition(
              INDEX_BAR_RANGE_KEY,
              new Condition()
                .withAttributeValueList(new AttributeValue().withN("0"))
                .withComparisonOperator(ComparisonOperator.GE.toString())
            )
        );
      assertTrue(indexBarRangePerHash == result.size());
      // check that only the projected attributes are retrieved
      for (IndexRangeKeyTestClass itemInBarIndex : result) {
        assertNotNull(itemInBarIndex.getFooAttribute());
        assertNotNull(itemInBarIndex.getBarAttribute());
      }
    }
  }

  /** Tests the exception when user specifies an invalid range key name in the query. */
  @Test
  public void testInvalidRangeKeyNameException() {
    IndexRangeKeyTestClass hashKeyItem = new IndexRangeKeyTestClass();
    hashKeyItem.setKey(0);
    try {
      mapper.query(
        IndexRangeKeyTestClass.class,
        new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
          .withHashKeyValues(hashKeyItem)
          .withRangeKeyCondition(
            "some_range_key",
            new Condition()
              .withAttributeValueList(new AttributeValue().withN("0"))
              .withComparisonOperator(ComparisonOperator.GE.toString())
          )
      );
      fail("some_range_key is not a valid range key name.");
    } catch (DynamoDBMappingException e) {
      System.out.println(e.getMessage());
    } catch (Exception e) {
      fail("Should trigger an DynamoDBMappingException.");
    }
  }

  /** Tests the exception when user specifies an invalid index name in the query. */
  @Test
  public void testInvalidIndexNameException() {
    IndexRangeKeyTestClass hashKeyItem = new IndexRangeKeyTestClass();
    hashKeyItem.setKey(0);
    try {
      mapper.query(
        IndexRangeKeyTestClass.class,
        new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
          .withHashKeyValues(hashKeyItem)
          .withRangeKeyCondition(
            INDEX_BAR_RANGE_KEY,
            new Condition()
              .withAttributeValueList(new AttributeValue().withN("0"))
              .withComparisonOperator(ComparisonOperator.GE.toString())
          )
          .withIndexName("some_index")
      );
      fail("some_index is not a valid index name.");
    } catch (IllegalArgumentException iae) {
      System.out.println(iae.getMessage());
    } catch (Exception e) {
      fail("Should trigger an IllegalArgumentException.");
    }
  }

  /** Tests making queries by using range key that is shared by multiple indexes. */
  @Test
  public void testQueryWithRangeKeyForMultipleIndexes() {
    int multipleIndexRangePerHash = rangePerHash / multipleIndexRangeStep;
    for (long hashKeyValue : hashKeyValues) {
      IndexRangeKeyTestClass hashKeyItem = new IndexRangeKeyTestClass();
      hashKeyItem.setKey(hashKeyValue);

      /** Query items by a range key that is shared by multiple indexes */
      List<IndexRangeKeyTestClass> result = mapper.query(
        IndexRangeKeyTestClass.class,
        new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
          .withHashKeyValues(hashKeyItem)
          .withRangeKeyCondition(
            MULTIPLE_INDEX_RANGE_KEY,
            new Condition()
              .withAttributeValueList(new AttributeValue().withN("0"))
              .withComparisonOperator(ComparisonOperator.GE.toString())
          )
          .withIndexName("index_foo_copy")
      );
      assertTrue(multipleIndexRangePerHash == result.size());
      // check that only the projected attributes are retrieved
      for (IndexRangeKeyTestClass itemInFooIndex : result) {
        assertNotNull(itemInFooIndex.getFooAttribute());
        assertNotNull(itemInFooIndex.getBarAttribute());
      }
      result =
        mapper.query(
          IndexRangeKeyTestClass.class,
          new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
            .withHashKeyValues(hashKeyItem)
            .withRangeKeyCondition(
              MULTIPLE_INDEX_RANGE_KEY,
              new Condition()
                .withAttributeValueList(new AttributeValue().withN("0"))
                .withComparisonOperator(ComparisonOperator.GE.toString())
            )
            .withIndexName("index_bar_copy")
        );
      assertTrue(multipleIndexRangePerHash == result.size());
      // check that only the projected attributes are retrieved
      for (IndexRangeKeyTestClass itemInFooIndex : result) {
        assertNotNull(itemInFooIndex.getFooAttribute());
        assertNotNull(itemInFooIndex.getBarAttribute());
      }

      /** Exception when user doesn't specify which index to use */
      try {
        mapper.query(
          IndexRangeKeyTestClass.class,
          new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
            .withHashKeyValues(hashKeyItem)
            .withRangeKeyCondition(
              MULTIPLE_INDEX_RANGE_KEY,
              new Condition()
                .withAttributeValueList(new AttributeValue().withN("0"))
                .withComparisonOperator(ComparisonOperator.GE.toString())
            )
        );
        fail(
          "No index name is specified when query with a range key shared by multiple indexes"
        );
      } catch (IllegalArgumentException iae) {
        System.out.println(iae.getMessage());
      } catch (Exception e) {
        fail("Should trigger an IllegalArgumentException.");
      }

      /** Exception when user uses an invalid index name */
      try {
        mapper.query(
          IndexRangeKeyTestClass.class,
          new DynamoDBQueryExpression<IndexRangeKeyTestClass>()
            .withHashKeyValues(hashKeyItem)
            .withRangeKeyCondition(
              MULTIPLE_INDEX_RANGE_KEY,
              new Condition()
                .withAttributeValueList(new AttributeValue().withN("0"))
                .withComparisonOperator(ComparisonOperator.GE.toString())
            )
            .withIndexName("index_foo")
        );
        fail(
          "index_foo is not annotated as part of the localSecondaryIndexNames in " +
          "the @DynamoDBIndexRangeKey annotation of multipleIndexRangeKey"
        );
      } catch (IllegalArgumentException iae) {
        System.out.println(iae.getMessage());
      } catch (Exception e) {
        fail("Should trigger an IllegalArgumentException.");
      }
    }
  }

  private IndexRangeKeyTestClass getUniqueObject() {
    IndexRangeKeyTestClass obj = new IndexRangeKeyTestClass();
    obj.setKey(startKey++);
    obj.setRangeKey((double) start++);
    obj.setIndexFooRangeKeyWithFakeName((double) start++);
    obj.setIndexBarRangeKey((double) start++);
    obj.setFooAttribute("" + startKey++);
    obj.setBarAttribute("" + startKey++);
    return obj;
  }
}
