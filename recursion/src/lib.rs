#[allow(dead_code)]
struct Package {
    contents: Contents,
}

impl Package {
    #[allow(dead_code)]
    fn do_have_key(&self) -> bool {
        match &self.contents {
            Contents::Key => true,
            Contents::Packages(pkgs) => {
                for pkg in pkgs {
                    if pkg.do_have_key() {
                        return true;
                    }
                }

                false
            }
        }
    }
}

#[allow(dead_code)]
enum Contents {
    Key,
    Packages(Vec<Package>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let pkg = Package {
            contents: Contents::Packages(vec![Package {
                contents: Contents::Packages(vec![
                    Package {
                        contents: Contents::Packages(Vec::new()),
                    },
                    Package {
                        contents: Contents::Packages(vec![Package {
                            contents: Contents::Key,
                        }]),
                    },
                    Package {
                        contents: Contents::Packages(Vec::new()),
                    },
                    Package {
                        contents: Contents::Packages(Vec::new()),
                    },
                ]),
            }]),
        };

        assert_eq!(true, pkg.do_have_key());
    }

    #[test]
    fn empty() {
        let pkg = Package {
            contents: Contents::Packages(Vec::new()),
        };

        assert_eq!(false, pkg.do_have_key());
    }
}
