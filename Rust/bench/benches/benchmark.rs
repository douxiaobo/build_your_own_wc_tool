use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
// use criterion::BenchmarkId;
use std::io::{ BufRead, BufReader};



// 假设您的函数在 lib.rs 或 main.rs 中声明，并已经标记为 pub
pub fn your_split_function(path: &Path) -> io::Result<()> {
    // ... 函数实现 ...
    // 打开指定路径的文件，使用 `?` 操作符处理可能的错误
    let file = File::open(path)?;
    // 创建一个缓冲读取器来提高文件读取效率
    let reader = BufReader::new(file);

    // 初始化计数器
    let mut _linecount = 0;  // 行数
    let mut _charcount = 0;  // 字符数
    let mut _wordscount = 0; // 单词数
    let mut current_word = false;   // 标记是否在单词内部
    let mut _bytescount = 0;  // 字节数

    // 使用 `split` 方法按行读取文件内容，这里使用 `\n` 作为分隔符
    for line_result in reader.split(b'\n') {
        // 解包每一行的字节数据，错误则立即返回
        let line_bytes = line_result?;
        // 尝试将字节数据转换为字符串，若失败则转换错误并返回
        let line_str = std::str::from_utf8(&line_bytes)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        // 行数加一
        _linecount += 1;

        // 计算单词数
        let mut chars_iter = line_str.chars();
        for ch in chars_iter.by_ref() {
            if ch.is_whitespace() {
                if current_word {
                    _wordscount += 1;    // 单词计数加一
                    current_word = false;   // 重置单词标记
                }
            } else {
                current_word = true;    // 在单词内部
            }
        }
        // 如果当前处于单词中，单词计数加一
        if current_word {
            _wordscount += 1;
            current_word = false;
        }

        _charcount += line_str.chars().count()+1;

        // 计算字节总数，考虑 "\r\n" 的情况
        _bytescount += line_bytes.len()+1;
    }

    // 输出统计结果
    // println!("linecount:{}", linecount);
    // println!("charcount:{}", charcount);
    // println!("wordscount:{}", wordscount);
    // println!("bytescount:{}", bytescount);

    // 函数成功完成，返回 Ok
    Ok(())
}

pub fn your_read_to_string_function(path: &Path) -> io::Result<()> {
    // ... 函数实现 ...
// 打开指定路径的文件，使用 `?` 操作符处理可能的错误
    let mut file = File::open(path)?;
    // 创建一个字符串缓冲区
    let mut contents = String::new();
    // 读取整个文件到字符串缓冲区
    file.read_to_string(&mut contents)?;

    // 初始化计数器
    let mut _linecount = 0;  // 行数
    let mut _charcount = 0;  // 字符数
    let mut _wordscount = 0; // 单词数
    let mut current_word = false;   // 标记是否在单词内部
    let mut _bytescount = 0;  // 字节数

    // 按行分割字符串
    for line in contents.lines() {
        // 行数加一
        _linecount += 1;

        // 计算单词数
        let mut chars_iter = line.chars();
        for ch in chars_iter.by_ref() {
            if ch.is_whitespace() {
                if current_word {
                    _wordscount += 1;    // 单词计数加一
                    current_word = false;   // 重置单词标记
                }
            } else {
                current_word = true;    // 在单词内部
            }
            // charcount += 1; // 总字符数加一
        }
        // 如果当前处于单词中，单词计数加一
        if current_word {
            _wordscount += 1;
            current_word = false;
        }
        
        _charcount += line.chars().count()+1;
        
        _bytescount += line.len()+2;
    }

    // 输出统计结果
    // println!("linecount:{}", linecount);
    // println!("charcount:{}", charcount);
    // println!("wordscount:{}", wordscount);
    // println!("bytescount:{}",bytescount);

    // 函数成功完成，返回 Ok
    Ok(())
}

fn bench_split_function(c: &mut Criterion) {
    c.bench_function("split", |b| {
        b.iter(|| {
            your_split_function(black_box(Path::new("./test.txt"))).unwrap();
        })
    });
}

fn bench_read_to_string_function(c: &mut Criterion) {
    c.bench_function("read_to_string", |b| {
        b.iter(|| {
            your_read_to_string_function(black_box(Path::new("./test.txt"))).unwrap();
        })
    });
}

// fn bench_file_processing(c: &mut Criterion) {
//     let path = Path::new("./test.txt");

//     c.bench_with_input(BenchmarkId::new("split", "your_split_function"), &path, |b, _| {
//         b.iter(|| {
//             your_split_function(black_box(&path)).unwrap();
//         })
//     });

//     c.bench_with_input(BenchmarkId::new("read_to_string", "your_read_to_string_function"), &path, |b, _| {
//         b.iter(|| {
//             your_read_to_string_function(black_box(&path)).unwrap();
//         })
//     });
// }


criterion_group!(benches, bench_split_function, bench_read_to_string_function);
criterion_main!(benches);



