use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::net::Ipv4Addr; // 正确导入的位置
use ipnetwork::Ipv4Network;

fn main() {
	// 删除已存在的文件
    delete_existing_files();
    print!("请输入一个有效的 CIDR：");
    io::stdout().flush().expect("刷新 stdout 失败");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");

    let input = input.trim();

    if let Ok(cidr) = input.parse::<Ipv4Network>() {
        let mut file = File::create("ip_all.txt").expect("创建文件失败");

        for ip in cidr.iter() {
            writeln!(file, "{}", ip).expect("写入文件失败");
        }
        print!("全部的 IP 地址已写入 ip.txt\n\n下面将分割成若干个 TXT 文件>>>\n请输入每个 TXT 文件的 IP 数量上限：");
        io::stdout().flush().expect("刷新 stdout 失败");

        let mut limit_input = String::new();
        io::stdin().read_line(&mut limit_input).expect("读取输入失败");

        if let Ok(limit) = limit_input.trim().parse::<usize>() {
            let ip_vec: Vec<Ipv4Addr> = cidr.iter().collect();
            let ip_count = ip_vec.len();
            let digits = calculate_digits(ip_count, limit);
            let mut file_counter = 1; // 修复初始值从 1 开始
            let mut ip_counter = 0;
			
            let mut file = create_file(file_counter, digits);
			
            for ip in ip_vec {
                writeln!(file, "{}", ip).expect("写入文件失败");
                ip_counter += 1;

                if ip_counter >= limit {
                    ip_counter = 0;
                    file_counter += 1;
                    file = create_file(file_counter, digits);
                }
            }

            println!("IP 地址已根据上限数量写入多个 TXT 文件");
        } else {
            println!("无效的上限数量");
        }
    } else {
        println!("无效的 CIDR");
    }
	// 等待用户输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn calculate_digits(total_ip: usize, limit: usize) -> usize {
    let file_count = (total_ip + limit - 1) / limit; // 计算需要的文件数量
    let digits = file_count.to_string().len(); // 计算需要的位数
    digits
}

fn create_file(counter: usize, digits: usize) -> File {
    let filename = format!("ip_{:0width$}.txt", counter, width = digits);
    let file = File::create(&filename).expect("创建文件失败");
    file
}


fn delete_existing_files() {
    let files = fs::read_dir(".").expect("读取目录失败");

    for file in files {
        if let Ok(file) = file {
            let filename = file.file_name().into_string().unwrap();
            if filename.starts_with("ip") && filename.ends_with(".txt") {
                if let Err(err) = fs::remove_file(&filename) {
                    println!("删除文件 {} 失败: {:?}", filename, err);
                }
            }
        }
    }
}