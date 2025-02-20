use anyhow::Result;
use std::sync::MutexGuard;
use std::{env, fs, sync::Mutex};
use vfs::Vfs;
use wasm_vfs_api::{
    junkyard_vfs::vfs_plugin::vfs_host::{Host, HostAbsolutePath},
    VfsPlugin, AbsolutePath
};
use wasmtime::{
    component::{Component, Linker, Resource, ResourceAny},
    Config, Engine, Store,
};
use wasmtime_wasi::{DirPerms, FilePerms, ResourceTable, WasiCtx, WasiView};

struct WasmVfsState {
    ctx: WasiCtx,
    table: ResourceTable,
}

impl WasmVfsState {
    fn new(root: &AbsolutePath) -> WasmVfsState {
        WasmVfsState {
            ctx: WasiCtx::builder()
                .inherit_stdio()
                .inherit_args()
                .preopened_dir(root.as_str(), "/", DirPerms::READ, FilePerms::READ)
                .unwrap()
                .build(),
            table: ResourceTable::new(),
        }
    }

    fn create_absolute_path_resource(&mut self, path: AbsolutePath) -> Resource<AbsolutePath> {
        self.table.push(path).unwrap()
    }

    fn take_absolute_path(&mut self, path: Resource<AbsolutePath>) -> Option<AbsolutePath> {
        self.table.delete(path).ok()
    }
}

impl WasiView for WasmVfsState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

pub struct WasmVfs {
    store: Mutex<Store<WasmVfsState>>,
    instance: VfsPlugin,
    vfs_plugin: ResourceAny,
}

impl WasmVfs {
    fn get_store(&self) -> MutexGuard<'_, Store<WasmVfsState>> {
        self.store.lock().unwrap_or_else(|s| s.into_inner())
    }
}

impl HostAbsolutePath for WasmVfsState {
    fn components(&mut self, self_: Resource<AbsolutePath>) -> Vec<String> {
        self.table.get(&self_)
            .as_ref()
            .map(|path| {
                path.components()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap()
    }

    fn is_root(&mut self, self_: Resource<AbsolutePath>) -> bool {
        self.table.get(&self_)
            .as_ref()
            .map(|path| path.is_root())
            .unwrap()
    }

    fn parent(&mut self, self_: Resource<AbsolutePath>) -> Resource<AbsolutePath> {
        self.create_absolute_path_resource(
            self.table.get(&self_)
                .as_ref()
                .map(|path| path.parent())
                .unwrap(),
        )
    }

    fn file_name(&mut self, self_: Resource<AbsolutePath>) -> String {
        self.table.get(&self_)
            .as_ref()
            .map(|path| path.name().to_string())
            .unwrap()
    }

    fn path(&mut self, self_: Resource<AbsolutePath>) -> String {
        self.table.get(&self_)
            .as_ref()
            .map(|path| path.path().to_string())
            .unwrap()
    }

    fn drop(&mut self, rep: Resource<AbsolutePath>) -> wasmtime::Result<()> {
        self.take_absolute_path(rep);
        Ok(())
    }
}

impl Host for WasmVfsState {
    fn create_absolute_path(&mut self, s: String) -> Resource<AbsolutePath> {
        self.create_absolute_path_resource(AbsolutePath::new(s))
    }
}

impl Vfs for WasmVfs {
    fn open(
        &self,
        _path: &AbsolutePath,
        _open_options: vfs::OpenOptions,
    ) -> vfs::VfsResult<Box<dyn vfs::File>> {
        todo!()
    }

    fn unlink(&self, _path: &AbsolutePath) -> vfs::VfsResult<()> {
        todo!()
    }

    fn stat(&self, path: &AbsolutePath) -> vfs::VfsResult<vfs::Stat> {
        let mut store = self.get_store();
        let path = store.data_mut().create_absolute_path_resource(path.clone());
        let ret = self
            .instance
            .junkyard_vfs_vfs_plugin_vfs()
            .filesystem()
            .call_stat(
                &mut *store,
                self.vfs_plugin,
                Resource::<AbsolutePath>::new_borrow(path.rep())
            );
        store.data_mut().take_absolute_path(path);
        Ok(ret.map_err(|e| e.to_string())??)
    }

    fn read_dir(&self, path: &AbsolutePath) -> vfs::VfsResult<Vec<AbsolutePath>> {
        let mut store = self.get_store();
        let path = store.data_mut().create_absolute_path_resource(path.clone());
        let ret = self
            .instance
            .junkyard_vfs_vfs_plugin_vfs()
            .filesystem()
            .call_read_dir(
                &mut *store,
                self.vfs_plugin,
                Resource::<AbsolutePath>::new_borrow(path.rep()),
            );
        store.data_mut().take_absolute_path(path);
        Ok(ret
            .map_err(|e| e.to_string())??
            .into_iter()
            .map(|s| store.data_mut().take_absolute_path(s).unwrap())
            .collect())
    }

    fn create_dir(&self, _path: &AbsolutePath) -> vfs::VfsResult<()> {
        todo!()
    }

    fn create_dir_all(&self, _path: &AbsolutePath) -> vfs::VfsResult<()> {
        todo!()
    }

    fn rename(&self, _from: &AbsolutePath, _to: &AbsolutePath) -> vfs::VfsResult<()> {
        todo!()
    }
}

pub fn load_wasm_vfs(path: &AbsolutePath, root: &AbsolutePath) -> Result<WasmVfs> {
    let hash = sha256::digest(fs::read(path.as_str())?);
    let mut loaded_path = env::home_dir().unwrap();
    loaded_path.push(format!(".junkyard/plugin_vfs_{}", hash));

    let mut config = Config::default();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let mut linker: Linker<WasmVfsState> = Linker::new(&engine);
    let component = if !fs::exists(&loaded_path)? {
        let component_binary = Component::from_file(&engine, path.as_str())?;
        fs::create_dir_all(&loaded_path.parent().unwrap()).unwrap();
        fs::write(&loaded_path, component_binary.serialize().unwrap()).unwrap();
        component_binary
    } else {
        unsafe { Component::deserialize_file(&engine, &loaded_path).unwrap() }
    };
    wasmtime_wasi::add_to_linker_sync(&mut linker)?;
    let state = WasmVfsState::new(root);
    let mut store = Store::new(&engine, state);
    VfsPlugin::add_to_linker(&mut linker, |s| s)?;
    let instance = VfsPlugin::instantiate(&mut store, &component, &linker)?;
    let vfs_plugin = instance
        .junkyard_vfs_vfs_plugin_vfs()
        .call_init(&mut store)?
        .unwrap();
    Ok(WasmVfs {
        store: Mutex::new(store),
        instance,
        vfs_plugin,
    })
}
