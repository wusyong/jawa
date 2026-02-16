pub use ffi::QList_QNetworkCookie;

use crate::QNetworkCookie;

#[cxx_qt::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-widgets/qnetworkcookie.h");
        type QNetworkCookie = crate::QNetworkCookie;

        include!("cxx-qt-widgets/qlist_QNetworkCookie.h");
        type QList_QNetworkCookie;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: Pin<&mut QList_QNetworkCookie>);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_QNetworkCookie, _: &QNetworkCookie) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve"]
        fn qlistReserve(self: Pin<&mut QList_QNetworkCookie>, size: isize);
        #[rust_name = "append"]
        fn qlistAppend(self: Pin<&mut QList_QNetworkCookie>, _: &QNetworkCookie);
        #[rust_name = "get_unchecked"]
        fn qlistGetUnchecked(self: &QList_QNetworkCookie, pos: isize) -> &QNetworkCookie;
        #[rust_name = "index_of"]
        fn qlistIndexOf(self: &QList_QNetworkCookie, _: &QNetworkCookie) -> isize;
        #[rust_name = "insert"]
        fn qlistInsert(self: Pin<&mut QList_QNetworkCookie>, _: isize, _: &QNetworkCookie);
        #[rust_name = "remove"]
        fn qlistRemove(self: Pin<&mut QList_QNetworkCookie>, _: isize);
        #[rust_name = "len"]
        fn qlistLen(self: &QList_QNetworkCookie) -> isize;
    }
}

impl ffi::QList_QNetworkCookie {
    pub fn get(&self, pos: isize) -> Option<&QNetworkCookie> {
        if pos < 0 || pos >= self.len() {
            None
        } else {
            Some(self.get_unchecked(pos))
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter {
            list: self,
            index: 0,
        }
    }
}

pub struct Iter<'a> {
    list: &'a QList_QNetworkCookie,
    index: isize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a QNetworkCookie;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.get(self.index).map(|item| {
            self.index += 1;
            item
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        (self.list.len() - self.index) as usize
    }
}

impl<'a> IntoIterator for &'a QList_QNetworkCookie {
    type Item = &'a QNetworkCookie;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
