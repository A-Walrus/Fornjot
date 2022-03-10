use std::{
    fmt,
    hash::{Hash, Hasher},
    ops::Deref,
    rc::Rc,
};

/// A handle to an object stored within [`Shape`]
///
/// If an object of type `T` (this could be `Curve`, `Vertex`, etc.) is added to
/// `Shape`, a `Handle<T>` is returned. This handle is then used in topological
/// types to refer to the object, instead of having those types own an instance
/// of the object.
///
/// This approach has two advantages:
///
/// 1. The object can't be mutated through the handle. Since an object can be
///    referred to by multiple other objects, mutating it locally would have no
///    effect on those other references. `Handle` preventing that removes this
///    source of errors.
/// 2. The object is guaranteed to be in `Shape`, as `Handle`s can't be created
///    any other way. This means that if the `Shape` needs to be modified, any
///    objects can be updated once, without requiring an update of all the other
///    objects that reference it.
///
/// # Equality
///
/// The equality of [`Handle`] is very strictly defined in terms of identity.
/// Two [`Handle`]s are considered equal, if they refer to objects in the same
/// memory location.
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Handle<T>(Storage<T>);

impl<T> Handle<T> {
    /// Access the object that the handle references
    ///
    /// `Handle` also implements `Deref`, but as that can be inconvenient to use
    /// in some cases, this method is an inherent proxy for that.
    pub fn get(&self) -> &T {
        self.0.deref()
    }

    /// Internal method to access the [`Storage`] this handle refers to
    pub(super) fn storage(&self) -> &Storage<T> {
        &self.0
    }
}

impl<T> Deref for Handle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<T> fmt::Debug for Handle<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.0.ptr(), self.get())
    }
}

/// Internal type used in collections within [`Shape`]
#[derive(Debug, Eq, Ord, PartialOrd)]
pub(super) struct Storage<T>(Rc<T>);

impl<T> Storage<T> {
    /// Create a [`Storage`] instance that wraps the provided object
    pub(super) fn new(value: T) -> Self {
        Self(Rc::new(value))
    }

    /// Create a handle that refers to this [`Storage`] instance
    pub(super) fn handle(&self) -> Handle<T> {
        Handle(self.clone())
    }

    fn ptr(&self) -> *const T {
        Rc::as_ptr(&self.0)
    }
}

impl<T> Deref for Storage<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

// Deriving `Clone` would only derive `Clone` where `T: Clone`. This
// implementation doesn't have that limitation, providing `Clone` for all
// `Handle`s instead.
impl<T> Clone for Storage<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> PartialEq for Storage<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Hash for Storage<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ptr().hash(state);
    }
}
