#[cfg(test)]
mod test {
    use std::time::{SystemTime, UNIX_EPOCH};

    use lz4_dir;
    #[test]
    fn test_lz4dir() {
        let time_str = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("TIMESTAMP GET FAILED");
        let static_path =
            std::path::Path::new(std::env::current_dir().unwrap().as_path()).join("static");

        //compress-dir
        let out_file = std::env::current_dir()
            .unwrap()
            .as_path()
            .join(format!("out{}.lz4", time_str.as_secs().to_string()));
        println!("test tar from : {:?} to:{:?}", static_path, &out_file);
        lz4_dir::compress_dir(&lz4_dir::Lz4Config::default(), &static_path, &out_file)
            .expect("comporess-dir failed!");

        //decompress-dir
        // let ret = lz4_dir::decompress_dir(&lz4_dir::Lz4Config::default(), &out_file);
        // println!("result is {:?}", ret);
    }
}
