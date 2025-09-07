# std trait
std traitで一般的なものの階層
意外と少ない
```
std
├─ prelude  (自動インポート)
│   ├─ 基本型: i32, u32, f32, bool, char, String, Vec<T>...
│   ├─ トレイト: Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Drop
│   └─ マクロ: println!, print!, dbg!, vec!, assert!...
│
├─ collections
│   ├─ Vec<T>, VecDeque<T>, LinkedList<T>
│   ├─ HashMap<K,V>, BTreeMap<K,V>
│   └─ HashSet<T>, BTreeSet<T>
│
├─ result & option
│   ├─ Result<T,E>
│   ├─ Option<T>
│   └─ error::Error (トレイト)
│
├─ io & fs
│   ├─ io
│   │   ├─ Read, Write, BufRead, Seek
│   │   ├─ stdin(), stdout(), stderr()
│   │   └─ BufReader, BufWriter
│   └─ fs
│       ├─ File
│       ├─ read_to_string, write
│       └─ metadata, remove_file...
│
├─ net
│   ├─ TcpStream, TcpListener
│   └─ UdpSocket
│
├─ thread & sync
│   ├─ thread::spawn, sleep
│   ├─ sync::Mutex<T>, RwLock<T>, Arc<T>
│   └─ sync::mpsc (channel)
│
├─ time
│   ├─ Duration, Instant, SystemTime
│
├─ env & process
│   ├─ env::args, env::var
│   └─ process::Command, process::exit
│
├─ string & char
│   ├─ string::String
│   ├─ str (文字列スライス)
│   └─ char
│
├─ num & ops & cmp
│   ├─ num: 数値変換、定数
│   ├─ ops: Add, Sub, Mul, Div...
│   └─ cmp: Ordering, min, max
│
├─ fmt
│   ├─ Display, Debug
│   └─ write!, format!, format_args!
│
└─ macro_rules!
    └─ カスタムマクロ定義用
```
