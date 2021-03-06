use std::cmp::PartialEq;
use std::fmt;
use std::ops::Deref;

macro_rules! def_static_names {
    ($($(#[$attr:meta])* $n:ident = $t:tt;)*) => ($(
        $(#[$attr])*
        pub const $n: Name = Name { source: $t };
    )*);
}

//the main types
def_static_names! {
    APPLICATION = "application";
    AUDIO = "audio";
    FONT = "font";
    IMAGE = "image";
    MESSAGE = "message";
    MODEL = "model";
    MULTIPART = "multipart";
    TEXT = "text";
    VIDEO = "video";
}

// some sub types
def_static_names! {
    PLAIN = "plain";
    JAVASCRIPT = "javascript";
    PNG = "png";
    SVG_XML = "svg+xml";
    OCTET_STREAM = "octet-stream";
    RELATED = "related";
    MIXED = "mixed";
    ALTERNATIVE = "alternative";
    //TODO more of them
}

// some fields
def_static_names! {
    CHARSET = "charset";
    BOUNDARY = "boundary";
}



//TODO add Spec :=/
/// A name section of a `Mime`.
///
/// For instance, for the Mime `image/svg+xml`, it contains 3 `Name`s,
/// `image`, `svg`, and `xml`.
///
/// In all cases, `Name`s are compared case insensitive.
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Name<'a> {
    // TODO: optimize with an Atom-like thing
    // There a `const` Names, and so it is possible for the static strings
    // to have a different memory address. Additionally, when used in match
    // statements, the strings are compared with a memcmp, possibly even
    // if the address and length are the same.
    //
    // Being an enum with an Atom variant that is a usize (and without a
    // string pointer and boolean) would allow for faster comparisons.
    /// the underlying str slice, which is _required to be lowercase_.
    /// Comparisons between two Name instances expect this, as they
    /// have to use `derive(PartialEq)` to be usable in a pattern
    source: &'a str,
}

impl<'a> Name<'a> {

    #[inline]
    pub(crate) fn new_unchecked(source: &'a str) -> Name<'a> {
        Name { source }
    }
}


impl<'a> Name<'a> {
    /// Get the value of this `Name` as a string.
    ///
    /// Note that the borrow is not tied to `&self` but the `'a` lifetime, allowing the
    /// string to outlive `Name`. Alternately, there is an `impl<'a> From<Name<'a>> for &'a str`
    /// which isn't rendered by Rustdoc, that can be accessed using `str::from(name)` or `name.into()`.
    pub fn as_str(&self) -> &'a str {
        self.source
    }
}

impl<'a> Deref for Name<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        self.source
    }
}

impl<'a> PartialEq<str> for Name<'a> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.source.eq_ignore_ascii_case(other)
    }
}

impl<'a, 'b> PartialEq<&'b str> for Name<'a> {
    #[inline]
    fn eq(&self, other: & &'b str) -> bool {
        self == *other
    }
}

impl<'a> PartialEq<Name<'a>> for str {
    #[inline]
    fn eq(&self, other: &Name<'a>) -> bool {
        other == self
    }
}

impl<'a, 'b> PartialEq<Name<'a>> for &'b str {
    #[inline]
    fn eq(&self, other: &Name<'a>) -> bool {
        other == self
    }
}

impl<'a> AsRef<str> for Name<'a> {
    #[inline]
    fn as_ref(&self) -> &str {
        self.source
    }
}

impl<'a> From<Name<'a>> for &'a str {
    #[inline]
    fn from(name: Name<'a>) -> &'a str {
        name.source
    }
}

impl<'a> fmt::Debug for Name<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.source, f)
    }
}

impl<'a> fmt::Display for Name<'a> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.source, f)
    }
}


#[cfg(test)]
mod test {
    use super::Name;

    #[test]
    fn test_name_eq_str() {
        let param = Name { source: "abc" };

        assert_eq!(param, param);
        assert_eq!(param, "ABC");
        assert_eq!("ABC", param);
        assert_eq!(param, "abc");
        assert_eq!("abc", param);
    }

    #[test]
    fn test_name_eq_name() {
        let n1 = Name::new_unchecked("abc");
        let n2 = Name::new_unchecked("abc");
        assert_eq!(n1, n2);

        let n3 = Name::new_unchecked("aBc");
        assert_ne!(n1, n3, concat!(
            "while Name is case insensitive it needs to derive(PartialEq) to be usable in match\n",
            "as such names can only be constructed from lowercase strings"
        ));

        assert_eq!(n1, n3.as_str());
        assert_eq!(n3, n1.as_str());
    }


}
