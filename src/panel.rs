use std::{cmp::Ordering, sync::Arc};

use cursive::align::HAlign;
use cursive::views::Dialog;
use cursive_table_view::{TableView, TableViewItem};
use vfs::{AbsolutePath, Kind, Vfs, VfsResult};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Data {
    Name,
    Kind,
    Len,
}

impl AsRef<str> for Data {
    fn as_ref(&self) -> &str {
        match self {
            Data::Name => "Name",
            Data::Kind => "Kind",
            Data::Len => "Len",
        }
    }
}

#[derive(Clone)]
pub struct File {
    name: String,
    full_path: AbsolutePath,
    kind: Kind,
    len: u64,
}

impl File {
    pub fn new(name: String, full_path: AbsolutePath, kind: Kind, len: u64) -> File {
        File {
            name,
            full_path,
            kind,
            len,
        }
    }
}

impl TableViewItem<Data> for File {
    fn to_column(&self, column: Data) -> String {
        match column {
            Data::Name => self.name.clone(),
            Data::Kind => format!("{:?}", self.kind),
            Data::Len => format!("{}", self.len),
        }
    }

    fn cmp(&self, other: &Self, column: Data) -> std::cmp::Ordering
    where
        Self: Sized,
    {
        match column {
            Data::Name => self.name.cmp(&other.name),
            Data::Kind => self.kind.cmp(&other.kind),
            Data::Len => self.len.cmp(&other.len),
        }
    }
}

fn list_files(provider: &dyn Vfs, path: &AbsolutePath) -> VfsResult<Vec<File>> {
    let entries = provider.read_dir(&path)?;
    let mut files = entries
        .into_iter()
        .map(|entry| {
            let (kind, size) = if let Ok(stat) = provider.stat(&entry) {
                (stat.kind, stat.size)
            } else {
                (Kind::Unknown, 0)
            };
            File::new(entry.name().into(), entry, kind, size)
        })
        .collect::<Vec<_>>();
    if !path.is_root() {
        files.insert(
            0,
            File::new("..".to_string(), path.clone(), Kind::Folder, 0),
        );
    }
    Ok(files)
}

pub fn init_panel(
    id: impl AsRef<str>,
    provider: Arc<dyn Vfs>,
    path: AbsolutePath,
) -> TableView<File, Data> {
    let mut table = TableView::<File, Data>::new()
        .column(Data::Name, Data::Name.as_ref(), |c| c.width_percent(60))
        .column(Data::Kind, Data::Kind.as_ref(), |c| c.align(HAlign::Center))
        .column(Data::Len, Data::Len.as_ref(), |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        });
    let items = list_files(provider.as_ref(), &path).unwrap_or(vec![File::new(
        "..".to_string(),
        path.clone(),
        Kind::Folder,
        0,
    )]);
    let table_id = id.as_ref().to_string();
    let vfs = provider.clone();
    table.set_on_submit(move |siv, _row, index| {
        let sink = siv.cb_sink().clone();
        siv.call_on_name(&table_id, |table: &mut TableView<File, Data>| {
            let file = table.borrow_item(index).unwrap();
            match file.kind {
                Kind::Folder => {
                    // let path = PathBuf::from(&file.full_path);
                    let next_path = if file.name == ".." {
                        &file.full_path.parent()
                    } else {
                        &file.full_path
                    };
                    let items = list_files(vfs.as_ref(), &next_path).unwrap_or_else(|err| {
                        // siv.add_layer(Dialog::info("error"));
                        sink.send(Box::new(move |siv| {
                            siv.add_layer(Dialog::info(format!(
                                "Filed to read folder contexts: {}",
                                err
                            )));
                        }))
                        .unwrap();
                        vec![File::new(
                            "..".to_string(),
                            file.full_path.clone(),
                            Kind::Folder,
                            0,
                        )]
                    });
                    let selected_index = if file.name == ".." {
                        if let Some(folder) = file.full_path.components().last() {
                            items.iter().position(|f| f.name.as_str() == *folder)
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    table.set_items(items);
                    if let Some(item_index) = selected_index {
                        table.set_selected_item(item_index);
                    }
                }
                _ => {}
            }
        });
    });
    table.set_items(items);
    table
}
