//! Spider
pub mod spider {
    /// This module is responsible for parsing the response.
    pub struct Spider<'a> {
        pub name: &'a str,
        pub start_urls: Vec<&'a str>
    }

    impl<'a> Spider<'a> {
        pub fn new(name: &'a str) -> Spider<'a> {
            Spider {
                name,
                start_urls: Vec::new()
            }
        }

        pub fn push_url(&mut self, s: &'a str) {
            self.start_urls.push(s)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Spider;

        #[test]
        fn spider_new_test() {
            let sp = Spider::new("rakun");
            assert_eq!("rakun", sp.name);
        }

        #[test]
        fn spider_push_url_test() {
            let url = "http://httpbin.org/get";
            let mut sp = Spider::new("rakun");

            sp.push_url(url);

            assert_eq!(1, sp.start_urls.len());
        }
    }
}
