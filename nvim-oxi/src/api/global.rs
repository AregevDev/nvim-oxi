use std::ffi::{CString, NulError};

use nvim_types::{BufHandle, Error as NvimError, String as NvimString};

use super::buffer::Buffer;
use crate::Result;

extern "C" {
    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L1057
    fn nvim_create_buf(
        listed: bool,
        scratch: bool,
        err: *mut NvimError,
    ) -> BufHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L963
    fn nvim_get_current_buf() -> BufHandle;

    // https://github.com/neovim/neovim/blob/master/src/nvim/api/vim.c#L398
    fn nvim_replace_termcodes(
        str: NvimString,
        from_part: bool,
        do_lt: bool,
        special: bool,
    ) -> NvimString;
}

/// Binding to `vim.api.nvim_create_buf`.
pub fn create_buf(is_listed: bool, is_scratch: bool) -> Result<Buffer> {
    let mut err = NvimError::default();
    let handle = unsafe { nvim_create_buf(is_listed, is_scratch, &mut err) };
    err.into_err_or_else(|| Buffer::from(handle))
}

/// Binding to `vim.api.nvim_echo`.
pub fn echo<Text, HlGroup, Chunks>(
    chunks: Chunks,
    add_to_history: bool,
) -> Result<()>
where
    Text: std::fmt::Display,
    HlGroup: AsRef<str>,
    Chunks: IntoIterator<Item = (Text, Option<HlGroup>)>,
{
    let chunks = chunks.into_iter().map(|(text, hlgroup)| todo!());
    todo!()
}

/// Binding to `vim.api.nvim_get_current_buf`.
pub fn get_current_buf() -> Buffer {
    Buffer::from(unsafe { nvim_get_current_buf() })
}

/// Binding to `vim.api.nvim_replace_termcodes`.
///
/// Fails when the provived `str` contains a null byte.
pub fn replace_termcodes(
    str: &str,
    from_part: bool,
    do_lt: bool,
    special: bool,
) -> std::result::Result<CString, NulError> {
    let str = NvimString::new(str)?;

    Ok(unsafe { nvim_replace_termcodes(str, from_part, do_lt, special) }
        .as_c_str()
        .to_owned())
}