use std::fmt;

use tokio::sync::watch::{channel, error::SendError, Receiver, Ref, Sender};

pub struct SwapLock<T: Clone> {
	r: Receiver<T>,
	s: Sender<T>,
}

impl<T> SwapLock<T>
where
	T: Clone,
{
	pub fn new(inner: T) -> Self {
		let (s, r) = channel(inner);
		Self { r, s }
	}

	pub fn borrow(&self) -> Ref<'_, T> {
		self.r.borrow()
	}

	pub async fn change(&self, f: impl FnOnce(&mut T)) -> Result<(), SendError<T>> {
		let mut new = self.r.borrow().clone();
		f(&mut new);
		self.s.send(new)
	}

	pub async fn replace(&self, new: T) -> Result<(), SendError<T>> {
		self.s.send(new)
	}
}

impl<T> fmt::Debug for SwapLock<T>
where
	T: fmt::Debug + Clone,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		f.debug_struct("SwapLock")
			.field("(watch)", &self.r)
			.finish()
	}
}