// douxiaobo@192 Rust % rustup default nightly
// info: syncing channel updates for 'nightly-aarch64-apple-darwin'
// info: latest update on 2024-06-16, rust version 1.81.0-nightly (3cf924b93 2024-06-15)
// info: downloading component 'cargo'
// info: downloading component 'clippy'
// info: downloading component 'rust-docs'
//  15.7 MiB /  15.7 MiB (100 %)   7.4 MiB/s in  2s ETA:  0s
// info: downloading component 'rust-std'
//  24.8 MiB /  24.8 MiB (100 %)   5.5 MiB/s in  4s ETA:  0s
// info: downloading component 'rustc'
//  51.6 MiB /  51.6 MiB (100 %)   5.7 MiB/s in  8s ETA:  0s
// info: downloading component 'rustfmt'
// info: installing component 'cargo'
// info: installing component 'clippy'
// info: installing component 'rust-docs'
//  15.7 MiB /  15.7 MiB (100 %)   4.3 MiB/s in  3s ETA:  0s
// info: installing component 'rust-std'
//  24.8 MiB /  24.8 MiB (100 %)  24.2 MiB/s in  1s ETA:  0s
// info: installing component 'rustc'
//  51.6 MiB /  51.6 MiB (100 %)  25.3 MiB/s in  1s ETA:  0s
// info: installing component 'rustfmt'
// info: default toolchain set to 'nightly-aarch64-apple-darwin'

//   nightly-aarch64-apple-darwin installed - rustc 1.81.0-nightly (3cf924b93 2024-06-15)

// douxiaobo@192 Rust % cargo new benches_split
//     Creating binary (application) `benches_split` package
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

// douxiaobo@192 Rust % cd bench
// douxiaobo@192 bench % cargo init --lib
//     Creating library package
// note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
// douxiaobo@192 bench % cargo bench
//     Updating crates.io index
//   Downloaded plotters-svg v0.3.6
//   Downloaded csv-core v0.1.11
//   Downloaded oorandom v11.1.3
//   Downloaded either v1.12.0
//   Downloaded cast v0.3.0
//   Downloaded criterion-plot v0.4.5
//   Downloaded textwrap v0.11.0
//   Downloaded same-file v1.0.6
//   Downloaded walkdir v2.5.0
//   Downloaded plotters-backend v0.3.6
//   Downloaded serde_cbor v0.11.2
//   Downloaded tinytemplate v1.2.1
//   Downloaded criterion v0.3.6
//   Downloaded half v1.8.3
//   Downloaded crossbeam-utils v0.8.20
//   Downloaded num-traits v0.2.19
//   Downloaded proc-macro2 v1.0.85
//   Downloaded serde v1.0.203
//   Downloaded serde_derive v1.0.203
//   Downloaded memchr v2.7.4
//   Downloaded itertools v0.10.5
//   Downloaded serde_json v1.0.117
//   Downloaded plotters v0.3.6
//   Downloaded clap v2.34.0
//   Downloaded regex v1.10.5
//   Downloaded syn v2.0.66
//   Downloaded regex-syntax v0.8.4
//   Downloaded unicode-width v0.1.13
//   Downloaded regex-automata v0.4.7
//   Downloaded libc v0.2.155
//   Downloaded csv v1.3.0
//   Downloaded 31 crates (4.9 MB) in 2.24s
//    Compiling crossbeam-utils v0.8.20
//    Compiling serde v1.0.203
//    Compiling proc-macro2 v1.0.85
//    Compiling autocfg v1.3.0
//    Compiling unicode-ident v1.0.12
//    Compiling either v1.12.0
//    Compiling ryu v1.0.18
//    Compiling itoa v1.0.11
//    Compiling rayon-core v1.12.1
//    Compiling libc v0.2.155
//    Compiling serde_json v1.0.117
//    Compiling memchr v2.7.4
//    Compiling unicode-width v0.1.13
//    Compiling plotters-backend v0.3.6
//    Compiling regex-syntax v0.8.4
//    Compiling textwrap v0.11.0
//    Compiling itertools v0.10.5
//    Compiling plotters-svg v0.3.6
//    Compiling same-file v1.0.6
//    Compiling num-traits v0.2.19
//    Compiling half v1.8.3
//    Compiling csv-core v0.1.11
//    Compiling cast v0.3.0
//    Compiling bitflags v1.3.2
//    Compiling walkdir v2.5.0
//    Compiling clap v2.34.0
//    Compiling oorandom v11.1.3
//    Compiling lazy_static v1.4.0
//    Compiling regex-automata v0.4.7
//    Compiling criterion-plot v0.4.5
//    Compiling regex v1.10.5
//    Compiling crossbeam-epoch v0.9.18
//    Compiling crossbeam-deque v0.8.5
//    Compiling quote v1.0.36
//    Compiling syn v2.0.66
//    Compiling atty v0.2.14
//    Compiling rayon v1.10.0
//    Compiling serde_cbor v0.11.2
//    Compiling csv v1.3.0
//    Compiling plotters v0.3.6
//    Compiling tinytemplate v1.2.1
//    Compiling serde_derive v1.0.203
//    Compiling criterion v0.3.6
//    Compiling bench v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/build_your_own_wc_tool/Rust/bench)
// douxiaobo@192 bench % cargo bench
//     Finished `bench` profile [optimized] target(s) in 0.03s
//      Running unittests src/lib.rs (target/release/deps/bench-83cb1408a3d0f2c4)

