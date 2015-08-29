//! A generic, fixed-size ring buffer.

use std::iter::Zip;
use std::ops::{Index, IndexMut};
use std::vec::Vec;

use types::Time;


/// A generic ring buffer.
///
/// A ring buffer can continue appending data to itself indefinitely. However,
/// it has a limited capacity; when that capacity is reached, it will overwrite
/// the oldest data.
///
/// The buffer is indexed with `Time` values. As data is overwritten, the
/// times stored in the buffer continue to slide forward.
///
/// # Initialization from slices
///
/// Sometimes, it helps to be able to specify the initial state of a ring
/// buffer. One example of this is to create a silent buffer of audio that
/// delayed samples can be accumulated in, such as may be used to generate
/// delays.
///
/// To make this easier, a buffer can be created from a slice. When created from
/// a slice, the buffer will be have a capacity that matches the size of the
/// slice, and the values of the slice will be inserted into the buffer,
/// starting at time `t=0`.
///
/// ```
/// # use oxcable::utils::ringbuffer::RingBuffer;
/// let values = ["first", "second", "third"];
/// let rb = RingBuffer::from(&values[..]);
/// assert_eq!(rb.capacity(), 3);
/// assert_eq!(rb[0], "first");
/// assert_eq!(rb[1], "second");
/// assert_eq!(rb[2], "third");
/// ```
///
/// # Example
///
/// ```
/// # use oxcable::utils::ringbuffer::RingBuffer;
/// // Create a new buffer that holds 4 elements...
/// let mut rb = RingBuffer::new(4);
///
/// // Insert 3 elements...
/// rb.push(1);
/// rb.push(2);
/// rb.push(3);
/// assert_eq!(rb.len(), 3);
/// assert_eq!(rb.get(0), Some(&1));
/// assert_eq!(rb.get(1), Some(&2));
/// assert_eq!(rb.get(2), Some(&3));
///
/// // Insert two more elements. This will push the oldest element out...
/// rb.push(4);
/// rb.push(5);
/// assert_eq!(rb.len(), 4);
/// assert_eq!(rb.get(0), None);
/// assert_eq!(rb.get(4), Some(&5));
///
/// // Pop the oldest element off the buffer...
/// assert_eq!(rb.get(1), Some(&2));
/// assert_eq!(rb.pop(), Some(2));
/// assert_eq!(rb.get(1), None);
/// assert_eq!(rb.len(), 3);
/// ```
#[derive(Clone, Debug)]
pub struct RingBuffer<T> {
    buf: Vec<T>,
    capacity: usize,
    start_i: usize,
    start_t: Time,
    end_t: Time,
}

impl<T> RingBuffer<T> {
    /// Returns an empty ring buffer that can hold at most `capacity` elements.
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buf: Vec::with_capacity(capacity),
            start_i: 0,
            capacity: capacity,
            start_t: 0,
            end_t: 0
        }
    }

    /// Returns the number of elements the ringbuffer currently contains.
    pub fn len(&self) -> usize {
        (self.end_t - self.start_t) as usize
    }

    /// Returns the number of elements the ringbuffer can hold at one time.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Returns the range of timestamps stored in the ringbuffer, as a tuple
    /// (first, last). First is inclusive, while last is exlusive.
    ///
    /// # Example
    ///
    /// ```
    /// # use oxcable::utils::ringbuffer::RingBuffer;
    /// let mut rb = RingBuffer::new(2);
    /// rb.push("first");  // t=0
    /// rb.push("second"); // t=1
    /// rb.push("third");  // t=2, first is overwritten
    /// assert_eq!(rb.times(), (1,3));
    /// ```
    pub fn times(&self) -> (Time, Time) {
        (self.start_t, self.end_t)
    }

    /// Attempts to return a ref to the data stored at time `t`. If the requested
    /// time is not in the buffer, instead returns `None`.
    pub fn get(&self, t: Time) -> Option<&T> {
        if self.start_t <= t && t < self.end_t {
            Some(&self[t])
        } else {
            None
        }
    }

    /// Attempts to return a mutable ref to the data stored at time `t`. If the
    /// requested time is not in the buffer, instead returns `None`.
    pub fn get_mut(&mut self, t: Time) -> Option<&mut T> {
        if self.start_t <= t && t < self.end_t {
            Some(&mut self[t])
        } else {
            None
        }
    }

    /// Pushes the supplied data onto the end of the buffer. If the buffer is
    /// full, this will overwrite the oldest data.
    pub fn push(&mut self, data: T) {
        if self.buf.len() < self.capacity {
            self.buf.push(data);
            self.end_t += 1;
        } else {
            let i = self.start_i + (self.end_t - self.start_t) as usize;
            self.buf[i % self.capacity] = data;
            self.start_i = (self.start_i+1) % self.capacity;
            self.start_t += 1;
            self.end_t += 1;
        }
    }

    /// Clears all the elements from the buffer.
    pub fn clear(&mut self) {
        self.start_t = self.end_t;
    }

    /// Returns an iterator over the buffer.
    /// 
    /// # Example
    ///
    /// ```
    /// # use oxcable::utils::ringbuffer::RingBuffer;
    /// let mut rb = RingBuffer::new(2);
    /// rb.push("first");
    /// rb.push("second");
    /// rb.push("third");
    ///
    /// let values: Vec<_> = rb.iter().collect();
    /// assert_eq!(values, [&"second", &"third"]);
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter { buffer: self, t: self.start_t }
    }

    /// Returns an iterator over the buffer's timestamps.
    /// 
    /// # Example
    ///
    /// ```
    /// # use oxcable::utils::ringbuffer::RingBuffer;
    /// let mut rb = RingBuffer::new(2);
    /// rb.push("first");
    /// rb.push("second");
    /// rb.push("third");
    ///
    /// let times: Vec<_> = rb.timestamp_iter().collect();
    /// assert_eq!(times, [1, 2]);
    /// ```
    pub fn timestamp_iter(&self) -> IterTimes {
        IterTimes { t: self.start_t, end: self.end_t }
    }

    /// Returns an iterator that pairs values with their timestamp.
    ///
    /// Equivalent to `self.timestamp_iter().zip(self.iter())`.
    /// 
    /// # Example
    ///
    /// ```
    /// # use oxcable::utils::ringbuffer::RingBuffer;
    /// let mut rb = RingBuffer::new(2);
    /// rb.push("first");
    /// rb.push("second");
    /// rb.push("third");
    ///
    /// let zipped: Vec<_> = rb.zipped_iter().collect();
    /// assert_eq!(zipped, [(1, &"second"), (2, &"third")]);
    /// ```
    pub fn zipped_iter(&self) -> Zip<IterTimes, Iter<T>> {
        self.timestamp_iter().zip(self.iter())
    }
}

