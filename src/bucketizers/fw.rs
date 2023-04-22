use std::ops::{Sub, Div};
use crate::{bucketize::{Bucketize, BucketizeSingle}, into_usize::IntoUsize};

/// A generic Fixed-Width Bucketizer Struct used to 
/// bin data of type T into fixed width buckets
/// 
/// ```
/// use std::ops::{Sub, Div};
/// use buckets::into_usize::IntoUsize;
///
/// pub struct FixedWidthBucketizer<T: PartialOrd + Sub<Output = T> + Div<Output = T> + IntoUsize> {
///     width: T,
///     offset: T,
/// }
/// ```
#[derive(Clone, Debug)]
pub struct FixedWidthBucketizer<T> 
where 
    T: PartialOrd
    + Sub<Output = T>
    + Div<Output = T>
    + IntoUsize
    + Copy
{
    width: T,
    offset: T,
}

impl<T> FixedWidthBucketizer<T> 
where 
    T: PartialOrd
    + Sub<Output = T>
    + Div<Output = T>
    + IntoUsize
    + Copy
{
    /// return a new `FixedWidthBucketizer<T>` instance 
    ///
    /// Creates a new `FixedWidthBucketizer` with a specified bucket width and offset.
    ///
    /// # Arguments
    ///
    /// * `width` - The fixed width of each bucket.
    /// * `offset` - An optional offset for the bucket boundaries.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketizers::fw::FixedWidthBucketizer;
    ///
    /// let width = 5;
    /// let offset = 0;
    /// let bucketizer = FixedWidthBucketizer::new(width, offset);
    ///
    /// ```
    pub fn new(width: T, offset: T) -> Self {
        FixedWidthBucketizer { width, offset }
    }
}

impl<T> BucketizeSingle<T> for FixedWidthBucketizer<T> 
where 
    T: PartialOrd
    + Sub<Output = T>
    + Div<Output = T>
    + IntoUsize 
    + Copy
{
    /// Bucketizes a single value using the FixedWidthBucketizer
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketize::{Bucketize, BucketizeSingle};
    /// use buckets::bucketizers::fw::FixedWidthBucketizer;
    ///
    /// let bucketizer = FixedWidthBucketizer::new(5, 0);
    /// let value = 12;
    /// let bucket = bucketizer.bucketize(&value);
    ///
    /// assert_eq!(bucket, 2);
    /// ```
    fn bucketize(&self, value: &T) -> usize {
        let adjusted_value = *value - self.offset;
        let bucket_index = adjusted_value / self.width;
        bucket_index.into_usize() 
    }
}

impl<T, I> Bucketize<T, I> for FixedWidthBucketizer<T> 
where 
    T: PartialOrd 
    + Sub<Output = T> 
    + Div<Output = T> 
    + IntoUsize
    + Copy,
    I: Iterator<Item = T>,
{}
