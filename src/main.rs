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

// 定义一个函数，获取用户输入的分割数
fn get_input_numbers() -> usize {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("无法读取输入");
        match input.trim().parse::<usize>() {
            Ok(num) => return num,
            Err(_) => {
                print!("请输入有效的数字：");
                io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新输出缓冲区
            }
        }
    }
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


// 选择写入文件的模式（等份分割、最大文件上限、全部写入）
fn get_write_mode() -> u32 {
    println!("选择写入txt文件的模式：\n");
    println!("1. 等份分割写入txt文件");
    println!("2. 设置txt文件写入上限");
    println!("3. 全部写入到txt文件中\n");
    loop {
        print!("请选择上面的模式(1/2/3)：");
        io::stdout().flush().expect("刷新输出缓冲区失败");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取用户输入失败");

        match input.trim().parse() {
            Ok(1) | Ok(2) | Ok(3) => return input.trim().parse().unwrap(),
            _ => {}
        }
    }
}

// 等份分割写入txt文件
fn write_equally_to_files(ips: &Vec<String>) {
    print!("设置要分割成多少份文件存储(等份切割)：");
    io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新输出缓冲区
    let num_segments = get_input_numbers(); // 分割多少份文件存储

    let ips_per_segment = ips.len() / num_segments;

    for segment in 0..num_segments {
        let start = segment * ips_per_segment;
        let end = if segment == num_segments - 1 {
            ips.len()
        } else {
            (segment + 1) * ips_per_segment
        };

        let segment_ips = &ips[start..end];
        let segment_output_file = format!("ip_{}.txt", segment + 1);

        if let Err(err) = write_ips_to_file(&segment_output_file, segment_ips.to_vec()) {
            eprintln!("写入文件 {} 时出错：{}", segment_output_file, err);
        }
    }
}

// txt文件的最大上限写入
fn write_with_max_limit(ips: &Vec<String>) {
    let mut ips_to_write = Vec::new(); // 用于储存待写入文件的IP地址
    let mut lines_written = 1; // 初始行数为1，第一次写入文件，从第1行开始写入
    let mut file_number = 0;
    let mut max_lines_per_file: usize;
    let mut current_file = format!("ip_{}.txt", file_number);
    loop {
        print!("设置每个文件的最多写入多少行(必须大于256行)：");
        io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新输出缓冲区
        max_lines_per_file = get_input_numbers(); // 最大行数
        if max_lines_per_file >= 256 {
            break; // 如果最大行数大于256，退出循环
        }
    }
    for ip in ips.iter() {
        ips_to_write.push(ip.to_string()); // 克隆 IP 地址并添加到 ips_to_write
        if lines_written >= max_lines_per_file {
            file_number += 1;
            current_file = format!("ip_{}.txt", file_number);
            lines_written = 0; // 初始行数归零，写入文件后，在代码后面重新加1：lines_written += 1；

            // 调用函数写入txt文件中
            if let Err(err) = write_ips_to_file(&current_file, ips_to_write.clone()) {
                eprintln!("写入文件 {} 时出错：{}", current_file, err);
            }
            ips_to_write.clear();
        }

        lines_written += 1; // 初始行数的值+1，意为从新文件的第1行开始写
    }

    // 最后一次，写入剩余的 IP 地址
    if !ips_to_write.is_empty() {
        if let Err(err) = write_ips_to_file(&current_file, ips_to_write) {
            eprintln!("写入文件 {} 时出错：{}", current_file, err);
        }
    }
}

// 全部写入到一个txt文件中
fn write_all_to_single_file(output_file: &str, ips: &Vec<String>) {
    if let Err(err) = write_ips_to_file(&output_file, (&ips).to_vec()) {
        eprintln!("写入文件 {} 时出错：{}", output_file, err);
    }
}


fn main() {
    println!("本程序：用于生成IPv4 CIDR范围内的所有IP地址！并多线程写入txt文件。");

    let external_cidr_filename = "ips-v4.txt";
    let output_file = "ip.txt"; // 全部写入，就写入这个文件中
    println!("------------------------------------------------------------------");
    println!("请在下面输入一个或多个CIDR，输入多个时请用空格隔开；");
    println!("(检查输入的内容不合法，就使用外部文件ips-v4.txt的CIDR)");
    print!("请输入您要生成的CIDR：");
    io::stdout().flush().expect("刷新输出缓冲区失败"); // 刷新输出缓冲区
    // 键盘中输入内容
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    println!("------------------------------------------------------------------");
    let mut cidrs: Vec<String> = input.trim().split_whitespace().map(|s| s.to_string()).collect();

    if cidrs.is_empty() || cidrs.iter().any(|cidr| cidr.parse::<IpNetwork>().is_err()) {
        // 命令行窗口中，可以输入一个、多个CIDR，输入多个CIDR用空格隔开，其他情况就使用外部ips-v4.txt文件中的CIDR
        match read_cidr_from_file(external_cidr_filename) {
            Ok(external_cidrs) => {
                cidrs = external_cidrs;
            }
            Err(err) => {
                eprintln!("读取外部{}文件出错：{}", external_cidr_filename, err);
                wait_for_enter();
                std::process::exit(1); // 立即退出程序，返回退出码 1 表示错误
            }
        }
    }
    // 记录开始时间
    let start_generate_time = Instant::now();
    let start_write_time: Instant;
    println!("开始生成IPv4地址...");
    match generate_ips(&cidrs) {
        Ok(ips) => {
            println!("生成的IPv4地址共{}个，消耗时间：{:?}", ips.len(),start_generate_time.elapsed());
            println!("------------------------------------------------------------------");
            let mode = get_write_mode(); // 获取用户选择的写入模式
            start_write_time = Instant::now();
            match mode {
                1 => write_equally_to_files(&ips),
                2 => write_with_max_limit(&ips),
                _ => write_all_to_single_file(&output_file, &ips),
            }
            println!("------------------------------------------------------------------");
            println!("写入txt文件，消耗时间: {:?}", start_write_time.elapsed());
            println!("------------------------------------------------------------------");
        }
        Err(err) => {
            eprintln!("生成IP地址时出错：{}", err);
        }
    }

    println!();
    wait_for_enter();
}
