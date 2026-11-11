//! Note Craft Plugins - Système de plugins avec WASM
//!
//! Ce module fournit:
//! - Gestion des plugins WASM
//! - Sandboxing avec wasmtime
//! - API pour les plugins

use wasmtime::{Engine, Instance, Module, Store, Val};

/// Structure principale du système de plugins
pub struct PluginSystem {
    engine: Engine,
}

impl Default for PluginSystem {
    fn default() -> Self {
        let engine = Engine::default();
        PluginSystem { engine }
    }
}

impl PluginSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Charge un plugin WASM
    pub fn load_plugin(&self, wasm_bytes: &[u8]) -> Result<Plugin, Box<dyn std::error::Error>> {
        let module = Module::from_binary(&self.engine, wasm_bytes)?;
        let mut store = Store::new(&self.engine, ());

        let instance = Instance::new(&mut store, &module, &[])?;

        Ok(Plugin { store, instance })
    }
}

/// Représente un plugin chargé
pub struct Plugin {
    store: Store<()>,
    instance: Instance,
}

impl Plugin {
    /// Appelle une fonction du plugin
    pub fn call(
        &mut self,
        func_name: &str,
        args: &[Val],
    ) -> Result<Vec<Val>, Box<dyn std::error::Error>> {
        let func = self
            .instance
            .get_func(&mut self.store, func_name)
            .ok_or("Function not found")?;

        let mut results = vec![Val::I32(0); args.len()];
        func.call(&mut self.store, args, &mut results)?;

        Ok(results)
    }
}

/// Tests de compilation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_system_creation() {
        let _system = PluginSystem::new();
        // PluginSystem créé avec succès
    }
}
