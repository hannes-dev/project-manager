use anyhow::Result;

pub trait Write<T> {
    fn write(&self, thing: T) -> Result<()>;
}
