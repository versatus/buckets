use crate::bucketize::{Bucketize, BucketizeSingle};

/// A bucketizer struct to bin data into quantiles 
///
/// ```
/// pub struct QuantileBucketizer<T: PartialOrd + Copy> {
///     quantiles: Vec<T>,
///     n_quantiles: usize
/// }
#[derive(Clone, Debug)]
pub struct QuantileBucketizer<T> 
where 
    T: PartialOrd + Copy 
{
    quantiles: Vec<T>,
    n_quantiles: usize
}

impl<T: PartialOrd + Copy> QuantileBucketizer<T> {
    /// Creates a new QuantileBucketizer with the given quantiles.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketizers::quantile::QuantileBucketizer;
    ///
    /// let quantiles = vec![25.0, 50.0, 75.0];
    /// let bucketizer = QuantileBucketizer::new(quantiles, 3);
    /// ```
    pub fn new(quantiles: Vec<T>, n_quantiles: usize) -> Self {
        QuantileBucketizer { quantiles, n_quantiles}
    }

    pub fn get_n_quantiles(&self) -> usize {
        self.n_quantiles
    }
}

impl<T: PartialOrd + Copy> BucketizeSingle<T> for QuantileBucketizer<T> {
    /// Bucketizes a single value using the QuantileBucketizer.
    ///
    /// # Example
    ///
    /// ```
    /// use buckets::bucketize::BucketizeSingle;
    /// use buckets::bucketizers::quantile::QuantileBucketizer;
    ///
    /// let quantiles = vec![25.0, 50.0, 75.0];
    /// let bucketizer = QuantileBucketizer::new(quantiles, 3);
    ///
    /// let value: f64 = 55.0;
    /// let bucket = bucketizer.bucketize(&value);
    /// assert_eq!(bucket, 2);
    /// ```
    fn bucketize(&self, value: &T) -> usize {
        self.quantiles
            .iter()
            .position(|&quantile| value < &quantile)
            .unwrap_or(self.quantiles.len())
    }
}

impl<T, I> Bucketize<T, I> for QuantileBucketizer<T> 
where
    T: PartialOrd + Copy,
    I: Iterator<Item = T>,
{}
