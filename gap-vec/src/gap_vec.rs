use std::fmt;
use std::mem::ManuallyDrop;

/// GapVec is a random access container with constant-time indexed removal.
#[derive(Debug)]
pub struct GapVec<T> {
    vec: Vec<MaybeValue<T>>,
    len: usize,
    free: Option<usize>,
}

union MaybeValue<T> {
    value: ManuallyDrop<T>,
    next_free: Option<usize>,
}

impl<T> GapVec<T> {
    #[inline]
    pub fn new() -> Self {
        GapVec {
            vec: Vec::new(),
            len: 0,
            free: None,
        }
    }

    pub fn push(&mut self, value: T) {
        let value = MaybeValue {
            value: ManuallyDrop::new(value),
        };
        if let Some(free) = self.free {
            debug_assert!(self.len < self.vec.len());
            self.free = unsafe { self.vec[free].next_free };
            self.vec[free] = value;
        } else {
            self.vec.push(value);
        }
        self.len += 1;
    }

    /// SAFETY: the caller must ensure that the value at `index` is initialized.
    pub unsafe fn drop(&mut self, index: usize) {
        debug_assert!(self.len > 0, "drop when empty");
        ManuallyDrop::drop(&mut self.vec[index].value);
        self.vec[index] = MaybeValue {
            next_free: self.free,
        };
        self.free = Some(index);
        self.len -= 1;
    }

    /// SAFETY: the caller must ensure that `index` is in bounds and points to
    /// an initialized value.
    #[inline]
    pub unsafe fn get(&self, index: usize) -> &T {
        debug_assert!(index < self.vec.len());
        &self.vec.get_unchecked(index).value
    }

    /// SAFETY: the caller must ensure that `index` is in bounds and points to
    /// an initialized value.
    #[inline]
    pub unsafe fn get_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < self.vec.len());
        &mut self.vec.get_unchecked_mut(index).value
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> fmt::Debug for MaybeValue<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MaybeValue {{ value: {:?}, next_free: {:?} }}",
            unsafe { &self.value },
            unsafe { self.next_free }
        )
    }
}
