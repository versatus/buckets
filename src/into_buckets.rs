use crate::bucketize::Bucketize;
/// A type to convert into when bucketizing 
///
/// ```
/// pub struct IntoBuckets<I, B> {
///     inner: I,
///     bucketizer: B,
/// }
pub struct IntoBuckets<I, B> {
    inner: I,
    bucketizer: B,
}

impl<I, B> IntoBuckets<I, B> {
    /// Creates a new `IntoBuckets` iterator by wrapping the input iterator `inner`
    /// and applying the provided bucketizer `bucketizer`.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketize::Bucketize;
    /// use buckets::bucketizers::fw::FixedWidthBucketizer;
    /// use buckets::into_buckets::IntoBuckets;
    ///
    /// let data = vec![1.0, 6.0, 11.0, 25.0];
    /// let bucketizer = FixedWidthBucketizer::new(5.0, 0.0);
    ///
    /// let bucketized = IntoBuckets::new(data.into_iter(), bucketizer);
    ///
    /// let binned_data: Vec<usize> = bucketized.collect();
    ///
    /// assert_eq!(binned_data, vec![0, 1, 2, 5]);
    /// ```
    pub fn new(inner: I, bucketizer: B) -> Self {
        IntoBuckets { inner, bucketizer }
    }
}

impl<I, T, B> Iterator for IntoBuckets<I, B>
where
    I: Iterator<Item = T>,
    B: Bucketize<T, I>,
    T: PartialOrd + Default + Copy,
{
    type Item = usize;
    /// Returns the next bucketized value from the inner iterator.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketize::Bucketize;
    /// use buckets::bucketizers::fw::FixedWidthBucketizer;
    /// use buckets::into_buckets::IntoBuckets;
    ///
    /// let data = vec![1.0, 6.0, 11.0, 25.0];
    /// let bucketizer = FixedWidthBucketizer::new(5.0, 0.0);
    ///
    /// let mut bucketized = IntoBuckets::new(data.into_iter(), bucketizer);
    ///
    /// assert_eq!(bucketized.next(), Some(0));
    /// assert_eq!(bucketized.next(), Some(1));
    /// assert_eq!(bucketized.next(), Some(2));
    /// assert_eq!(bucketized.next(), Some(5));
    /// assert_eq!(bucketized.next(), None);
    /// ```
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|value| {
                self.bucketizer.bucketize(
                    &value 
                )
            }
        )
    }
}
