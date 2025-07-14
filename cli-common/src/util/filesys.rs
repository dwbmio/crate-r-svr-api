use ignore::DirEntry;
use md5::{self, Digest, Md5};
use zip::write::SimpleFileOptions;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::Path;
use std::{fs, io};
use crate::zip_support::zip::result::ZipError;
use crate::zip_support::zip_extensions::*;

///获取文件的md5值
/// ## Example:
/// ```
/// use cli_common::util::filesys::file_md5;
/// let ret: String = file_md5("c://a.txt");
/// ```
pub fn file_md5(path: &str) -> String {
    let mut f = File::open(path).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    let mut hasher = Md5::new();
    hasher.update(&buffer);
    // hex!()
    let ret = hasher.finalize();
    return format!("{:X}", ret);
}

///拷贝当前文件夹内所有文件到指定文件夹
///##Exmample
/// ```
/// //todo
/// //copy_dir_all()
/// ```
pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

///压缩指定路径的所有文件到zip
///###Example:
///```
/// use cli_common::util::filesys::zip_dir;
/// use cli_common::Ex::walkdir::WalkDir;
///
/// let zipfile = std::fs::File::create(target).unwrap();
/// let dir  = WalkDir::new(src_dir);
/// zip_dir(&mut dir.into_iter().filter_map(|e|e.ok()), src_dir.to_str().unwrap(), zipfile);
///```
///
///
pub fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options : SimpleFileOptions = SimpleFileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        // Write file or directory explicitly
        // Some unzip tools unzip files with directory paths correctly, some do not!
        if path.is_file() {
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            println!("adding dir {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

pub fn unzip_to_path(zip_f: &str, tar_p: &str) -> Result<(), ZipError> {
    let _ = zip::ZipArchive::new(File::open(zip_f).unwrap()).unwrap();
    let out = Path::new(tar_p);
    let _ = zip_extract(&Path::new(zip_f).to_path_buf(), &out.to_path_buf());
    Ok(())
}
