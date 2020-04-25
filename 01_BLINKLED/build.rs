use std::{env, fs, path::Path};

fn main() {
    // copy the linker script from the boot crate to the current directory
    // so it will be invoked by the linker
    if let Some(link_script) = env::var_os("DEP_RUSPIRO_BOOT_LINKERSCRIPT") {
        let ld_source = link_script
            .to_str()
            .unwrap()
            .replace("\\", "/");
        let src_file = Path::new(&ld_source);
        let trg_file = format!(
            "{}/{}",
            env::current_dir().unwrap().display(),
            src_file.file_name().unwrap().to_str().unwrap()
        );
        println!("Copy linker script from {:?}, to {:?}", src_file, trg_file);
        fs::copy(src_file, trg_file).unwrap();
    } else {
        println!("error in ruspiro build, `ruspiro-boot` not a dependency?");
    }
}
