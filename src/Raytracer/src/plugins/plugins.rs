use std::error::Error;
use std::ffi::OsStr;
use std::fs;
use std::collections::HashMap;
use crate::plugins::dlloader::DLLoader;
use crate::interfaces::Primitives;

pub struct Plugins {
    libs: Vec<DLLoader>,
    pub primitives: HashMap<String, Box <dyn Primitives>>,
    pub lights: HashMap<String, Box <dyn Primitives>>
}

impl Plugins {
    pub fn new() -> Plugins {
        Plugins {
            libs: Vec::new(),
            primitives: HashMap::new(),
            lights: HashMap::new(),
        }
    }

    pub fn parse(&mut self, plugins_dir: &str) -> Result<(), Box<dyn Error>> {
        if !self.primitives.is_empty() || !self.lights.is_empty() {
            return Err("The primitives and lights vectors must be empty before running parser".into());
        }
        if let Ok(entries) = fs::read_dir(plugins_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            let plugin_dir = entry.path();
                            let plugin_dir_name = plugin_dir.file_name().unwrap().to_str().unwrap();
                            println!("Inspecting folder: {:?}", plugin_dir_name);
                            if let Ok(plugin_entries) = fs::read_dir(&plugin_dir) {
                                for plugin_entry in plugin_entries {
                                    if let Ok(plugin_entry) = plugin_entry {
                                        if let Some(extension) = plugin_entry.path().extension() {
                                            if extension == OsStr::new("so") {
                                                if let Ok(lib) = DLLoader::from_lib(plugin_entry.path()) {
                                                    self.libs.push(lib);
                                                }
                                                // Add plugin to primitives or lights
                                                if plugin_dir_name == "Primitives" {
                                                    println!("plugin: {}", plugin_entry.file_name().to_string_lossy());
                                                    match self.libs.last() {
                                                        Some(lib) => {
                                                            match lib.get_instance::<Box <dyn Primitives>>("entryPoint") {
                                                                Ok(instance) => {
                                                                    self.primitives.insert(plugin_entry.file_name().to_string_lossy().trim_end_matches(".so").to_string(), instance);
                                                                    if let Some(prim) = self.primitives.get("tmr") {

                                                                        let d: Box<dyn Primitives> = (*prim).clone();
                                                                    }
                                                                }
                                                                Err(_) => return Err(format!("No entry point found in {}", plugin_entry.file_name().to_string_lossy()).into())
                                                            }
                                                        }
                                                        None => return Err("Something wrong happened".into())
                                                    }
                                                } else if plugin_dir_name == "Lights" {
                                                    println!("plugin: {}", plugin_entry.file_name().to_string_lossy());
                                                }
                                            } else {
                                                return Err(format!("File should be a shared linked lib (*.so): {:?}", plugin_entry.path()).into());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if self.primitives.is_empty() && self.lights.is_empty() {
                return Err("No plugins found".into());
            }
            Ok(())
        } else {
            Err(format!("Cannot read directory {}", plugins_dir).into())
        }
    }
}
