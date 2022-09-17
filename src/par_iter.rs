use crate::*;

use rayon::prelude::*;
use rayon::iter::plumbing::*;

#[align(64)]
struct Counter(AtomicUsize);

impl<I: ExactSizeIterator + Send, FP: FileProgress + Send> ParallelIterator for FileProgressIter<I, FP> 
where
    I::Item : Send
{
    type Item = I::Item;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(
            self,
            consumer,
        )
    }

    fn opt_len(&self) -> Option<usize> {
        None
    }
}

impl<I: ExactSizeIterator + DoubleEndedIterator + Send, FP: FileProgress + Send> IndexedParallelIterator for FileProgressIter<I, FP> 
where
    I::Item : Send
{
    fn drive<C>(self, consumer: C) -> C::Result
        where C: Consumer<Self::Item>
    {
        bridge(self, consumer)
    }

    fn len(&self) -> usize {
        <Self as ExactSizeIterator>::len(self)
    }

    fn with_producer<CB>(self, callback: CB) -> CB::Output
        where CB: ProducerCallback<Self::Item>
    {
        // Drain every item, and then the vector only needs to free its buffer.
        callback.callback(self)
    }
}

impl<I: ExactSizeIterator + Send, FP: FileProgress + Send> UnindexedProducer for FileProgressIter<I, FP> 
where
    I::Item : Send
{
    type Item = I::Item;

    /// Split the file in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {

    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}

impl<I: DoubleEndedIterator + ExactSizeIterator + Send, FP: FileProgress + Send> Producer for FileProgressIter<I, FP> 
where
    I::Item : Send
{
    type Item = I::Item;
    type IntoIter = Self;

    fn into_iter(self) -> Self::IntoIter {
        self
    }

    fn split_at(mut self, index: usize) -> (Self, Self) {

    }
}