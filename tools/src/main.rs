use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::env;

#[derive(Debug)]
struct WordEntry {
    word: String,
    part_of_speech: String,
    translation: String,
}

fn main() {
    // 获取项目根目录路径（Cargo.toml所在目录）
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("example.txt");
    
    println!("尝试从路径读取: {}", path.display());
    
    // 检查文件是否存在
    if !path.exists() {
        println!("错误：文件不存在！");
        println!("当前工作目录: {}", env::current_dir().unwrap().display());
        println!("文件应在: {}", path.display());
        return;
    }
    
    let words = match read_words_file(&path) {
        Ok(words) => words,
        Err(e) => {
            println!("读取文件失败: {}", e);
            return;
        }
    };
    
    // 开始背诵
    start_recitation(&words);
}

fn read_words_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<WordEntry>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut words = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        let line = line.trim(); // 去除两端空白
        if line.is_empty() {
            continue; // 跳过空行
        }
        
        let parts: Vec<&str> = line.split("::").collect();
        
        if parts.len() == 3 {
            words.push(WordEntry {
                word: parts[0].trim().to_string(),
                part_of_speech: parts[1].trim().to_string(),
                translation: parts[2].trim().to_string(),
            });
        } else {
            eprintln!("警告：忽略无效行 - {}", line);
        }
    }
    
    Ok(words)
}

fn start_recitation(words: &[WordEntry]) {
    println!("开始单词背诵 (共{}个单词)", words.len());
    println!("输入任意键查看答案，输入q退出");
    
    for (i, entry) in words.iter().enumerate() {
        println!("\n({}) {}", i + 1, entry.word);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取输入失败");
        
        if input.trim().to_lowercase() == "q" {
            println!("退出背诵");
            return;
        }
        
        println!("{} {}", entry.part_of_speech, entry.translation);
    }
    
    println!("\n所有单词背诵完成！");
}