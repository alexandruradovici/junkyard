use std::{env, fs, sync::Mutex};

use anyhow::Result;
use exports::junkyard_vfs::wasm_local_fs::vfs::Kind;
use wasmtime::{
    component::{Component, Linker, ResourceAny},
    Config, Engine, Store,
};
use wasmtime_wasi::{DirPerms, FilePerms, ResourceTable, WasiCtx, WasiView};

use vfs::{AbsolutePath, DirEntry, Vfs};

wasmtime::component::bindgen!({
    async: false,
    path: "wit/vfs.wit"
});

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
}

impl WasiView for WasmVfsState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}

fn to_vfs_kind(kind: Kind) -> vfs::Kind {
    match kind {
        Kind::File => vfs::Kind::File,
        Kind::Folder => vfs::Kind::Folder,
        Kind::Link => vfs::Kind::Link,
        Kind::Unknown => vfs::Kind::Unknown,
    }
}

pub struct WasmVfs {
    store: Mutex<Store<WasmVfsState>>,
    instance: Plugin,
    plugin: ResourceAny,
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
        let mut store = self.store.lock().unwrap_or_else(|err| err.into_inner());
        let stat = self
            .instance
            .junkyard_vfs_wasm_local_fs_vfs()
            .filesystem()
            .call_stat(&mut *store, self.plugin, path.as_str())?
            .map_err(|e| anyhow::Error::msg(e))?;
        Ok(vfs::Stat::new(to_vfs_kind(stat.kind), stat.size))
    }

    fn read_dir(&self, path: &AbsolutePath) -> vfs::VfsResult<Vec<DirEntry>> {
        let mut store = self.store.lock().unwrap_or_else(|err| err.into_inner());
        Ok(self
            .instance
            .junkyard_vfs_wasm_local_fs_vfs()
            .filesystem()
            .call_read_dir(&mut *store, self.plugin, path.as_str())?
            .map_err(|e| anyhow::Error::msg(e))?
            .into_iter()
            .map(|s| DirEntry::new(AbsolutePath::new(s)))
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
    let instance = Plugin::instantiate(&mut store, &component, &linker)?;
    let plugin = instance
        .junkyard_vfs_wasm_local_fs_vfs()
        .call_init(&mut store)?
        .unwrap();
    Ok(WasmVfs {
        store: Mutex::new(store),
        instance,
        plugin,
    })
}
