use std::slice;

use deno_core::{error::AnyError, Extension, OpState};
use deno_ops::op;
use windows::{
    core::GUID,
    Win32::{
        System::Com::CoTaskMemFree,
        UI::Shell::{
            FOLDERID_Documents, FOLDERID_LocalAppData, FOLDERID_LocalAppDataLow, FOLDERID_Pictures,
            FOLDERID_Profile, FOLDERID_ProgramData, FOLDERID_ProgramFiles,
            FOLDERID_ProgramFilesX64, FOLDERID_ProgramFilesX86, FOLDERID_RoamingAppData,
            FOLDERID_UserProgramFiles, SHGetKnownFolderPath, KF_FLAG_DONT_VERIFY,
            KF_FLAG_NO_PACKAGE_REDIRECTION,
        },
    },
};

fn get_known_folder_path(id: &GUID) -> Option<String> {
    unsafe {
        let path = SHGetKnownFolderPath(
            id,
            (KF_FLAG_NO_PACKAGE_REDIRECTION.0 | KF_FLAG_DONT_VERIFY.0) as _,
            None,
        )
        .ok()?;
        if path.is_null() {
            return None;
        }
        let len = (0..)
            .into_iter()
            .find(|i| path.0.add(*i).read() == 0)
            .unwrap();
        let path_str = String::from_utf16(slice::from_raw_parts(path.0, len)).ok();
        CoTaskMemFree(path.0 as *const _);
        path_str
    }
}

#[op]
pub fn op_expand_path(_: &mut OpState, mut path: String) -> Result<String, AnyError> {
    let mut completed = 0;
    loop {
        let start = match path[completed..].find("${") {
            Some(start) => completed + start,
            None => break,
        };
        let end = match path[start + 2..].find("}") {
            Some(end) => start + 2 + end,
            None => break,
        };
        let replacement = match &path[start + 2..end] {
            "Documents" => get_known_folder_path(&FOLDERID_Documents),
            "LocalAppData" => get_known_folder_path(&FOLDERID_LocalAppData),
            "LocalAppDataLow" => get_known_folder_path(&FOLDERID_LocalAppDataLow),
            "Pictures" => get_known_folder_path(&FOLDERID_Pictures),
            "Profile" => get_known_folder_path(&FOLDERID_Profile),
            "ProgramData" => get_known_folder_path(&FOLDERID_ProgramData),
            "ProgramFiles" => get_known_folder_path(&FOLDERID_ProgramFiles),
            "ProgramFilesX64" => get_known_folder_path(&FOLDERID_ProgramFilesX64),
            "ProgramFilesX86" => get_known_folder_path(&FOLDERID_ProgramFilesX86),
            "RoamingAppData" => get_known_folder_path(&FOLDERID_RoamingAppData),
            "UserProgramFiles" => get_known_folder_path(&FOLDERID_UserProgramFiles),
            _ => None,
        };
        if let Some(replacement) = replacement {
            path.replace_range(start..=end, &replacement);
            completed = start + replacement.len();
        } else {
            completed = end + 1;
        }
    }
    Ok(path)
}

pub fn extension() -> Extension {
    Extension::builder()
        .ops(vec![op_expand_path::decl()])
        .build()
}
