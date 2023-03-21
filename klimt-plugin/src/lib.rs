use downcast_rs::{impl_downcast, Downcast};
use libloading::{Library, Symbol};
use std::any::Any;
use std::ffi::OsStr;
use thiserror::Error;

pub trait Plugin: Downcast + Any + Send + Sync {
    fn print_name(&self);
}

impl_downcast!(Plugin);

pub type CreatePlugin = unsafe fn() -> *mut dyn Plugin;

#[derive(Debug, Error)]
pub enum DynamicPluginLoadError {
    #[error("cannot load library for dynamic plugin: {0}")]
    Library(libloading::Error),
    #[error("dynamic library does not contain a valid Bevy dynamic plugin")]
    Plugin(libloading::Error),
}

pub unsafe fn dynamically_load_plugin<P: AsRef<OsStr>>(
    path: P,
) -> Result<(Library, Box<dyn Plugin>), DynamicPluginLoadError> {
    let lib = Library::new(path).map_err(DynamicPluginLoadError::Library)?;
    let func: Symbol<CreatePlugin> = lib
        .get(b"_kaantor_create_plugin")
        .map_err(DynamicPluginLoadError::Plugin)?;
    let plugin = Box::from_raw(func());
    Ok((lib, plugin))
}
