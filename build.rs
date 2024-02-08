use copy_to_output::copy_to_output;
use std::env;

fn main() {
    //拷贝资源文件到编译目录
    //println!("cargo:rerun-if-changed=templates/*");
    println!("cargo:rerun-if-changed=conf.json");
    //let out_dir = env::var_os("OUT_DIR").unwrap();
    //println!("out_dir: {:?}", out_dir);
    let profile = env::var("PROFILE").unwrap();
    println!("build profile: {:?}", profile);
    copy_to_output("templates", &profile).expect("templates not copy");
    copy_to_output("conf.json", &profile).expect("conf.json not copy");
}
