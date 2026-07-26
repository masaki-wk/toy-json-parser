use crate::CodeLocation;

/// An iterator which gives a tuple of a current [`CodeLocation`] and an item from [`Iterator`].
#[derive(Debug)]
pub struct LocatedIterator<T> {
    inner: T,
    location: CodeLocation,
}

impl<T> LocatedIterator<T> {
    // Creates a new `LocatedIterator` from a mutable reference implementing `Iterator`.
    const fn new(inner: T) -> Self {
        Self {
            inner,
            location: CodeLocation::new(1, 1),
        }
    }

    // Advances the iterator and returns the tuple of the current location and the item.
    fn take_item(&mut self) -> Option<(CodeLocation, T::Item)>
    where
        T: Iterator,
        <T as Iterator>::Item: PartialEq<char>,
    {
        let item = self.inner.next()?;
        let loc = self.location;
        if item == '\n' {
            self.location.line += 1;
            self.location.column = 1;
        } else {
            self.location.column += 1;
        }
        Some((loc, item))
    }
}

impl<T> Iterator for LocatedIterator<T>
where
    T: Iterator,
    <T as Iterator>::Item: PartialEq<char>,
{
    type Item = (CodeLocation, T::Item);

    fn next(&mut self) -> Option<Self::Item> {
        self.take_item()
    }
}

/// Extends [`Iterator`] trait with [`locate`] method which gives the located iterator.
///
/// [`locate`]: IteratorLocationExt::locate
///
pub trait IteratorLocationExt: Iterator {
    /// Creates an iterator which gives a tuple of a current [`CodeLocation`] and an item from [`Iterator`].
    fn locate(self) -> LocatedIterator<Self>
    where
        Self: Sized,
    {
        LocatedIterator::new(self)
    }
}

impl<T: Iterator> IteratorLocationExt for T {}
