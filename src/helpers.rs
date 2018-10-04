
#[cfg(feature = "mediawiki")]
use mwparser_utils::filename_to_make;
#[cfg(feature = "mfnf")]
use mfnf_sitemap::{ExcludeMarker, Part};
#[cfg(feature = "mfnf")]
use serde_json;
#[cfg(feature = "mfnf")]
use handlebars::{Handlebars, Output, RenderContext, Helper, Context, HelperResult};

#[cfg(feature = "mediawiki")]
handlebars_helper!(EscapeMake: |path: str| filename_to_make(&path));

#[cfg(feature = "mfnf")]
pub fn is_article_excluded(h: &Helper, _: &Handlebars, _: &Context, _rc: &mut RenderContext, out: &mut Output) -> HelperResult {
    let marker = h.param(0).expect("first argument should be the ExcludeMarker!").value();
    let subtarget = h.param(1).expect("second argument should be the subtarget!").value();
    let marker: ExcludeMarker = serde_json::from_value(marker.clone())
        .expect("could not deserialize marker!");
    let subtarget: String = serde_json::from_value(subtarget.clone())
        .expect("could not deserialize subtarget!");
    let excluded = marker.subtargets
        .iter()
        .find(|t| t.name == subtarget && t.parameters.is_empty())
        .is_some();
    out.write(if excluded {"true"} else {""})?;
    Ok(())
}

#[cfg(feature = "mfnf")]
pub fn is_part_excluded(h: &Helper, _: &Handlebars, _: &Context, _rc: &mut RenderContext, out: &mut Output) -> HelperResult {
    let part = h.param(0).expect("first argument should be the Part!").value();
    let subtarget = h.param(1).expect("second argument should be the subtarget!").value();
    let part: Part = serde_json::from_value(part.clone())
        .expect("could not deserialize Part!");
    let subtarget: String = serde_json::from_value(subtarget.clone())
        .expect("could not deserialize subtarget!");
    let excluded = part.chapters.iter().all(
        |chapter| chapter.markers.exclude.subtargets
            .iter()
            .find(|t| t.name == subtarget && t.parameters.is_empty())
            .is_some()
    );
    out.write(if excluded {"true"} else {""})?;
    Ok(())
}

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
