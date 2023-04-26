
# Serde Format Tests

Compare different formats if they succeed in de/serialization roundtrip for a reasonable complex datatype.

The tested datastructure contains:
- Nested enums
- Circular structs

# Results

| Format      | Result | Serialized Size | Serialize Time [ns] | Deserialize Time [ns] | Roundtrip Time [ns] |
|-------------|--------|-----------------|---------------------|-----------------------|---------------------|
| Json        | Ok     | 2469160         | 8218661             | 7814051               | 16032712            |
| RON         | Ok     | 2179713         | 9211143             | 40118776              | 49329919            |
| Bincode     | Ok     | 960144          | 2387951             | 2895743               | 5283694             |
| Bitcode     | Ok     | 580069          | 2002988             | 2335723               | 4338711             |
| MessagePack | Ok     | 609723          | 1722762             | 3981611               | 5704373             |
| Postcard    | Ok     | 379939          | 1106667             | 1653843               | 2760510             |
| Ciborium    | Ok     | 1749907         | 6345706             | 15620680              | 21966386            |


## Term Aggregation

| Format      | Result | Serialized Size | Serialize Time [ns] | Deserialize Time [ns] | Roundtrip Time [ns] |
|-------------|--------|-----------------|---------------------|-----------------------|---------------------|
| Json        | Ok     | 61938           | 72626               | 160170                | 232796              |
| RON         | Ok     | 55764           | 346068              | 430046                | 776114              |
| Bincode     | Ok     | 34789           | 12343               | 35797                 | 48140               |
| Bitcode     | Ok     | 20870           | 28534               | 56836                 | 85370               |
| MessagePack | Ok     | 20205           | 32310               | 90339                 | 122649              |
| Postcard    | Ok     | 17860           | 17683               | 44082                 | 61765               |
| Ciborium    | Ok     | 49452           | 96902               | 244397                | 341299              |

## Percentile Aggregation

| Format      | Result | Serialized Size | Serialize Time [ns] | Deserialize Time [ns] | Roundtrip Time [ns] |
|-------------|--------|-----------------|---------------------|-----------------------|---------------------|
| Json        | Ok     | 1842            | 7023                | 7564                  | 14587               |
| RON         | Ok     | 1732            | 21330               | 36047                 | 57377               |
| Bincode     | Ok     | 4404            | 2705                | 2665                  | 5370                |
| Bitcode     | Ok     | 4219            | 7865                | 3687                  | 11552               |
| MessagePack | Ok     | 798             | 4769                | 13905                 | 18674               |
| Postcard    | Ok     | 643             | 3156                | 2375                  | 5531                |
| Ciborium    | Ok     | 1112            | 10719               | 21210                 | 31929               |
