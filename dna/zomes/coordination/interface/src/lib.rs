


pub trait Descriptor {
    fn new(&self) -> Self;
    fn map_link(&self) -> Self;
    fn update(&self) -> Self;

}