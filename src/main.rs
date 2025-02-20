use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use cursive::{
    menu,
    theme::BorderStyle,
    view::{Nameable, Resizable},
    views::{Dialog, FixedLayout, OnLayoutView, Panel},
    Rect, View, With,
};
use local_fs::LocalFileSystem;
use panel::init_panel;
use vfs::AbsolutePath;
use wasm::load_wasm_vfs;

mod panel;

fn main() {
    let mut siv = cursive::default();

    siv.load_toml(include_str!("../visual.toml")).unwrap();

    siv.set_autorefresh(true);

    // let provider_left = Arc::new(LocalFileSystem {});
    let provider_left = Arc::new(load_wasm_vfs(&AbsolutePath::new("/Users/alexandru/programe/Wylio/junkyard/packages/filesystem/wasm_local_js_fs/component.wasm"), &AbsolutePath::new("/")).unwrap());
    // let provider_right = Arc::new(LocalFileSystem {});
    let provider_right = Arc::new(load_wasm_vfs(&AbsolutePath::new("/Users/alexandru/programe/Wylio/junkyard/target/wasm32-wasip2/release/wasm_local_fs.wasm"), &AbsolutePath::new("/")).unwrap());

    let left = init_panel("left", provider_left, AbsolutePath::from("/"));
    let right = init_panel("right", provider_right, AbsolutePath::from("/"));

    let mut no_shadow_theme = siv.current_theme().clone();
    no_shadow_theme.shadow = false; // Disable shadow
                                    // no_shadow_theme.borders = BorderStyle::None; // Remove border effect
    no_shadow_theme.borders = BorderStyle::Simple;
    siv.set_theme(no_shadow_theme); // Apply no-shadow theme

    siv.add_fullscreen_layer(
        OnLayoutView::new(
            FixedLayout::new()
                .child(
                    Rect::from_size((0, 0), (0, 0)),
                    Panel::new(left.with_name("left").min_size((30, 20))).title("Left"),
                )
                .child(
                    Rect::from_size((0, 0), (0, 0)),
                    Panel::new(right.with_name("right").min_size((30, 20))).title("Right"),
                ),
            |layout, size| {
                if size.x > 1 && size.y > 2 {
                    layout.set_child_position(0, Rect::from_size((0, 0), (size.x / 2, size.y - 2)));
                    layout.set_child_position(
                        1,
                        Rect::from_size((size.x / 2 + 1, 0), (size.x / 2, size.y - 2)),
                    );
                }
                layout.layout(size);
                // eprintln!("{:?}", size);
            },
        )
        .full_screen(),
    );

    // let counter = AtomicUsize::new(1);
    // // The menubar is a list of (label, menu tree) pairs.
    // siv.menubar()
    //     // We add a new "File" tree
    //     .add_subtree(
    //         "File",
    //         menu::Tree::new()
    //             // Trees are made of leaves, with are directly actionable...
    //             .leaf("New", move |s| {
    //                 // Here we use the counter to add an entry
    //                 // in the list of "Recent" items.
    //                 let i = counter.fetch_add(1, Ordering::Relaxed);
    //                 let filename = format!("New {i}");
    //                 s.menubar()
    //                     .find_subtree("File")
    //                     .unwrap()
    //                     .find_subtree("Recent")
    //                     .unwrap()
    //                     .insert_leaf(0, filename, |_| ());

    //                 s.add_layer(Dialog::info("New file!"));
    //             })
    //             // ... and of sub-trees, which open up when selected.
    //             .subtree(
    //                 "Recent",
    //                 // The `.with()` method can help when running loops
    //                 // within builder patterns.
    //                 menu::Tree::new().with(|tree| {
    //                     for i in 1..100 {
    //                         // We don't actually do anything here,
    //                         // but you could!
    //                         tree.add_item(menu::Item::leaf(format!("Item {i}"), |_| ()).with(|s| {
    //                             if i % 5 == 0 {
    //                                 s.disable();
    //                             }
    //                         }))
    //                     }
    //                 }),
    //             )
    //             // Delimiter are simple lines between items,
    //             // and cannot be selected.
    //             .delimiter()
    //             .with(|tree| {
    //                 for i in 1..10 {
    //                     tree.add_leaf(format!("Option {i}"), |_| ());
    //                 }
    //             }),
    //     )
    //     .add_subtree(
    //         "Help",
    //         menu::Tree::new()
    //             .subtree(
    //                 "Help",
    //                 menu::Tree::new()
    //                     .leaf("General", |s| s.add_layer(Dialog::info("Help message!")))
    //                     .leaf("Online", |s| {
    //                         let text = "Google it yourself!\n\
    //                                     Kids, these days...";
    //                         s.add_layer(Dialog::info(text))
    //                     }),
    //             )
    //             .leaf("About", |s| s.add_layer(Dialog::info("Cursive v0.0.0"))),
    //     )
    //     .add_delimiter()
    //     .add_leaf("Quit", |s| s.quit());

    // siv.set_autohide_menu(false);

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
