use fxhash::FxHashMap;
use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};
use std::fmt::Debug;

/// Contains the intermediate aggregation result, which is optimized to be merged with other
/// intermediate results.
#[derive(Clone, PartialEq, Serialize, Deserialize, Readable, Writable)]
pub struct IntermediateAggregationResults2 {
    pub(crate) aggs_res: VecWithNames<IntermediateAggregationResult>,
}
impl Debug for IntermediateAggregationResults2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IntermediateAggregationResults2").finish()
    }
}

/// Represents an associative array `(key => values)` in a very efficient manner.
#[derive(Clone, PartialEq, Serialize, Deserialize, Readable, Writable)]
pub(crate) struct VecWithNames<T: Clone> {
    pub(crate) values: Vec<T>,
    keys: Vec<String>,
}

/// An aggregation is either a bucket or a metric.
#[derive(Clone, PartialEq, Serialize, Deserialize, Readable, Writable)]
pub enum IntermediateAggregationResult {
    /// Bucket variant
    Bucket(IntermediateBucketResult),
    /// Metric variant
    Metric(IntermediateMetricResult),
}
/// Holds the intermediate data for metric results
#[derive(Clone, PartialEq, Serialize, Deserialize, Readable, Writable)]
pub enum IntermediateMetricResult {
    /// Intermediate average result.
    Percentiles(PercentilesCollector),
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Readable, Writable)]
/// The percentiles collector used during segment collection and for merging results.
pub struct PercentilesCollector {
    sketch: Sketch,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Readable, Writable)]
/// The percentiles collector used during segment collection and for merging results.
pub struct Sketch {
    store: Store,
}
#[derive(Clone, PartialEq, Serialize, Deserialize, Readable, Writable)]
/// The percentiles collector used during segment collection and for merging results.
pub struct Store {
    bins: Vec<u64>,
}

/// The intermediate bucket results. Internally they can be easily merged via the keys of the
/// buckets.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Readable, Writable)]
pub enum IntermediateBucketResult {
    /// Term aggregation
    Terms(IntermediateTermBucketResult),
    /// This is the histogram entry for a bucket, which contains a key, count, and optionally
    /// sub_aggregations.
    Histogram {
        /// The column_type of the underlying `Column`
        column_type: Option<ColumnType>,
        /// The buckets
        buckets: Vec<IntermediateHistogramBucketEntry>,
    },
}

/// The column type represents the column type.
/// Any changes need to be propagated to `COLUMN_TYPES`.
#[derive(
    Hash,
    Eq,
    PartialEq,
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    Readable,
    Writable,
)]
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

/// This is the histogram entry for a bucket, which contains a key, count, and optionally
/// sub_aggregations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Readable, Writable)]
pub struct IntermediateHistogramBucketEntry {
    /// The unique the bucket is identified.
    pub key: f64,
    /// The number of documents in the bucket.
    pub doc_count: u64,
    /// The sub_aggregation in this bucket.
    pub sub_aggregation: IntermediateAggregationResults2,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize, Readable, Writable)]
/// Term aggregation including error counts
pub struct IntermediateTermBucketResult {
    pub(crate) entries: FxHashMap<String, IntermediateTermBucketEntry>,
    pub(crate) sum_other_doc_count: u64,
    pub(crate) doc_count_error_upper_bound: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialOrd, Readable, Writable)]
/// The key to identify a bucket.
pub enum Key {
    /// String key
    Str(String),
    /// `f64` key
    F64(f64),
}
impl Eq for Key {}
impl std::hash::Hash for Key {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
        match self {
            Key::Str(text) => text.hash(state),
            Key::F64(val) => val.to_bits().hash(state),
        }
    }
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Str(l), Self::Str(r)) => l == r,
            (Self::F64(l), Self::F64(r)) => l == r,
            _ => false,
        }
    }
}

/// This is the term entry for a bucket, which contains a count, and optionally
/// sub_aggregations.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Readable, Writable)]
pub struct IntermediateTermBucketEntry {
    /// The number of documents in the bucket.
    pub doc_count: u64,
    /// The sub_aggregation in this bucket.
    pub sub_aggregation: IntermediateAggregationResults2,
}
