
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


