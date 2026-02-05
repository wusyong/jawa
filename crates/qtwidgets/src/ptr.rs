use std::{
    cmp::Ordering,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    mem,
    ops::{Deref, DerefMut},
    pin::Pin,
    ptr,
};

use cxx::{UniquePtr, memory::UniquePtrTarget};

/// A pointer wrapper of QWidget to make sure the ownership is properly transferred.
///
/// The ownership of QWidget will pass to parent if it has one. We want to make sure we doesn't
/// hold the ownership when it's added to parent, while still keep it when it's top-level widget.
/// Please make sure to construct the correct variant when adding any QWidget bindings.
///
/// # Safety
///
/// This type only ensures the ownership transfer properly. The user still need to ensure that
/// the pointer is valid when using the raw pointer variant since it's nearly impossible to check
/// when will all ancestor widgets be dropped or its related widgets transferred to other widgets.
#[derive(Debug)]
pub enum WidgetPtr<T: UniquePtrTarget> {
    Unique(UniquePtr<T>),
    Raw(*mut T),
}

impl<T> WidgetPtr<T>
where
    T: UniquePtrTarget,
{
    /// Checks whether the WidgetPtr does not own an object.
    pub fn is_null(&self) -> bool {
        self.as_ptr().is_null()
    }

    pub fn is_unique(&self) -> bool {
        matches!(self, WidgetPtr::Unique(_))
    }

    /// Returns a reference to the object owned by this WidgetPtr if any,
    /// otherwise None.
    pub fn as_ref(&self) -> Option<&T> {
        let ptr = self.as_ptr();
        unsafe { ptr.as_ref() }
    }

    /// Returns a mutable pinned reference to the object owned by this WidgetPtr
    /// if any, otherwise None.
    pub fn as_mut(&mut self) -> Option<Pin<&mut T>> {
        let ptr = self.as_mut_ptr();
        unsafe {
            let mut_reference = ptr.as_mut()?;
            Some(Pin::new_unchecked(mut_reference))
        }
    }

    /// Returns a mutable pinned reference to the object owned by this
    /// WidgetPtr.
    ///
    /// # Panics
    ///
    /// Panics if the WidgetPtr holds a null pointer.
    pub fn pin_mut(&mut self) -> Pin<&mut T> {
        match self.as_mut() {
            Some(target) => target,
            None => panic!(
                "called pin_mut on a null WidgetPtr<{}>",
                display(T::__typename),
            ),
        }
    }

    /// Returns a raw const pointer to the object owned by this WidgetPtr if
    /// any, otherwise the null pointer.
    pub fn as_ptr(&self) -> *const T {
        match self {
            WidgetPtr::Unique(ptr) => ptr.as_ptr(),
            WidgetPtr::Raw(ptr) => ptr.cast_const(),
        }
    }

    /// Returns a raw mutable pointer to the object owned by this WidgetPtr if
    /// any, otherwise the null pointer.
    pub fn as_mut_ptr(&self) -> *mut T {
        self.as_ptr().cast_mut()
    }

    /// Releases ownership of the WidgetPtr, making it a raw pointer. Use this whenever the widget
    /// is added to a parent, so that the ownership is properly transferred.
    pub fn cast_raw(&mut self) {
        if self.is_unique() {
            let raw_ptr = match mem::replace(self, WidgetPtr::Raw(ptr::null_mut())) {
                WidgetPtr::Unique(unique_ptr) => unique_ptr.into_raw(),
                WidgetPtr::Raw(_) => unreachable!(),
            };
            *self = WidgetPtr::Raw(raw_ptr);
        }
    }
}

impl<T: UniquePtrTarget> From<UniquePtr<T>> for WidgetPtr<T> {
    fn from(value: UniquePtr<T>) -> Self {
        Self::Unique(value)
    }
}

impl<T: UniquePtrTarget> From<*mut T> for WidgetPtr<T> {
    fn from(value: *mut T) -> Self {
        Self::Raw(value)
    }
}

unsafe impl<T> Send for WidgetPtr<T> where T: Send + UniquePtrTarget {}
unsafe impl<T> Sync for WidgetPtr<T> where T: Sync + UniquePtrTarget {}

// WidgetPtr is not a self-referential type and is safe to move out of a Pin,
// regardless whether the pointer's target is Unpin.
impl<T> Unpin for WidgetPtr<T> where T: UniquePtrTarget {}

impl<T> Deref for WidgetPtr<T>
where
    T: UniquePtrTarget,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self.as_ref() {
            Some(target) => target,
            None => panic!(
                "called deref on a null WidgetPtr<{}>",
                display(T::__typename),
            ),
        }
    }
}

impl<T> DerefMut for WidgetPtr<T>
where
    T: UniquePtrTarget + Unpin,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self.as_mut() {
            Some(target) => Pin::into_inner(target),
            None => panic!(
                "called deref_mut on a null UniquePtr<{}>",
                display(T::__typename),
            ),
        }
    }
}

impl<T> Display for WidgetPtr<T>
where
    T: Display + UniquePtrTarget,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self.as_ref() {
            None => formatter.write_str("nullptr"),
            Some(value) => Display::fmt(value, formatter),
        }
    }
}

impl<T> PartialEq for WidgetPtr<T>
where
    T: PartialEq + UniquePtrTarget,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T> Eq for WidgetPtr<T> where T: Eq + UniquePtrTarget {}

impl<T> PartialOrd for WidgetPtr<T>
where
    T: PartialOrd + UniquePtrTarget,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.as_ref(), &other.as_ref())
    }
}

impl<T> Ord for WidgetPtr<T>
where
    T: Ord + UniquePtrTarget,
{
    fn cmp(&self, other: &Self) -> Ordering {
        Ord::cmp(&self.as_ref(), &other.as_ref())
    }
}

impl<T> Hash for WidgetPtr<T>
where
    T: Hash + UniquePtrTarget,
{
    fn hash<H>(&self, hasher: &mut H)
    where
        H: Hasher,
    {
        self.as_ref().hash(hasher);
    }
}

fn display(fmt: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl Display {
    DisplayInvoke(fmt)
}

struct DisplayInvoke<T>(T);

impl<T> Display for DisplayInvoke<T>
where
    T: Fn(&mut fmt::Formatter) -> fmt::Result,
{
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(formatter)
    }
}
