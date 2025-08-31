pub trait Action {
	type AppEvent;

	fn app_event(&self) -> Self::AppEvent;
}
