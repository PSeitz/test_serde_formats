use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Debug};

/// An aggregation is either a bucket or a metric.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum IntermediateAggregationResult {
    /// Bucket variant
    Bucket(IntermediateBucketResult),
    /// Metric variant
    Metric(IntermediateMetricResult),
}

/// Holds the intermediate data for metric results
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum IntermediateMetricResult {
    /// Intermediate average result.
    Percentiles(PercentilesCollector),
    /// Intermediate stats result.
    Stats(IntermediateStats),
}

impl Default for IntermediateMetricResult {
    fn default() -> Self {
        Self::Percentiles(Default::default())
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
/// The percentiles collector used during segment collection and for merging results.
pub struct PercentilesCollector {
    buckets: Vec<u64>, //sketch: sketches_ddsketch::DDSketch,
}

impl Debug for PercentilesCollector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PercentilesCollector").finish()
    }
}

impl Default for PercentilesCollector {
    fn default() -> Self {
        Self {
            buckets: Vec::new(),
            //sketch: sketches_ddsketch::DDSketch::new(Default::default()),
        }
    }
}

/// The intermediate bucket results. Internally they can be easily merged via the keys of the
/// buckets.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum IntermediateBucketResult {
    /// This is the histogram entry for a bucket, which contains a key, count, and optionally
    /// sub_aggregations.
    HistogramVec {
        /// The column_type of the underlying `Column`
        column_type: Option<ColumnType>,
        /// The buckets
        buckets: Vec<IntermediateHistogramBucketEntry>,
    },
    /// This is the histogram entry for a bucket, which contains a key, count, and optionally
    /// sub_aggregations.
    HistogramKeyed {
        /// The column_type of the underlying `Column`
        column_type: Option<ColumnType>,
        /// The buckets
        buckets: HashMap<u64, IntermediateHistogramBucketEntry>,
    },
}

/// This is the histogram entry for a bucket, which contains a key, count, and optionally
/// sub_aggregations.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntermediateHistogramBucketEntry {
    /// The unique the bucket is identified.
    pub key: f64,
    /// The number of documents in the bucket.
    pub doc_count: u64,
    /// The sub_aggregation in this bucket.
    pub sub_aggregation: IntermediateAggregationResults,
}

/// Contains the intermediate aggregation result, which is optimized to be merged with other
/// intermediate results.
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct IntermediateAggregationResults {
    pub(crate) metrics: Option<VecWithNames<IntermediateMetricResult>>,
    pub(crate) buckets: Option<VecWithNames<IntermediateBucketResult>>,
}

/// Represents an associative array `(key => values)` in a very efficient manner.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct VecWithNames<T: Clone + Debug> {
    pub(crate) values: Vec<T>,
    pub(crate) keys: Vec<String>,
}

impl<T: Clone + Debug> Default for VecWithNames<T> {
    fn default() -> VecWithNames<T> {
        Self {
            values: Vec::new(),
            keys: Vec::new(),
        }
    }
}

/// Intermediate result of the stats aggregation that can be combined with other intermediate
/// results.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct IntermediateStats {
    /// The number of extracted values.
    count: u64,
    /// The sum of the extracted values.
    sum: f64,
    /// The min value.
    min: f64,
    /// The max value.
    max: f64,
}

//impl Default for IntermediateStats {
//fn default() -> Self {
//Self {
//count: 0,
//sum: 0.0,
//min: f64::INFINITY,
//max: f64::NEG_INFINITY,
//}
//}
//}

impl Default for IntermediateStats {
    fn default() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::MIN,
            max: f64::MAX,
        }
    }
}

/// The column type represents the column type.
/// Any changes need to be propagated to `COLUMN_TYPES`.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Ord, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
pub enum ColumnType {
    I64 = 0u8,
    U64 = 1u8,
    F64 = 2u8,
    Bytes = 3u8,
    Str = 4u8,
    Bool = 5u8,
    IpAddr = 6u8,
    DateTime = 7u8,
}

impl Default for ColumnType {
    fn default() -> Self {
        Self::F64
    }
}

pub fn get_test_struct() -> IntermediateAggregationResults {
    let mut metrics = VecWithNames::default();
    let mut buckets = VecWithNames::default();

    metrics.keys.push("percentiles".to_owned());

    let percentile_buckets: Vec<u64> = (0..10_000).collect();
    metrics.values.push(IntermediateMetricResult::Percentiles(
        PercentilesCollector {
            buckets: percentile_buckets,
        },
    ));

    metrics.keys.push("stats".to_owned());
    metrics
        .values
        .push(IntermediateMetricResult::Stats(Default::default()));

    buckets.keys.push("histogram".to_owned());
    let histogram_buckets: Vec<IntermediateHistogramBucketEntry> = (0..10_000)
        .map(|_| IntermediateHistogramBucketEntry {
            key: 10.0,
            doc_count: 100,
            sub_aggregation: get_leaf(),
        })
        .collect();

    buckets.values.push(IntermediateBucketResult::HistogramVec {
        column_type: Default::default(),
        buckets: histogram_buckets,
    });

    IntermediateAggregationResults {
        metrics: Some(metrics),
        buckets: Some(buckets),
    }
}

pub fn get_leaf() -> IntermediateAggregationResults {
    let mut buckets = VecWithNames::default();

    buckets.keys.push("bucket2".to_owned());
    buckets
        .values
        .push(IntermediateBucketResult::HistogramKeyed {
            column_type: Default::default(),
            buckets: vec![(
                10,
                IntermediateHistogramBucketEntry {
                    key: 10.0,
                    doc_count: 100,
                    sub_aggregation: Default::default(),
                },
            )]
            .into_iter()
            .collect(),
        });

    IntermediateAggregationResults {
        metrics: None,
        buckets: Some(buckets),
    }
}
