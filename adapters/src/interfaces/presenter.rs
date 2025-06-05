pub trait Presenter<T> {
    type PresentModel;

    fn present_value(&self, data: T) -> Self::PresentModel;
}
