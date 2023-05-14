use std::error::Error;
use std::ffi::OsStr;
use libloading::{Library, Symbol};

pub struct DLLoader {
    library: Library,
}

impl DLLoader {
    pub fn from_lib<P: AsRef<OsStr>>(lib_path: P) -> Result<Self, Box<dyn Error>> {
        let library = unsafe { Library::new(lib_path) }?;
        Ok(Self { library })
    }

    pub fn get_instance<T>(&self, symbol: &str) -> Result<T, Box<dyn Error>> {
        let entry_point: Symbol<extern "C" fn() -> T> = unsafe { self.library.get(symbol.as_bytes()) }?;
        Ok(entry_point())
    }
    pub fn get_instance2<T>(&self, symbol: &str) -> Result<Symbol<extern "C" fn() -> T>, Box<dyn Error>> {
        let entry_point: Symbol<extern "C" fn() -> T> = unsafe { self.library.get(symbol.as_bytes()) }?;
        Ok(entry_point)
    }
}