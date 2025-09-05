# default prelude
std::prelude::v1

トレイトを読み込まなくても、defaultで使えるもの
```
- トレイト
Copy, Clone
Sized, Drop
Eq, PartialEq, Ord, PartialOrd
Fn, FnMut, FnOnce
Iterator, FromIterator
IntoIterator
Option / Result の関連トレイト（FromResidual, Try）
ToOwned （主に &str → String などで利用）

- 型（構造体・列挙型）
Box
String
ToString
Vec
Option
Result
range 系：Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive

- モジュールやマクロ
macro_rules! で定義された標準マクロ（vec!, format!, dbg!, assert!, panic! など）は別途 std からインポートされますが、これも「プレリュードに含まれている」とよく説明されます。
```
