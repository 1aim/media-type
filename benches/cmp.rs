#![feature(test)]

extern crate media_type as mime;
extern crate test;

use mime::{MediaType, TEXT, PLAIN, CHARSET, AsciiCaseInsensitiveEq};
use mime::spec::{HttpSpec, Obs};
use test::{Bencher, black_box};

#[bench]
fn is_charset_latin1(b: &mut Bencher) {
    b.iter(|| {
        let mime = MediaType::<HttpSpec>::parse(black_box("text/plain; charset=utf-8")).unwrap();
        match (mime.type_(), mime.subtype()) {
            (TEXT, PLAIN) => {
                for (param, value) in mime.params() {
                    match param {
                        CHARSET if value.eq_ignore_ascii_case("latin1") => (),
                        _ => return false,
                    }
                }
            }
            _ => return false
        }
        true
    })
}

#[bench]
fn bench_eq_parsed_both_utf8(b: &mut Bencher) {
    let mime = MediaType::<HttpSpec<Obs>>::parse("text/plain; charset=utf-8").unwrap();
    let pre_parsed = MediaType::<HttpSpec<Obs>>::parse("text/plain; charset=utf-8").unwrap();
    b.bytes = mime.as_str_repr().len() as u64;
    b.iter(|| {
        assert_eq!(mime, pre_parsed);
    })
}

#[bench]
fn bench_eq_parsed_both_non_charset_utf8(b: &mut Bencher) {
    let mime = MediaType::<HttpSpec<Obs>>::parse("text/plain; foo=bar").unwrap();
    let pre_parsed = MediaType::<HttpSpec<Obs>>::parse("text/plain; foo=bar").unwrap();
    b.bytes = mime.as_str_repr().len() as u64;
    b.iter(|| {
        assert_eq!(mime, pre_parsed);
    })
}
//
//#[bench]
//fn bench_eq_consts(b: &mut Bencher) {
//    let mime = TEXT_PLAIN_UTF_8;
//    b.bytes = mime.as_ref().len() as u64;
//    b.iter(|| {
//        assert_eq!(mime, TEXT_PLAIN_UTF_8);
//    });
//}

#[bench]
fn bench_ne_parsed_subtype(b: &mut Bencher) {
    let left = MediaType::<HttpSpec<Obs>>::parse("text/plain; charset=utf-8").unwrap();
    let right = MediaType::<HttpSpec<Obs>>::parse("text/css; charset=utf-8").unwrap();
    b.bytes = left.as_str_repr().len() as u64;
    b.iter(|| {
        assert_ne!(left, right);
    });
}

#[bench]
fn bench_eq_type_(b: &mut Bencher) {
    let mime = MediaType::<HttpSpec<Obs>>::parse("text/plain; charset=utf-8").unwrap();
    let name = TEXT;
    b.bytes = name.as_ref().len() as u64;
    b.iter(|| {
        assert_eq!(mime.type_(), name);
    });
}
