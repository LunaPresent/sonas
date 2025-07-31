use crate::app::component::{Component, CounterComponent, Ref};

#[derive(Debug, Default)]
pub struct RootComponent {
	counter: CounterComponent,
}

impl Component for RootComponent {
	fn children(&mut self) -> impl Iterator<Item = Ref> {
		std::iter::once(Ref::from(&mut self.counter as *mut _))
	}

	fn follow_focus<'a>(&'a mut self) -> super::FollowResult {
		super::FollowResult::Propagate(Ref::from(&mut self.counter as *mut _))
	}
}
