package software.aws.cryptography.examples.migration;

import com.amazonaws.services.dynamodbv2.datamodeling.encryption.DynamoDBEncryptor;
import com.amazonaws.services.dynamodbv2.datamodeling.encryption.providers.DirectKmsMaterialProvider;
import com.amazonaws.services.kms.AWSKMS;
import com.amazonaws.services.kms.AWSKMSClientBuilder;
import software.amazon.awssdk.core.client.config.ClientOverrideConfiguration;
import software.amazon.awssdk.enhanced.dynamodb.DynamoDbEnhancedClient;
import software.amazon.awssdk.enhanced.dynamodb.DynamoDbTable;
import software.amazon.awssdk.enhanced.dynamodb.Key;
import software.amazon.awssdk.enhanced.dynamodb.TableSchema;
import software.amazon.awssdk.enhanced.dynamodb.model.GetItemEnhancedRequest;
import software.amazon.awssdk.services.dynamodb.DynamoDbClient;
import software.amazon.cryptography.dbencryptionsdk.dynamodb.model.LegacyConfig;
import software.amazon.cryptography.dbencryptionsdk.dynamodb.model.LegacyPolicy;
import software.amazon.cryptography.materialproviders.IKeyring;
import software.amazon.cryptography.materialproviders.MaterialProviders;
import software.amazon.cryptography.materialproviders.model.CreateAwsKmsMrkMultiKeyringInput;
import software.amazon.cryptography.materialproviders.model.MaterialProvidersConfig;
import software.amazon.cryptography.dbencryptionsdk.structuredencryption.model.CryptoAction;
import software.aws.cryptography.dbencryptionsdk.dynamodb.DynamoDbEncryptionInterceptor;
import software.aws.cryptography.dbencryptionsdk.dynamodb.enhancedclient.CreateDynamoDbEncryptionInterceptorInput;
import software.aws.cryptography.dbencryptionsdk.dynamodb.enhancedclient.DynamoDbEnhancedClientEncryption;
import software.aws.cryptography.dbencryptionsdk.dynamodb.enhancedclient.DynamoDbEnhancedTableEncryptionConfig;

import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/*
  Migration Step 3: This is an example demonstrating how to update your configuration
  to stop accepting reading items encrypted using the old format.
  In order to proceed with this step, you will need to re-encrypt all
  old items in your table.

  Once you complete Step 3, you can be sure that all items being read by your system
  ensure the security properties configured for the new format.

  Running this example requires access to the DDB Table whose name
  is provided in CLI arguments.
  This table must be configured with the following
  primary key configuration:
    - Partition key is named "partition_key" with type (S)
    - Sort key is named "sort_key" with type (S)
 */
public class MigrationExampleStep3 {

    public static void MigrationStep3(String kmsKeyId, String ddbTableName, int sortReadValue) {
        // 1. Continue to configure your Keyring, Table Schema,
        // and allowedUnauthenticatedAttributes as you did in Step 1.
        // However, now you can remove the configuration for the old DynamoDBEncryptor
        // and the legacy attribute actions.
        final MaterialProviders matProv = MaterialProviders.builder()
                .MaterialProvidersConfig(MaterialProvidersConfig.builder().build())
                .build();
        final CreateAwsKmsMrkMultiKeyringInput keyringInput = CreateAwsKmsMrkMultiKeyringInput.builder()
                .generator(kmsKeyId)
                .build();
        final IKeyring kmsKeyring = matProv.CreateAwsKmsMrkMultiKeyring(keyringInput);

        final TableSchema<SimpleClass> tableSchema = TableSchema.fromBean(SimpleClass.class);

        final List<String> unauthAttributes = Arrays.asList("do_nothing");

        // 3. Create the DynamoDb Encryption Interceptor with the above configuration.
        //    Do not configure any legacy behavior.
        final Map<String, DynamoDbEnhancedTableEncryptionConfig> tableConfigs = new HashMap<>();
        tableConfigs.put(ddbTableName,
                DynamoDbEnhancedTableEncryptionConfig.builder()
                        .logicalTableName(ddbTableName)
                        .keyring(kmsKeyring)
                        .allowedUnauthenticatedAttributes(unauthAttributes)
                        .tableSchema(tableSchema)
                        .build());
        final DynamoDbEncryptionInterceptor interceptor =
                DynamoDbEnhancedClientEncryption.CreateDynamoDbEncryptionInterceptor(
                        CreateDynamoDbEncryptionInterceptorInput.builder()
                                .tableEncryptionConfigs(tableConfigs)
                                .build()
                );

        // 4. Create a new AWS SDK DynamoDb client using the DynamoDb Encryption Interceptor above
        final DynamoDbClient ddb = DynamoDbClient.builder()
                .overrideConfiguration(
                        ClientOverrideConfiguration.builder()
                                .addExecutionInterceptor(interceptor)
                                .build())
                .build();

        // 5. Create the DynamoDbEnhancedClient using the AWS SDK Client created above,
        //    and create a Table with your modelled class
        final DynamoDbEnhancedClient enhancedClient = DynamoDbEnhancedClient.builder()
                .dynamoDbClient(ddb)
                .build();
        final DynamoDbTable<SimpleClass> table = enhancedClient.table(ddbTableName, tableSchema);

        // 6. Put an item into your table using the DynamoDb Enhanced Client.
        //    This item will be encrypted in the latest format, using the
        //    configuration from your modelled class to decide
        //    which attribute to encrypt and/or sign.
        final SimpleClass item = new SimpleClass();
        item.setPartitionKey("MigrationExample");
        item.setSortKey(3);
        item.setEncryptAndSign("encrypt and sign me!");
        item.setSignOnly("sign me!");
        item.setDoNothing("ignore me!");

        table.putItem(item);

        // 7. Get an item back from the table using the DynamoDb Enhanced Client.
        //    If this is an item written in the old format (e.g. any item written
        //    during Step 0 or 1), then we fail to return the item.
        //    If this is an item written in the new format (e.g. any item written
        //    during Step 2 or after), then we will attempt to decrypt the item using
        //    the non-legacy behavior.
        final Key key = Key.builder()
                .partitionValue("MigrationExample").sortValue(sortReadValue)
                .build();

        final SimpleClass decryptedItem = table.getItem(
                (GetItemEnhancedRequest.Builder requestBuilder) -> requestBuilder.key(key));

        // Demonstrate we get the expected item back
        assert decryptedItem.getPartitionKey().equals("MigrationExample");
        assert decryptedItem.getEncryptAndSign().equals("encrypt and sign me!");
    }

    public static void main(final String[] args) {
        if (args.length < 3) {
            throw new IllegalArgumentException("To run this example, include the kmsKeyId, ddbTableName, and sortReadValue as args.");
        }
        final String kmsKeyId = args[0];
        final String ddbTableName = args[1];
        // You can manipulate this value to demonstrate reading records written in other steps
        final int sortReadValue = Integer.parseInt(args[2]);
        MigrationStep3(kmsKeyId, ddbTableName, sortReadValue);
    }
}
