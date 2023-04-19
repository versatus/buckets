pub trait BucketizeSingle<T: PartialOrd + Copy> {
    fn bucketize(&self, item: &T) -> usize;
}

pub trait Bucketize<T, I>: BucketizeSingle<T> 
where 
    T: PartialOrd + Copy,
    I: Iterator<Item = T>,
{
    fn bucketize_iter(
        &self, 
        iter: I, 
    ) -> Vec<usize> 
    {
        iter.map(move |value| {
            self.bucketize(&value)
        }).collect::<Vec<usize>>()
        
    }
}
