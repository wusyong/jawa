use std::pin::Pin;

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
        fn clear(self: Pin<&mut QList_QNetworkCookie>);
        fn contains(self: &QList_QNetworkCookie, _: &QNetworkCookie) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve"]
        fn qlistReserve(_: Pin<&mut QList_QNetworkCookie>, _: isize);
        #[rust_name = "append"]
        fn qlistAppend(_: Pin<&mut QList_QNetworkCookie>, _: &QNetworkCookie);
        #[rust_name = "get_unchecked"]
        fn qlistGetUnchecked(_: &QList_QNetworkCookie, _: isize) -> &QNetworkCookie;
        #[rust_name = "index_of"]
        fn qlistIndexOf(_: &QList_QNetworkCookie, _: &QNetworkCookie) -> isize;
        #[rust_name = "insert"]
        fn qlistInsert(_: Pin<&mut QList_QNetworkCookie>, _: isize, _: &QNetworkCookie);
        #[rust_name = "remove"]
        fn qlistRemove(_: Pin<&mut QList_QNetworkCookie>, _: isize);
        #[rust_name = "len"]
        fn qlistLen(_: &QList_QNetworkCookie) -> isize;

        #[cxx_name = "removeOne"]
        fn remove_one(self: Pin<&mut QList_QNetworkCookie>, _: &QNetworkCookie) -> bool;
    }

    impl UniquePtr<QList_QNetworkCookie> {}
}

impl ffi::QList_QNetworkCookie {
    pub fn reserve(self: Pin<&mut Self>, size: isize) {
        ffi::reserve(self, size);
    }

    pub fn append(self: Pin<&mut Self>, cookie: &QNetworkCookie) {
        ffi::append(self, cookie);
    }

    pub fn get_unchecked(self: &Self, pos: isize) -> &QNetworkCookie {
        ffi::get_unchecked(self, pos)
    }

    pub fn index_of(self: &Self, cookie: &QNetworkCookie) -> isize {
        ffi::index_of(self, cookie)
    }

    pub fn insert(self: Pin<&mut Self>, pos: isize, cookie: &QNetworkCookie) {
        ffi::insert(self, pos, cookie);
    }

    pub fn remove(self: Pin<&mut Self>, pos: isize) {
        ffi::remove(self, pos);
    }

    pub fn len(self: &Self) -> isize {
        ffi::len(self)
    }

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
