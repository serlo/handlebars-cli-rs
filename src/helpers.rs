
#[cfg(feature = "mediawiki")]
use mwparser_utils::filename_to_make;

#[cfg(feature = "mediawiki")]
handlebars_helper!(EscapeMake: |path: str| filename_to_make(&path));

/// based on  https://github.com/bt/rust_urlencoding
#[cfg(feature = "mediawiki")]
pub fn urlencode(data: &str) -> String {
    let mut escaped = String::new();
    for b in data.as_bytes().iter() {
        match *b as char {
            // Accepted characters
            'A'...'Z' | 'a'...'z' | '0'...'9' | '/' | ':' | '-' | '_' | '.' | '~' => {
                escaped.push(*b as char)
            }

            // Everything else is percent-encoded
            b => escaped.push_str(format!("%{:02X}", b as u32).as_str()),
        };
    }
    return escaped;
}

#[cfg(feature = "mediawiki")]
handlebars_helper!(UrlEncode: |input: str| urlencode(&input));

handlebars_helper!(AddHelper: |x: f64, y: f64| x + y);
handlebars_helper!(MultHelper: |x: f64, y: f64| x * y);
