use ignore;
use log::{error, info};
use std::{
    default,
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};
use tar::Builder;

// ------
// --core
type Lz4Result<T> = Result<T, Lz4DirError>;

#[derive(thiserror::Error, Debug)]
pub enum Lz4DirError {
    #[error("Output failed! {0}")]
    OutputFailed(String),

    #[error("Archive target failed!{0}")]
    ArchiveFailed(String),

    #[error("lz4 compress error")]
    CompressFailed(#[from] lzzzz::lz4f::Error),

    #[error("{0}. Decompress error")]
    DecompressFailed(String),
}

#[derive(Default)]
pub enum Lz4CompressMethod {
    #[default]
    UNHC,
    HC,
}

#[derive(Default)]
pub enum Lz4Sequence {
    #[default]
    TarLz4,
    Lz4Tar,
}

#[derive(Default)]
pub struct Lz4Config {
    is_force: bool,
    compress_mode: Lz4CompressMethod,
    sequence: Lz4Sequence,
}

// ------

/// tar a dir's all file to output file path
/// lz4 first then tar. so the extend
fn dir_to_lz4tar<'a>(
    from_dir: &PathBuf,
    temp_dir: &PathBuf,
    mut out_tar: &'a File,
) -> Lz4Result<&'a File> {
    let mut tar_builder = Builder::new(out_tar);
    for r in ignore::WalkBuilder::new(from_dir).build() {
        let lz_file = r
            .clone()
            .map_err(|f| Lz4DirError::ArchiveFailed(f.to_string()))?;
        let is_file = lz_file
            .metadata()
            .map_err(|f| {
                Lz4DirError::ArchiveFailed(
                    format!("error={} metadata get failed!", f.to_string()).to_owned(),
                )
            })?
            .is_file();
        if is_file {
            println!(
                "inner file {:?} || {:?}",
                &lz_file,
                lz_file
                    .path()
                    .strip_prefix(from_dir)
                    .unwrap()
                    .display()
                    .to_string()
            );
            //zip first
            let rela_path = lz_file.path().strip_prefix(from_dir).unwrap();
            let tmp_lz4 = temp_dir.join(rela_path);
            println!("template file create filed!{}", tmp_lz4.display());
            if tmp_lz4.parent().is_some() {
                let _ = fs::create_dir_all(&tmp_lz4.parent().unwrap());
            }

            let mut base_f = File::open(lz_file.path()).map_err(|f| {
                Lz4DirError::ArchiveFailed(
                    format!("open base file failed! Path = {}", rela_path.display()).to_owned(),
                )
            })?;
            let mut tmp_lz4_f = File::options()
                .read(true)
                .write(true)
                .create_new(true)
                .open(&tmp_lz4)
                .map_err(|e| {
                    Lz4DirError::ArchiveFailed(
                        format!(
                            "temp file create failed! Path = {} Err = {}",
                            tmp_lz4.display(),
                            e.to_string()
                        )
                        .to_owned(),
                    )
                })?;
            _compress_file(&mut base_f, &mut tmp_lz4_f, Lz4CompressMethod::HC)?;
            tar_builder
                .append_file(
                    tar_inner
                        .path()
                        .strip_prefix(from_dir)
                        .unwrap()
                        .display()
                        .to_string(),
                    &mut File::open(tar_inner.path()).map_err(|f| {
                        Lz4DirError::ArchiveFailed(
                            format!("Append failed!Error= {}", f.to_string()).to_owned(),
                        )
                    })?,
                )
                .map_err(|f| {
                    Lz4DirError::ArchiveFailed(
                        format!(
                            "Append path failed!path={}, error={}",
                            tar_inner.path().display().to_string(),
                            f.to_string()
                        )
                        .to_owned(),
                    )
                })?
        }
    }
    tar_builder.finish().map_err(|f| {
        Lz4DirError::ArchiveFailed(
            format!("error={} metadata get failed!", f.to_string()).to_owned(),
        )
    })?;
    // loop {
    //     let read_count = f.read(&mut data).map_err(|e| Lz4DirError::ArchiveFailed("create *tar file failed!".to_owned()))?;
    //     if read_count < CELL_LEN {
    //         break;
    //     }
    // }
    Ok(out_tar)
}

fn dir_to_tarlz4<'a>(from_dir: &PathBuf, mut out_tar: &'a File) -> Lz4Result<&'a File> {
    Ok(out_tar)
}