impl<T: Clone> RingBuffer<T> {
    /// Removes the oldest element from the ringbuffer and return it. If there
    /// are no elements, returns `None` insteads.
    pub fn pop(&mut self) -> Option<T> {
        if self.start_t == self.end_t {
            None
        } else {
            let result = self[self.start_t].clone();
            self.start_i = (self.start_i+1) % self.capacity;
            self.start_t += 1;
            Some(result)
        }
    }

    /// Resizes the ringbuffer to hold up to `capacity` elements. If the new
    /// capacity is smaller than the old one, then the oldest elements will be
    /// removed from the buffer.
    ///
    /// # Example
    ///
    /// ```
    /// # use oxcable::utils::ringbuffer::RingBuffer;
    /// let mut rb = RingBuffer::new(4);
    /// rb.push(1); rb.push(2); rb.push(3); rb.push(4);
    /// rb.resize(2);
    /// assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), [3,4]);
    /// ```
    pub fn resize(&mut self, capacity: usize) {
        let mut elems = (self.end_t - self.start_t) as usize;
        if capacity < elems {
            let drops = elems - capacity;
            self.start_i += drops;
            self.start_t += drops as Time;
            elems = capacity;
        }

        let mut new_buf = Vec::with_capacity(capacity);
        for i in self.start_i..(self.start_i+elems) {
            new_buf.push(self.buf[i % self.capacity].clone());
        }
        self.buf = new_buf;

        self.start_i = 0;
        self.capacity = capacity;
    }
}

impl<T> Index<Time> for RingBuffer<T> {
    type Output = T;
    fn index(&self, t: Time) -> &T {
        if t < self.start_t || t >= self.end_t {
            panic!("index out of bounds: buffer has range [{},{}), but index is {}",
                self.start_t, self.end_t, t);
        }
        let i = self.start_i + (t - self.start_t) as usize;
        &self.buf[i % self.capacity]
    }
}

impl<T> IndexMut<Time> for RingBuffer<T> {
    fn index_mut(&mut self, t: Time) -> &mut T {
        if t < self.start_t || t >= self.end_t {
            panic!("index out of bounds: buffer has range [{},{}), but index is {}",
                self.start_t, self.end_t, t);
        }
        let i = self.start_i + (t - self.start_t) as usize;
        &mut self.buf[i % self.capacity]
    }
}

impl<'a, T> From<&'a [T]> for RingBuffer<T> where T: Clone {
    /// Test
    fn from(s: &'a [T]) -> RingBuffer<T> {
        RingBuffer {
            buf: Vec::from(s),
            start_i: 0,
            capacity: s.len(),
            start_t: 0,
            end_t: s.len() as Time
        }
    }
}


