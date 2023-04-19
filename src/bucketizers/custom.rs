use std::marker::PhantomData;
use std::iter::Iterator;
use crate::bucketize::{Bucketize, BucketizeSingle}; 

/// A bucketizer that allows the caller to 
/// implement a custom bucketization scheme 
/// and still expose all underlying `Bucketize` trait 
/// methods
///
/// ```
/// use std::marker::PhantomData;
///
/// pub struct CustomBucketizer<T: PartialOrd, F> 
/// where 
///     F: Fn(&T) -> usize 
/// {
///     bucketizer: F,
///     phantom: PhantomData<T>
/// }
///
/// ```
pub struct CustomBucketizer<T: PartialOrd + Copy, F> 
where 
    F: Fn(&T) -> usize 
{
    bucketizer: F,
    phantom: PhantomData<T>
}

impl<T: PartialOrd + Copy, F> CustomBucketizer<T, F> 
where 
    F: Fn(&T) -> usize,
{
    /// Creates a new custom bucketizer with a custom 
    /// bucketization function implemented by the caller.
    /// Exposes all available `Bucketize` trait methods 
    ///
    /// ```
    /// use buckets::bucketize::Bucketize;
    /// use buckets::into_buckets::IntoBuckets;
    /// use buckets::bucketizers::custom::CustomBucketizer;
    ///
    /// let data = vec![0.4, 2.5, 8.5, 10.4, 20.2];
    ///
    /// let bucketizer_func = |value: &f64| -> usize {
    ///     if *value < 5.0 {
    ///         0
    ///     } else if *value < 10.0 {
    ///         1
    ///     } else if *value < 20.0 {
    ///         2
    ///     } else {
    ///         3
    ///     }
    /// };
    /// 
    /// let bucketizer = CustomBucketizer::new(bucketizer_func);
    /// let bucketized = IntoBuckets::new(data.into_iter(), bucketizer);
    /// 
    /// let binned_data: Vec<usize> = bucketized.collect();
    /// 
    /// println!("{:?}", binned_data);
    /// ```
    pub fn new(func: F) -> Self {
        CustomBucketizer { 
            bucketizer: func, 
            phantom: PhantomData::default() 
        }
    }
}

/// Implements the `Bucketize` trait and its methods to the 
/// `CustomBucketizer` type.
impl <T: PartialOrd + Copy, F> BucketizeSingle<T> for CustomBucketizer<T, F> 
where 
    F: Fn(&T) -> usize,
{
    /// Bucketizes a single value using the CustomBucketizer
    ///
    /// ```
    /// use buckets::bucketize::{Bucketize, BucketizeSingle};
    /// use buckets::into_buckets::IntoBuckets;
    /// use buckets::bucketizers::custom::CustomBucketizer;
    ///
    /// let bucketizer_func = |value: &f64| -> usize {
    ///     if *value < 5.0 {
    ///         0
    ///     } else if *value < 10.0 {
    ///         1
    ///     } else if *value < 20.0 {
    ///         2
    ///     } else {
    ///         3
    ///     }
    /// };
    ///
    /// let bucketizer: CustomBucketizer<f64, _> = CustomBucketizer::new(bucketizer_func);
    /// let value: f64 = 7.0;
    /// let bucket = bucketizer.bucketize(&value);
    ///
    /// assert_eq!(bucket, 1);
    ///
    /// ```
    fn bucketize(&self, value: &T) -> usize {
        (self.bucketizer)(value)
    }
}

impl <T, F, I> Bucketize<T, I> for CustomBucketizer<T, F> 
where 
    F: Fn(&T) -> usize,
    T: PartialOrd + Copy,
    I: Iterator<Item = T>,
{}
