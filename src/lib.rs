use std::path::PathBuf;

mod download;

pub fn download<P: AsRef<str>>(url: P, thread: usize, output_file: PathBuf) {
    let download = download::Download {
        threads: thread,
        url: url.as_ref().to_owned(),
        memory: 256,
        filename: output_file,
        ..Default::default()
    };

    download.get().unwrap();
}

#[cfg(test)]
mod test {
    use std::{fs::File, io::Read, path::PathBuf};

    use sha2::Digest;
    use sha2::Sha256;

    use crate::download;

    fn calc_sha256(file: PathBuf) -> String {
        // 创建一个新的Sha256对象
        let mut hasher = Sha256::new();

        // 打开文件进行读取
        let mut file = File::open(file).unwrap();

        // 一次性读取文件所有内容并更新哈希状态
        let mut buffer = [0; 1024];
        loop {
            let count = file.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
        }

        // 完成哈希计算
        let result = hasher.finalize();

        // 将哈希结果转换为十六进制字符串
        let hash_hex = hex::encode(result);
        return hash_hex;
    }

    #[test]
    fn test() {
        let p = PathBuf::from("test");
        let p = p.join("tokenizer.model");
        download(
            "https://hf-mirror.com/Qwen/CodeQwen1.5-7B/resolve/main/tokenizer.model?download=true"
                .to_string(),
            8,
            p.clone(),
        );

        assert_eq!(
            calc_sha256(p),
            "656b66a920a54bc45e8e06dc587691ab3c0b2930b9ae56d5fa31e72db2f3bff3"
        );
    }

    #[test]
    fn test2() {
        let p = PathBuf::from("test");
        let p = p.join("test.txt");
        download(
            "https://hf-mirror.com/Qwen/Qwen1.5-MoE-A2.7B/raw/main/merges.txt".to_string(),
            num_cpus::get(),
            p.clone(),
        );

        assert_eq!(
            calc_sha256(p),
            "599bab54075088774b1733fde865d5bd747cbcc7a547c5bc12610e874e26f5e3"
        );
    }

    #[test]
    fn test3() {
        let p = PathBuf::from("test");
        let p = p.join("test.safetensors");
        download(
            "https://hf-mirror.com/Qwen/Qwen1.5-MoE-A2.7B/raw/main/tokenizer.json".to_string(),
            4,
            p.clone(),
        );
        assert_eq!(
            calc_sha256(p),
            "f7c9b2dba4a296b1aa76c16a34b8225c0c118978400d4bb66bff0902d702f5b8"
        );
    }
}
