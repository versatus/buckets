use crate::bucketize::{Bucketize, BucketizeSingle};

/// A bucketizer that uses predefined ranges to bucketize data.
///
/// The `RangeBucketizer` takes a list of ranges (represented as tuples)
/// where each range specifies the inclusive lower bound and exclusive upper
/// bound for a bucket. Each value will be assigned to the bucket with the
/// corresponding range.
///
/// # Example
///
/// ```
/// pub struct RangeBucketizer<T: PartialOrd + Copy> {
///     ranges: Vec<(T, T)>,
/// }
/// ```
pub struct RangeBucketizer<T: PartialOrd + Copy> {
    ranges: Vec<(T, T)>,
}

impl<T> RangeBucketizer<T> 
where
    T: PartialOrd + Copy,
{
    /// Creates a new `RangeBucketizer` with a specified list of ranges.
    ///
    /// # Arguments
    ///
    /// * `ranges` - A vector of tuples representing the inclusive lower bound and exclusive upper bound for each bucket.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketizers::range::RangeBucketizer;
    ///
    /// let ranges = vec![
    ///     (0, 5),
    ///     (5, 10),
    ///     (10, 20),
    ///     (20, std::i32::MAX),
    /// ];
    ///
    /// let bucketizer = RangeBucketizer::new(ranges);
    ///
    /// // The bucketizer can now be used to bucketize data using the Bucketize trait.
    /// ```
    pub fn new(ranges: Vec<(T, T)>) -> Self {
        RangeBucketizer { ranges }
    }
}

impl<T: PartialOrd + Copy> BucketizeSingle<T> for RangeBucketizer<T> {
    /// Bucketizes a single value using the `RangeBucketizer`.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketize::{Bucketize, BucketizeSingle};
    /// use buckets::bucketizers::range::RangeBucketizer;
    ///
    /// let ranges = vec![
    ///     (0, 5),
    ///     (5, 10),
    ///     (10, 20),
    ///     (20, std::i32::MAX),
    /// ];
    ///
    /// let bucketizer = RangeBucketizer::new(ranges);
    ///
    /// let value = 7;
    /// let bucket = bucketizer.bucketize(&value);
    ///
    /// assert_eq!(bucket, 1);
    /// ```
    fn bucketize(&self, value: &T) -> usize {
        let bucket_position = self.ranges
            .iter()
            .position(|(start, end)| value >= start && value < end);

        if let Some(val) = bucket_position {
            return val
        } else {
            return self.ranges.len() - 1 
        }
    }
}

impl<T, I> Bucketize<T, I> for RangeBucketizer<T> 
where 
    T: PartialOrd + Copy,
    I: Iterator<Item = T>,
{}
