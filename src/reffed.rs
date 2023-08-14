/*! # Method-Directed Reference-to-Reference Conversion

The `std::convert` module provides traits for converting references from one
to another ([`AsRef`] and [`AsMut`]).

It is not always possible for the compiler to infer which reference type you're targeting.
These traits put the type parameter in the method instead of the trait,
so you can write [`.reffed::<T>()`][Reffed::reffed] for immutable references,
and [`.ref_muted::<T>()`][RefMuted::ref_muted] for mutable ones.

These traits are blanket-implemented on all types that have [`AsRef`] and [`AsMut`]
implementations.
*/

/// Allows calling [`AsRef`] inline.
pub trait Reffed {
	/// Converts a reference to `Self` into a reference to `T`.
	///
	/// # Examples
	/// ```rust
	/// use tap::reffed::Reffed;
	/// use std::path::Path;
	///
	/// let path_as_string = "/some/sort/of/path";
	/// let displayable_path = path_as_string.reffed::<Path>()
	///     .display();
	///
	/// assert_eq!(displayable_path.to_string(), path_as_string);
	/// ```
	fn reffed<T>(&self) -> &T
	where
		Self: AsRef<T>,
		T: ?Sized;
}

impl<T> Reffed for T
where
	T: ?Sized,
{
	fn reffed<U>(&self) -> &U
	where
		T: AsRef<U>,
		U: ?Sized,
	{
		self.as_ref()
	}
}

/// Allows calling [`AsMut`] inline.
pub trait RefMuted {
	/// Converts a mutable reference to `Self` into a mutable reference to `T`.
	///
	/// # Examples
	/// ```rust
	/// use tap::reffed::RefMuted;
	///
	/// // This seems contrived because there's nothing
	/// // in the rust stdlib that implements
	/// // `AsMut` but not `DerefMut`.
	/// let mut ascii_hi = vec![104, 105];
	/// ascii_hi.ref_muted::<[u8]>().make_ascii_uppercase();
	///
	/// assert_eq!(std::str::from_utf8(&ascii_hi).unwrap(), "HI");
	/// ```
	fn ref_muted<T>(&mut self) -> &mut T
	where
		Self: AsMut<T>,
		T: ?Sized;
}

impl<T> RefMuted for T
where
	T: ?Sized,
{
	fn ref_muted<U>(&mut self) -> &mut U
	where
		T: AsMut<U>,
		U: ?Sized,
	{
		self.as_mut()
	}
}