// running 1 test
// test tests::it_works ... ignored

// test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running benches/benchmark.rs (target/release/deps/benchmark-a9993166dfb13859)

// running 0 tests

// test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

// douxiaobo@192 bench % 

// Compiling your_package_name v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/build_your_own_wc_tool/Rust/bench)
// warning: variable `linecount` is assigned to, but never used
//   --> benches/benchmark.rs:19:13
//    |
// 19 |     let mut linecount = 0;  // 行数
//    |             ^^^^^^^^^
//    |
//    = note: consider using `_linecount` instead
//    = note: `#[warn(unused_variables)]` on by default

// warning: variable `charcount` is assigned to, but never used
//   --> benches/benchmark.rs:20:13
//    |
// 20 |     let mut charcount = 0;  // 字符数
//    |             ^^^^^^^^^
//    |
//    = note: consider using `_charcount` instead

// warning: variable `wordscount` is assigned to, but never used
//   --> benches/benchmark.rs:21:13
//    |
// 21 |     let mut wordscount = 0; // 单词数
//    |             ^^^^^^^^^^
//    |
//    = note: consider using `_wordscount` instead

// warning: variable `bytescount` is assigned to, but never used
//   --> benches/benchmark.rs:23:13
//    |
// 23 |     let mut bytescount = 0;  // 字节数
//    |             ^^^^^^^^^^
//    |
//    = note: consider using `_bytescount` instead

// warning: variable `linecount` is assigned to, but never used
//   --> benches/benchmark.rs:80:13
//    |
// 80 |     let mut linecount = 0;  // 行数
//    |             ^^^^^^^^^
//    |
//    = note: consider using `_linecount` instead

// warning: variable `charcount` is assigned to, but never used
//   --> benches/benchmark.rs:81:13
//    |
// 81 |     let mut charcount = 0;  // 字符数
//    |             ^^^^^^^^^
//    |
//    = note: consider using `_charcount` instead

// warning: variable `wordscount` is assigned to, but never used
//   --> benches/benchmark.rs:82:13
//    |
// 82 |     let mut wordscount = 0; // 单词数
//    |             ^^^^^^^^^^
//    |
//    = note: consider using `_wordscount` instead

// warning: variable `bytescount` is assigned to, but never used
//   --> benches/benchmark.rs:84:13
//    |
// 84 |     let mut bytescount = 0;  // 字节数
//    |             ^^^^^^^^^^
//    |
//    = note: consider using `_bytescount` instead

// warning: `your_package_name` (bench "benchmark") generated 8 warnings
//     Finished `bench` profile [optimized] target(s) in 0.50s
//      Running unittests src/lib.rs (target/release/deps/your_package_name-055ff5669348ed04)

// running 1 test
// test tests::it_works ... ignored

// test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running benches/benchmark.rs (target/release/deps/benchmark-cea4dc8d4be040a8)
// WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
// This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

// Gnuplot not found, using plotters backend
// split                   time:   [504.57 µs 505.97 µs 507.46 µs]                  
// Found 3 outliers among 100 measurements (3.00%)
//   1 (1.00%) high mild
//   2 (2.00%) high severe

// read_to_string          time:   [315.50 µs 317.03 µs 318.52 µs]                           

// douxiaobo@192 bench % 

// douxiaobo@192 bench % cargo bench
//    Compiling your_package_name v0.1.0 (/Users/douxiaobo/Documents/Practice in Coding/build_your_own_wc_tool/Rust/bench)
//     Finished `bench` profile [optimized] target(s) in 0.48s
//      Running unittests src/lib.rs (target/release/deps/your_package_name-055ff5669348ed04)

// running 1 test
// test tests::it_works ... ignored

// test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s

//      Running benches/benchmark.rs (target/release/deps/benchmark-cea4dc8d4be040a8)
// WARNING: HTML report generation will become a non-default optional feature in Criterion.rs 0.4.0.
// This feature is being moved to cargo-criterion (https://github.com/bheisler/cargo-criterion) and will be optional in a future version of Criterion.rs. To silence this warning, either switch to cargo-criterion or enable the 'html_reports' feature in your Cargo.toml.

// Gnuplot not found, using plotters backend
// split                   time:   [506.44 µs 507.34 µs 508.06 µs]                  
//                         change: [-0.9154% -0.4256% +0.0516%] (p = 0.09 > 0.05)
//                         No change in performance detected.

// read_to_string          time:   [311.34 µs 312.28 µs 313.44 µs]                           
//                         change: [-1.3752% -0.9339% -0.4229%] (p = 0.00 < 0.05)
//                         Change within noise threshold.
// Found 23 outliers among 100 measurements (23.00%)
//   20 (20.00%) high mild
//   3 (3.00%) high severe

// douxiaobo@192 bench % 