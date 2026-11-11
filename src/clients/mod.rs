pub mod local;
pub mod remote;

pub trait ClientRunner: Send + 'static {
    fn run(&mut self);
}
