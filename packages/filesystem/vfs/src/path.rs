use regex::Regex;

#[repr(transparent)]
#[derive(PartialEq, Debug, PartialOrd, Clone)]
pub struct AbsolutePath {
    path: String,
}

impl From<&str> for AbsolutePath {
    fn from(value: &str) -> Self {
        if AbsolutePath::is_normalized(value) {
            AbsolutePath {
                path: value.to_owned(),
            }
        } else {
            AbsolutePath {
                path: AbsolutePath::normalize(value),
            }
        }
    }
}

impl From<String> for AbsolutePath {
    fn from(value: String) -> Self {
        if AbsolutePath::is_normalized(&value) {
            AbsolutePath { path: value }
        } else {
            AbsolutePath {
                path: AbsolutePath::normalize(value),
            }
        }
    }
}

impl AbsolutePath {
    pub fn new(s: impl AsRef<str>) -> AbsolutePath {
        s.as_ref().into()
    }

    pub fn as_str(&self) -> &str {
        &self.path
    }

    pub fn components(&self) -> Vec<&str> {
        if self.is_root() {
            vec![]
        } else {
            // avoid the first slash
            self.path[1..].split('/').collect()
        }
    }

    pub fn is_root(&self) -> bool {
        self.path == "/"
    }

    pub fn name(&self) -> &str {
        // if components is empty,this means that the path is root
        self.components().last().unwrap_or(&"/")
    }

    pub fn parent(&self) -> AbsolutePath {
        if self.is_root() {
            self.clone()
        } else {
            let components = self.components();
            AbsolutePath::new(components.iter().as_slice()[..components.len() - 1].join("/"))
        }
    }
}

impl AbsolutePath {
    fn is_normalized(value: impl AsRef<str>) -> bool {
        let regex =
            Regex::new(r"^(?:/(?:(?:[^\./][^/]*)|(?:\.[^\.][^/]*)|(?:\.{2,}[^/]+)))+$").unwrap();
        regex.is_match(value.as_ref())
    }

    fn normalize(value: impl AsRef<str>) -> String {
        let parts = value.as_ref().split('/');
        let mut normalized_parts = vec![];
        for part in parts {
            match part {
                "." | "" => { /* ignore self references */ }
                ".." => {
                    normalized_parts.pop();
                }
                _ => normalized_parts.push(part),
            }
        }
        format!("/{}", normalized_parts.join("/"))
    }
}

#[cfg(test)]
mod tests {
    mod is_normalized {
        use crate::AbsolutePath;

        #[test]
        fn not_normalized_no_slash() {
            assert_eq!(AbsolutePath::is_normalized("folder/folder2"), false);
        }

        #[test]
        fn not_normalized_up_dir() {
            assert_eq!(AbsolutePath::is_normalized("/folder/../folder2"), false);
        }

        #[test]
        fn not_normalized_starts_with_dot() {
            assert_eq!(AbsolutePath::is_normalized("./folder/folder2"), false);
        }

        #[test]
        fn not_normalized_starts_with_dot_dot() {
            assert_eq!(AbsolutePath::is_normalized("../folder/folder2"), false);
        }

        #[test]
        fn not_normalized_ends_with_dot_dot() {
            assert_eq!(AbsolutePath::is_normalized("/folder/folder2/.."), false);
        }

        #[test]
        fn not_normalized_ends_with_dot() {
            assert_eq!(AbsolutePath::is_normalized("/folder/folder2/."), false);
        }

        #[test]
        fn not_normalized_ends_with_slash() {
            assert_eq!(AbsolutePath::is_normalized("/folder/folder2/"), false);
        }

        #[test]
        fn not_normalized_double_slash() {
            assert_eq!(AbsolutePath::is_normalized("/folder//folder2/"), false);
        }

        #[test]
        fn not_normalized_self() {
            assert_eq!(AbsolutePath::is_normalized("/folder/./folder2/"), false);
        }

        #[test]
        fn normalized_triple_dot() {
            assert!(AbsolutePath::is_normalized("/folder/.../folder2"));
        }

        #[test]
        fn normalized_dot_dot_name() {
            assert!(AbsolutePath::is_normalized("/folder/..folder2"));
        }

        #[test]
        fn normalized_hidden() {
            assert!(AbsolutePath::is_normalized("/folder/.folder2"));
        }
    }

    mod normalize {
        use crate::AbsolutePath;

        #[test]
        fn add_slash() {
            assert_eq!(AbsolutePath::normalize("folder/folder2"), "/folder/folder2");
        }

        #[test]
        fn no_trialing_slash() {
            assert_eq!(
                AbsolutePath::normalize("/folder/folder2/"),
                "/folder/folder2"
            );
        }

        #[test]
        fn no_double_slash() {
            assert_eq!(
                AbsolutePath::normalize("//folder/folder2"),
                "/folder/folder2"
            );
            assert_eq!(
                AbsolutePath::normalize("/folder//folder2"),
                "/folder/folder2"
            );
            assert_eq!(
                AbsolutePath::normalize("/folder/folder2//"),
                "/folder/folder2"
            );
            assert_eq!(
                AbsolutePath::normalize("//folder//folder2//"),
                "/folder/folder2"
            );
            assert_eq!(
                AbsolutePath::normalize("/folder//folder2//"),
                "/folder/folder2"
            );
        }

