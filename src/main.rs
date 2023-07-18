// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

// Alternatively, Iterator::zip() lets us iterate simultaneously over prefix
// and request segments. The zip() iterator is finished as soon as one of
// the source iterators is finished, but we need to iterate over all request
// segments. A neat trick that makes zip() work is to use map() and chain()
// to produce an iterator that returns Some(str) for each pattern segments,
// and then returns None indefinitely.

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    let mut req_segs = request_path.split("/");
    for pre_seg in prefix.split("/") {
        let Some(req_seg) = req_segs.next() else {
            return false;
        };

        if req_seg != pre_seg && pre_seg != "*" {
            return false;
        } 
    }
    true
}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}

fn main() {}
