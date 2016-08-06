use std::str;
use std::io::Read;

extern crate encoding;
use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_1252;
use encoding::label::encoding_from_whatwg_label;

extern crate flate2;
use flate2::read::{GzDecoder, ZlibDecoder};

extern crate hyper;
use hyper::Client;
use hyper::header::{AcceptCharset, AcceptEncoding, ContentEncoding, ContentType, Charset, Headers,
                    qitem, Encoding as HyperEncoding};

fn fetch_page(url: &str) -> Result<String, &'static str> {
    let client = Client::new();

    let mut headers = Headers::new();
    headers.set(AcceptCharset(vec![qitem(Charset::Ext("utf-8".to_owned()))]));
    headers.set(AcceptEncoding(vec![qitem(HyperEncoding::Gzip),
                                    qitem(HyperEncoding::Deflate),
                                    qitem(HyperEncoding::Chunked),
                                    qitem(HyperEncoding::Identity)]));

    let mut fetch_result =
        try!(client.get(url).headers(headers).send().ok().ok_or("Error GET-ing page"));

    let mut body_buffer = Vec::new();

    try!(fetch_result.read_to_end(&mut body_buffer)
        .ok()
        .ok_or("Error reading page body to buffer"));

    println!("headers {}", fetch_result.headers);

    match fetch_result.headers.get::<ContentEncoding>() {
        Some(content_encoding_header) => {
            println!("encodings {}", content_encoding_header);
            for encoding in content_encoding_header.iter().rev() {
                match *encoding {
                    HyperEncoding::Gzip => {
                        let mut unzipped_body_buffer = Vec::new();
                        {
                            let mut d = try!(GzDecoder::new(body_buffer.as_slice())
                                .ok()
                                .ok_or("Error creating GzEncoder"));
                            try!(d.read_to_end(&mut unzipped_body_buffer)
                                .ok()
                                .ok_or("Error gzip decoding page body"));
                        }
                        body_buffer = unzipped_body_buffer;
                    }

                    HyperEncoding::Deflate => {
                        let mut unzipped_body_buffer = Vec::new();
                        {
                            let mut d = ZlibDecoder::new(body_buffer.as_slice());
                            let res = d.read_to_end(&mut unzipped_body_buffer);

                            try!(res.ok().ok_or("Error DEFLATE decoding page body"));
                        }
                        body_buffer = unzipped_body_buffer;
                    }
                    HyperEncoding::Chunked => {}
                    HyperEncoding::Identity => {}
                    _ => return Result::Err("Unsupported Decoding"),
                }
            }
        }
        None => {}
    }

    match fetch_result.headers.get::<ContentType>() {
        Some(content_type_header) => {
            match content_type_header.get_param(hyper::mime::Attr::Charset)
                .ok_or("Error getting charset from content-type header") {
                Result::Ok(charset) => {
                    println!("charset: {}", charset);
                    match encoding_from_whatwg_label(charset) {
                        Some(decoder) => {
                            return decoder.decode(&body_buffer, DecoderTrap::Strict)
                                .ok()
                                .ok_or("Error decoding page body (using decoder for charset from \
                                        content-type)");
                        }
                        None => {
                            // No decoder found for charset, will try default decoder
                        }
                    }
                }
                Result::Err(_) => {
                    // no charset in content-type header, will use default
                }
            }
        }
        None => {
            // no content-type header, will use default charset
        }
    }

    return WINDOWS_1252.decode(&body_buffer, DecoderTrap::Strict)
        .ok()
        .ok_or("Error decoding page body (using WINDOWS_1252)");
}

fn print_page(url: &str) {
    let decoded = fetch_page(url);

    println!("{}", decoded.unwrap());
}


fn main() {
    print_page("https://eu.httpbin.org/deflate");
}

#[test]
fn fetch_gzip_compressed_page() {
    fetch_page("http://httpbin.org/gzip").expect("Fetch to succeed");
}

#[test]
fn fetch_utf8_encoded_page() {
    fetch_page("https://eu.httpbin.org/encoding/utf8").expect("Fetch to succeed");
}

#[test]
fn fetch_win1215_encoded_page() {
    fetch_page("http://www.cbloom.com/rants.html").expect("Fetch to succeed"); // will fail when read as UTF-8
}

#[test]
fn fetch_deflate_compressed_page() {
    fetch_page("http://httpbin.org/deflate").expect("Fetch to succeed");
}
