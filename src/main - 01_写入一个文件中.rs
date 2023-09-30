extern crate ipnetwork;

use ipnetwork::IpNetwork;
use std::path::Path;
use std::time::Instant;
use std::sync::{Mutex, Arc};
use std::fs::File;
use std::io::{self, Write, BufRead};
use std::thread;


// 生成IP地址列表
fn generate_ips(cidrs: &Vec<String>) -> io::Result<Vec<String>> {
    let mut ips = Vec::new();

    for cidr in cidrs {
        let cidr_network = cidr.parse::<IpNetwork>().map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("解析{}时出现 {} 错误！", cidr, e),
            )
        })?;

        match cidr_network {
            IpNetwork::V4(v4_network) => {
                let ip_iter = v4_network.iter();
                let ip_strings: Vec<String> = ip_iter.map(|ip| ip.to_string()).collect();
                ips.extend(ip_strings);
            }
            IpNetwork::V6(_) => {
                println!("IPv6 CIDR范围不受支持。");
            }
        }
    }

    Ok(ips)
}

// 将IP地址列表写入文件
fn write_ips_to_file(output_file: &str, ips: Vec<String>) -> io::Result<()> {
    let file = File::create(output_file)?;
    let file = Arc::new(Mutex::new(file)); // 使用互斥锁包装文件对象
    let mut handles = vec![];

    for ip in ips {
        let file = Arc::clone(&file); // 克隆互斥锁的Arc
        let handle = thread::spawn(move || {
            // 在线程中使用互斥锁来写入文件
            let mut file = file.lock().unwrap();
            writeln!(&mut *file, "{}", ip).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}





// 从外部文件中读取CIDR地址
fn read_cidr_from_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(path)?;

    let mut cidrs = Vec::new();

    for line in io::BufReader::new(file).lines() {
        if let Ok(line) = line {
            cidrs.push(line.trim().to_string());
        }
    }

    Ok(cidrs)
}

fn wait_for_enter() {
    let mut input = String::new();
    print!("按下Enter键关闭窗口...");
    io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新输出缓冲区
    io::stdin().read_line(&mut input).expect("读取输入失败");
}

fn main() {
    println!("本程序：生成IPv4 CIDR范围内的所有IP地址！\n");

    let external_cidr_filename = "ips-v4.txt";
	let output_file = "ip.txt";
	
	// 获取用户输入的CIDR地址并检查其合法性
    print!("请输入CIDR地址（按回车键使用外置CIDR）：");
    io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新输出缓冲区

    let mut input = String::new();
	io::stdin().read_line(&mut input).expect("无法读取输入");

	let mut cidrs: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    if cidrs.is_empty() || cidrs.iter().any(|cidr| cidr.parse::<IpNetwork>().is_err()) {
        // 如果CIDR为空字符或不合法，使用外置CIDR
        match read_cidr_from_file(external_cidr_filename) {
            Ok(external_cidrs) => {
                // println!("读取的CIDR地址:");
                // for cidr in &external_cidrs {
                    // println!("{}", cidr);
                // }
                cidrs = external_cidrs;
            }
            Err(err) => {
                eprintln!("读取外部文件出错：{}", err);
            }
        }
    }
	
    // 创建一个Instant实例来记录开始时间
    let start_time = Instant::now();
    match generate_ips(&cidrs) {
        Ok(ips) => {
			println!("44343434343::::{}",ips.len());
            if let Err(err) = write_ips_to_file(output_file, ips) {
                eprintln!("写入文件时出错：{}", err);
            }
        }
        Err(err) => {
            eprintln!("生成IP地址时出错：{}", err);
        }
    }
    let elapsed_time = start_time.elapsed();

    println!("\n程序运行完毕！用时: {:?}", elapsed_time);
    println!();

    wait_for_enter();
}
