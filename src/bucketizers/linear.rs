use std::ops::{Sub, Div, Deref};
use crate::{bucketize::{Bucketize, BucketizeSingle}, into_usize::IntoUsize};


/// A bucketizer that uses linearly spaced buckets to bucketize data.
///
/// The `LinearBucketizer` takes a start value, an end value, and the number of
/// equally spaced buckets to create. Each value will be assigned to the
/// corresponding bucket based on its position in the linearly spaced range.
///
/// ```
/// use std::ops::{Sub, Div, Deref};
/// use buckets::into_usize::IntoUsize;
/// pub struct LinearBucketizer<T> 
/// where 
///     T: PartialOrd + Sub<Output = T> + Div<Output = T> + Copy + IntoUsize,
/// {
///     start: T,
///     num_buckets: usize,
///     bucket_width: T
/// }
/// ```
pub struct LinearBucketizer<T> 
where 
    T: PartialOrd + Sub<Output = T> + Div<Output = T> + Copy + IntoUsize,
{
    start: T,
    num_buckets: usize,
    bucket_width: T,
}

impl<T> LinearBucketizer<T> 
where 
    T: PartialOrd + Sub<Output = T> + Div<Output = T> + Copy + IntoUsize,
{
    /// Creates a new `LinearBucketizer` with a specified start value, end value, and number of buckets.
    ///
    /// # Arguments
    ///
    /// * `start` - The start value of the linearly spaced range.
    /// * `end` - The end value of the linearly spaced range.
    /// * `num_buckets` - The number of equally spaced buckets to create.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketizers::linear::LinearBucketizer;
    ///
    /// let start = 0.0;
    /// let end = 20.0;
    /// let num_buckets = 4.0;
    ///
    /// let bucketizer = LinearBucketizer::new(start, end, num_buckets);
    ///
    /// ```
    pub fn new(start: T, end: T, num_buckets: T) -> Self {
        let bucket_width = (end - start) / num_buckets;
        let num_buckets = num_buckets.into_usize();

        LinearBucketizer {
            start,
            num_buckets,
            bucket_width,
        }
    }
}

impl<T> BucketizeSingle<T> for LinearBucketizer<T>
where 
    T: Sub<Output = T>
    + Div<Output = T>
    + PartialOrd
    + IntoUsize
    + Copy
{
    /// Bucketizes a single value using the `LinearBucketizer`.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketize::{Bucketize, BucketizeSingle};
    /// use buckets::bucketizers::linear::LinearBucketizer;
    ///
    /// let start = 0.0;
    /// let end = 20.0;
    /// let num_buckets = 4.0;
    ///
    /// let bucketizer = LinearBucketizer::new(start, end, num_buckets);
    ///
    /// let value = 7.0;
    /// let bucket = bucketizer.bucketize(&value);
    ///
    /// assert_eq!(bucket, 1);
    /// ```
    fn bucketize(&self, value: &T) -> usize {
        let bucket_index = ((*value - self.start) / self.bucket_width).into_usize();
        if bucket_index < self.num_buckets {
            bucket_index
        } else {
            self.num_buckets - 1
        }
    }
}

impl<T, I> Bucketize<T, I> for LinearBucketizer<T>
where
    T: Sub<Output = T> 
    + Div<Output = T> 
    + PartialOrd
    + IntoUsize
    + Copy,
    I: Iterator<Item = T>,
{}

