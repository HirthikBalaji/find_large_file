

# 📁 **Find Large Files (Rust CLI Tool)**

A fast and efficient Rust command-line tool that recursively scans a directory and lists the **largest space-consuming files**.

Built using:

* ⚡ **walkdir** – fast recursive directory traversal
* 📏 **humansize** – human-friendly size formatting
* 🦀 **Rust** – performance + safety

---

## 🚀 Features

* ✅ Recursively scans all subfolders
* ✅ Displays largest files (default: top 10)
* ✅ Human-readable file sizes (MB/GB)
* ✅ Cross-platform (Windows/Linux/macOS)
* ✅ Lightweight and blazing fast

---

## 📦 Installation

```bash
git clone https://github.com/HirthikBalaji/find_large_file.git
cd find_large_file
cargo build --release
```

The compiled binary will be located at:

```
target/release/find_large_file
```

(Windows: `find_large_file.exe`)

---

## ▶️ Usage

### Basic scan

```
cargo run -- <directory>
```

### Scan with custom limit

```
cargo run -- <directory> <top_n>
```

### Examples

Scan D drive (top 10 files):

```
cargo run -- D:\ 10
```

Scan Downloads folder:

```
cargo run -- "C:\Users\Hirthik\Downloads" 20
```

Scan current folder:

```
cargo run -- . 15
```

---

## 📜 Example Output

```
Top 10 largest files in 'D:\':

1.23 GB     D:\Movies\sample.mp4
892.11 MB   D:\Backups\data.zip
421.55 MB   D:\Models\model.pt
...
```

---

## 🧩 Project Structure

```
find_large_file/
├── Cargo.toml
└── src/
    └── main.rs
```

---

## 📚 Dependencies

```toml
[dependencies]
walkdir = "2.5"
humansize = "2.1"
```

---

## 🤝 Contributions

PRs and feature requests are welcome!
Want CSV export, file-type filters, multithreading, or clap-based CLI?
Create an issue and I'll help refine it!

---

## 🔗 Author / GitHub Profile

**Hirthik Balaji**
🔗 GitHub: [https://github.com/HirthikBalaji/](https://github.com/HirthikBalaji/)

---

## 📜 License

This project is licensed under the **MIT License**.


