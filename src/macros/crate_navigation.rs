macro_rules! crate_dir {
    () => {
        env!("CARGO_MANIFEST_DIR")
    };
}
pub(crate) use crate_dir;

macro_rules! doc_dir {
    () => {
        concat!($crate::macros::crate_navigation::crate_dir!(), "/", "doc")
    };
}
pub(crate) use doc_dir;

macro_rules! examples_dir {
    () => {
        concat!(
            $crate::macros::crate_navigation::doc_dir!(),
            "/",
            "examples"
        )
    };
    ($lang:literal) => {
        concat!(
            $crate::macros::crate_navigation::examples_dir!(),
            "/",
            $lang
        )
    };
}
pub(crate) use examples_dir;

macro_rules! example_dir {
    ($lang:literal, $name:literal) => {
        concat!(
            $crate::macros::crate_navigation::examples_dir!($lang),
            "/",
            $name
        )
    };
}
pub(crate) use example_dir;

macro_rules! example_file {
    ($lang:literal, $name:literal, $filename:literal) => {
        concat!(
            $crate::macros::crate_navigation::example_dir!($lang, $name),
            "/",
            $filename
        )
    };
}
pub(crate) use example_file;

macro_rules! example_file_content {
    ($lang:literal, $name:literal, $filename:literal) => {
        include_str!($crate::macros::crate_navigation::example_file!(
            $lang, $name, $filename
        ))
    };
}
pub(crate) use example_file_content;
