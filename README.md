# pget
parallel download file and resume download process after interrupt

Used to solve the network instability issues encountered during large file downloads, supporting resumable downloads and concurrent downloads.


pget will create a .cache director to store the download cache, the download cache will be helpful for resuming download, you can choose keep the cache or not.
# How to use
Add the dependency

```shell
cargo add pget  # --features progress_bar
```

progress_bar feature will use a default progress bar, if you don't enable this feature, no grogress bar would be shown in the terminal.

Use the crate:

* normal download

```rust
use std::path::PathBuf;
let p = PathBuf::from("test");
let p = p.join("tokenizer.model");
pget::download(
    "https://hf-mirror.com/NexaAIDev/Octopus-v2/resolve/main/tokenizer.model".to_string(),
    num_cpus::get(),
    p.clone(),
).unwrap();
```

* download with cache clean
```rust
use std::path::PathBuf;
 let p = PathBuf::from("test");
 let p = p.join("merges.txt");
 pget::download_with_cleaner(
 "https://hf-mirror.com/Qwen/Qwen1.5-MoE-A2.7B/raw/main/merges.txt".to_string(),
 num_cpus::get(),
 p.clone(),
 false //keep the cache or not
).unwrap();
```
* download with custom progress bar
```rust
use std::path::PathBuf;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
let progress = ProgressBar::new(0);
progress.set_style(
 ProgressStyle::with_template(
    "{msg} [{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} {bytes_per_sec} ({eta})",
                ).unwrap(),
            );
 progress.set_message("tokenizer");
 let url =
 "https://hf-mirror.com/Qwen/Qwen1.5-MoE-A2.7B/raw/main/tokenizer.json".to_string();
 let p = PathBuf::from("test");
 let p = p.join("qwen.safetensors");
 pget::download_with_custom_progress(url, 4, p.clone(), Some(progress),false).unwrap();
```