/// compress file to tar
fn _compress_file<'o>(
    f_file: &'o mut File,
    t_file: &'o mut File,
    mode: Lz4CompressMethod,
) -> Lz4Result<()> {
    let mut lz4_out = lzzzz::lz4f::WriteCompressor::new(
        t_file.try_clone().unwrap(),
        lzzzz::lz4f::Preferences::default(),
    )?;
    let mut reader = io::BufReader::new(f_file);
    let mut lz4_buf = Vec::new();
    reader.read_to_end(&mut lz4_buf).map_err(|f| {
        Lz4DirError::DecompressFailed(format!("Error={}, Lz4f write failed!", f.to_string()))
    })?;
    println!("write in {:?}", lz4_buf);
    lz4_out.write_all(&lz4_buf);

    Ok(())
}

fn pack_dirto_tar<'a>(dir: &PathBuf, lz4_out: &PathBuf) -> Lz4Result<(File, File)> {
    println!("out tar tmp file is {}", lz4_out.display());
    if let Ok(lz4_outf) = File::create(lz4_out) {
        let mut tar_pf = lz4_out.as_path().to_owned();
        tar_pf.set_extension("lz4.tar");
        if let Ok(tar_f) = File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(tar_pf)
        {
            tar_dir_files(dir, &tar_f)?;
            return Ok((lz4_outf, tar_f));
        } else {
            return Err(Lz4DirError::OutputFailed(
                "tar dir's files failed! already exists!".to_owned(),
            ));
        }
    }
    return Err(Lz4DirError::OutputFailed(
        "tar dir's files failed! already exists!".to_owned(),
    ));
}

// /// compress single file
// pub fn compress_file<'a>(cfg: &Lz4Config, f: &PathBuf, lz4_out: &PathBuf) -> Lz4Result<()> {
//     println!("out tar tmp file is {}", lz4_out.display());
//     if let Ok(lz4_outf) = File::create(lz4_out) {
//         let mut tar_pf = lz4_out.as_path().to_owned();
//         tar_pf.set_extension("lz4");
//         if let Ok(tar_f) = File::options()
//             .read(true)
//             .write(true)
//             .create_new(true)
//             .open(tar_pf)
//         {
//             _compress_file(f_file, t_file, mode);
//             return Ok((lz4_outf, tar_f));
//         } else {
//             return Err(Lz4DirError::OutputFailed(
//                 "tar dir's files failed! already exists!".to_owned(),
//             ));
//         }
//     }
//     return Err(Lz4DirError::OutputFailed(
//         "tar dir's files failed! already exists!".to_owned(),
//     ));
//     Ok(())
// }

/// compress hole dir by ignore::walk
/// watch out the defualt cfg of 'Lz4Config'
pub fn compress_dir<'a>(cfg: &Lz4Config, dir: &PathBuf, lz4_out: &PathBuf) -> Lz4Result<()> {
    let temp_dir = tempfile::tempdir_in("")
        .map_err(|f| {
            Lz4DirError::ArchiveFailed(
                format!("create template dir failed!Err = {}", f.to_string()).to_owned(),
            )
        })?
        .into_path();
    let (mut lz4_outf, mut tar_f) = match cfg.sequence {
        Lz4Sequence::TarLz4 => pack_dirto_tar(dir, &lz4_out)?,
        Lz4Sequence::Lz4Tar => pack_dirto_tar(dir, &lz4_out)?,
    };
    //todo
    Ok(())
}

/// decompress hole dir
pub fn decompress_dir<'a>(cfg: &Lz4Config, lzf: &PathBuf) -> Lz4Result<()> {
    if !lzf.is_file() {
        return Err(Lz4DirError::DecompressFailed(
            "Target is not file type.".to_owned(),
        ));
    }

    let mut out_f = File::open(lzf)
        .map_err(|e| Lz4DirError::DecompressFailed(format!("Open failed!Err={}", e.to_string())))?;

    //check magic frame
    let mut r_buffer = Vec::new();
    out_f.read_to_end(&mut r_buffer);
    println!("out lz4 contnet is {:?}", r_buffer);

    println!(
        "decompress dir magic number is  base is : {:?}, bytes is {:#x}",
        out_f, 100
    );
    // let mut tar_pf = lzf.as_path().to_owned();
    // tar_pf.set_extension("lz4.tar");
    // let mut c = lzzzz::lz4f::ReadDecompressor::new(out_f)?;

    // let mut tar_f = pack_dirto_tar(cfg.is_force, &tar_pf)?;
    // tar_f.write_all(&r_buffer);

    Ok(())
}