/// An iterator over RingBuffer values.
pub struct Iter<'a, T: 'a> {
    buffer: &'a RingBuffer<T>,
    t: Time
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.t == self.buffer.end_t {
            None
        } else {
            let result = &self.buffer[self.t];
            self.t += 1;
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = (self.buffer.end_t - self.t) as usize;
        (exact, Some(exact))
    }

    fn count(self) -> usize {
        self.size_hint().0
    }
}


/// An iterator over RingBuffer timestamps.
pub struct IterTimes {
    t: Time,
    end: Time
}

impl Iterator for IterTimes {
    type Item = Time;
    fn next(&mut self) -> Option<Self::Item> {
        if self.t == self.end {
            None
        } else {
            let result = self.t;
            self.t += 1;
            Some(result)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = (self.end - self.t) as usize;
        (exact, Some(exact))
    }

    fn count(self) -> usize {
        self.size_hint().0
    }
}


#[cfg(test)]
mod tests {
    use super::RingBuffer;

    fn get_test_rb() -> RingBuffer<i32> {
        RingBuffer {
            buf: vec![7,13],
            capacity: 2,
            start_i: 1,
            start_t: 7,
            end_t: 9
        }
    }

    #[test]
    fn test_push() {
        let mut rb = RingBuffer::<i32>::new(2);

        rb.push(13);
        assert_eq!(rb.start_t, 0);
        assert_eq!(rb.end_t, 1);
        assert_eq!(rb.buf[0], 13);

        rb.push(7);
        assert_eq!(rb.start_t, 0);
        assert_eq!(rb.end_t, 2);
        assert_eq!(rb.buf[0], 13);
        assert_eq!(rb.buf[1], 7);

        rb.push(3);
        assert_eq!(rb.start_t, 1);
        assert_eq!(rb.end_t, 3);
        assert_eq!(rb.buf[0], 3);
        assert_eq!(rb.buf[1], 7);
    }

    #[test]
    fn test_pop() {
        let mut rb = get_test_rb();
        assert_eq!(rb.pop(), Some(13));
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), [7]);
        assert_eq!(rb.pop(), Some(7));
        assert_eq!(rb.pop(), None);
    }

    #[test]
    fn test_get() {
        // get is just a wrapper around Index, so just do some simple black box
        // testing; glass box is handled in test_index.
        let rb = get_test_rb();
        assert_eq!(rb.get(6), None);
        assert_eq!(rb.get(7), Some(&13));
        assert_eq!(rb.get(8), Some(&7));
        assert_eq!(rb.get(9), None);
    }

    #[test]
    fn test_get_mut() {
        // get_mut is just a wrapper around IndexMut, so just do some simple
        // black box testing; glass box is handled in test_index_mut.
        let mut rb = get_test_rb();
        assert_eq!(rb.get_mut(6), None);
        assert_eq!(rb.get_mut(7), Some(&mut 13));
        assert_eq!(rb.get_mut(8), Some(&mut 7));
        assert_eq!(rb.get_mut(9), None);
    }

    #[test]
    fn test_index() {
        let mut rb = get_test_rb();

        // Test with odd start
        assert_eq!(rb[7], 13);
        assert_eq!(rb[8], 7);

        // Test with even start
        rb.start_i = 0; rb.start_t = 6; rb.end_t = 8;
        assert_eq!(rb[6], 7);
        assert_eq!(rb[7], 13);
    }

    #[test]
    #[should_panic]
    fn test_index_under() {
        let rb = get_test_rb();
        rb[6];
    }

    #[test]
    #[should_panic]
    fn test_index_over() {
        let rb = get_test_rb();
        rb[9];
    }

    #[test]
    fn test_index_mut() {
        let mut rb = get_test_rb();

        rb[7] = 22;
        rb[8] = 23;
        assert_eq!(rb.buf[1], 22);
        assert_eq!(rb.buf[0], 23);
    }

    #[test]
    #[should_panic]
    fn test_index_mut_under() {
        let mut rb = get_test_rb();
        rb[6] = 3;
    }

    #[test]
    #[should_panic]
    fn test_index_mut_over() {
        let mut rb = get_test_rb();
        rb[9] = 3;
    }

    #[test]
    fn test_resize() {
        // Test expanding
        let mut rb = get_test_rb();
        rb.resize(4);
        rb.push(19);
        rb.push(21);
        rb.push(23);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), [7,19,21,23]);

        // Test contracting
        rb.resize(2);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), [21,23]);

        // Test expanding a partially full buffer
        let mut rb = RingBuffer::new(2);
        rb.push(1);
        rb.resize(4);
        rb.push(2);
        rb.push(3);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), [1,2,3]);

        // Test contracting a partially full buffer
        let mut rb = RingBuffer::new(4);
        rb.push(1);
        rb.resize(2);
        rb.push(2);
        rb.push(3);
        assert_eq!(rb.iter().cloned().collect::<Vec<_>>(), [2,3]);
    }
}
