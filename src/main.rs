use std::env;
use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};



fn main() -> std::io::Result<()> {
    let mut dir = ".".to_string(); //源目录
    let mut c_dir = "".to_string(); //目标目录
    let mut ext_name = ""; //后缀名
    let mut time = 60; //向前推的时间，单位秒
    let mut du_time = 60; //扫描间隔时间，单位秒

    let args: Vec<String> = env::args().collect();

    for i in 0..args.len() {
//                println!("{}",args[i]);
        if args[i].to_lowercase().starts_with("-d") {
            dir = args[i + 1].to_string();
        }
        if args[i].to_lowercase().starts_with("-c") {
            c_dir = args[i + 1].to_string();
        }
        if args[i].to_lowercase().starts_with("-ext") {
            ext_name = args[i + 1].as_str();
        }

        if args[i].to_lowercase().starts_with("-s") {
            time = args[i + 1]
                .as_str()
                .parse::<u64>()
                .expect("传入的时间格式不对！！正确的格式： -s 1000 ");
        }

        if args[i].to_lowercase().starts_with("-u") {
            du_time = args[i + 1]
                .as_str()
                .parse::<u64>()
                .expect("传入的间隔时间格式不对！！正确的格式： -u 1000 ");
        }
    }
    
    

    println!("------------------------------------");
    println!("★❆此程序监控指定目录文件，拷贝修改时间范围内的文件到目的目录。\n★❆参数说明：");
    println!(" -d 源目录 *必传\n -c 目的目录 *必传\n -ext 拷贝后附加的后缀名，注意不会修改文件原有后缀名而是后面附加 *非必传，默认空，即保持原文件\n -s 拷贝当前时间前多久的文件，按文件的修改时间计算,单位秒，传入0为全拷贝模式 *非必传，默认60\n -u 循环扫描的间隔时间，单位秒。 *非必传，默认60");
    println!("------------------------------------");
    println!("★❆如果传入范围时间(-s)为0，则进行全拷贝。即拷贝目录下所有文件，且只拷贝运行一次就退出！！");
    println!("    例如：CopyAndRename.exe -d e:\\123 -c e:\\copy -ext jpg -s 0");
    println!("------------------------------------");
    println!("★❆如果范围时间和间隔时间相等，则每个文件只会被拷贝一次！！");
    println!("    例如：CopyAndRename.exe -d e:\\123 -c e:\\copy -ext jpg -s 60 -u 60");
    println!("------------------------------------");
    println!("★❆如果需要监控一个目录及历史文件，可以先运行一次全拷贝，然后再运行参数范围时间和间隔时间相等，例如60s，则每60秒拷贝一次目录内新增的文件。");
    println!("------------------------------------");
    
    if(dir.is_empty()||c_dir.is_empty()){
        println!("错误!!!参数不够，请传参后运行！！ -d -c 为必须参数！！");
        return  Ok(());
    }
    
    if !Path::new(&dir).exists() {
        panic!("-d传入的目录{}不存在！！",&dir);
    }
    
    //检查目标目录，不存在则递归创建
    fs::create_dir_all(&c_dir)?;

    if (!dir.ends_with("\\")) {
        dir = format!("{}\\", dir);
    }
    if (!c_dir.ends_with("\\")) {
        c_dir = format!("{}\\", c_dir);
    }
    
    println!(
        "运行参数：源目录:{},目的目录:{},后缀名:{},拷贝的时间修改范围:{}s,扫描间隔时间:{}s",
        dir, c_dir, ext_name, time, du_time
    );

    if(time == 0){
        time = 999999999;
    }

    loop {
        let last_modified_file = std::fs::read_dir(dir.as_str())
            .expect("传入的监听目录错误！！正确的: -d d:\\ss")
            .flatten() // Remove failed
            .filter(|f| f.metadata().unwrap().is_file()) // Filter out directories (only consider files)
            //    .max_by_key(|x| x.metadata().unwrap().modified().unwrap()); // Get the most recently modified file
            .filter(|f| {
                SystemTime::now()
                    .duration_since(f.metadata().unwrap().modified().unwrap())
                    .unwrap()
                    .as_secs()
                    < time
            });

        last_modified_file.for_each(|f| {
            let file_path = format!("{}{}", dir, f.file_name().into_string().unwrap());
            let copy_path = format!(
                "{}{}.{}",
                c_dir,
                f.file_name().into_string().unwrap(),
                ext_name
            );
            println!("Copy file: {} - > {}", file_path, copy_path);
            fs::copy(file_path, copy_path).unwrap();
        });
        if(time == 999999999){ //如果传入时间为0，只拷贝运行一次就退出
           return Ok(());
        }
        std::thread::sleep(Duration::from_secs(du_time));
    }

    Ok(())
}