        #[test]
        fn parent_dir() {
            assert_eq!(AbsolutePath::normalize("/folder/.."), "/");
            assert_eq!(AbsolutePath::normalize("/folder/../"), "/");
            assert_eq!(AbsolutePath::normalize("folder/.."), "/");

            assert_eq!(AbsolutePath::normalize("folder/../../.."), "/");
        }

        #[test]
        fn current_dir() {
            assert_eq!(AbsolutePath::normalize("/folder/."), "/folder");
            assert_eq!(AbsolutePath::normalize("/folder/./"), "/folder");
            assert_eq!(AbsolutePath::normalize("folder/."), "/folder");

            assert_eq!(AbsolutePath::normalize("folder/././."), "/folder");
            assert_eq!(AbsolutePath::normalize("./folder/././."), "/folder");
        }

        #[test]
        fn hidden() {
            assert_eq!(AbsolutePath::normalize("/folder/.f"), "/folder/.f");
        }
    }

    mod absolute_path {
        use crate::path::AbsolutePath;

        #[test]
        fn simple() {
            let path = AbsolutePath::new("/folder");
            assert_eq!(path.as_str(), "/folder");
            assert_eq!(path.components().as_slice(), ["folder"]);

            let path = AbsolutePath::new("/folder/folder2");
            assert_eq!(path.as_str(), "/folder/folder2");
            assert_eq!(path.components().as_slice(), ["folder", "folder2"]);
        }

        #[test]
        fn from_relative() {
            let path = AbsolutePath::new("folder");
            assert_eq!(path.as_str(), "/folder");
            assert_eq!(path.components().as_slice(), ["folder"]);

            let path = AbsolutePath::new("folder/folder2");
            assert_eq!(path.as_str(), "/folder/folder2");
            assert_eq!(path.components().as_slice(), ["folder", "folder2"]);
        }

        #[test]
        fn from_relative_current() {
            let path = AbsolutePath::new("./folder");
            assert_eq!(path.as_str(), "/folder");
            assert_eq!(path.components().as_slice(), ["folder"]);

            let path = AbsolutePath::new("./folder/folder2");
            assert_eq!(path.as_str(), "/folder/folder2");
            assert_eq!(path.components().as_slice(), ["folder", "folder2"]);
        }

        #[test]
        fn from_relative_parent() {
            let path = AbsolutePath::new("../folder");
            assert_eq!(path.as_str(), "/folder");
            assert_eq!(path.components().as_slice(), ["folder"]);

            let path = AbsolutePath::new("../folder/folder2");
            assert_eq!(path.as_str(), "/folder/folder2");
            assert_eq!(path.components().as_slice(), ["folder", "folder2"]);
        }

        #[test]
        fn with_parent() {
            let path = AbsolutePath::new("../folder/../folder2/folder3///..");
            assert_eq!(path.as_str(), "/folder2");
            assert_eq!(path.components().as_slice(), ["folder2"]);

            let path = AbsolutePath::new("../folder/folder2/../folder2/folder3///..");
            assert_eq!(path.as_str(), "/folder/folder2");
            assert_eq!(path.components().as_slice(), ["folder", "folder2"]);
        }

        #[test]
        fn with_current() {
            let path = AbsolutePath::new("./folder/./folder2/folder3///.");
            assert_eq!(path.as_str(), "/folder/folder2/folder3");
            assert_eq!(
                path.components().as_slice(),
                ["folder", "folder2", "folder3"]
            );

            let path = AbsolutePath::new("./folder/folder2/../folder2/folder3///.");
            assert_eq!(path.as_str(), "/folder/folder2/folder3");
            assert_eq!(
                path.components().as_slice(),
                ["folder", "folder2", "folder3"]
            );
        }

        #[test]
        fn root() {
            let path = AbsolutePath::new("/");
            assert_eq!(path.as_str(), "/");
            assert!(path.is_root());
            assert_eq!(path.components(), Vec::<&str>::new());

            let path = AbsolutePath::new("./");
            assert_eq!(path.as_str(), "/");
            assert!(path.is_root());
            assert_eq!(path.components().as_slice(), Vec::<&str>::new());

            let path = AbsolutePath::new("../");
            assert_eq!(path.as_str(), "/");
            assert!(path.is_root());
            assert_eq!(path.components().as_slice(), Vec::<&str>::new());

            let path = AbsolutePath::new("././.");
            assert_eq!(path.as_str(), "/");
            assert!(path.is_root());
            assert_eq!(path.components().as_slice(), Vec::<&str>::new());

            let path = AbsolutePath::new("/../../../../");
            assert_eq!(path.as_str(), "/");
            assert!(path.is_root());
            assert_eq!(path.components().as_slice(), Vec::<&str>::new());

            let path = AbsolutePath::new("../../../../../");
            assert_eq!(path.as_str(), "/");
            assert!(path.is_root());
            assert_eq!(path.components().as_slice(), Vec::<&str>::new());
        }

        #[test]
        fn name_root() {
            let path = AbsolutePath::new("/");
            assert_eq!(path.name(), "/");
        }

        #[test]
        fn name_folder() {
            let path = AbsolutePath::new("/folder/folder2/folder3");
            assert_eq!(path.name(), "folder3");
        }

        #[test]
        fn parent_folder() {
            let path = AbsolutePath::new("/folder/folder2/folder3");
            assert_eq!(path.parent().as_str(), "/folder/folder2");
        }

        #[test]
        fn parent_folder_root() {
            let path = AbsolutePath::new("/");
            assert_eq!(path.parent().as_str(), "/");
        }
    }
}
