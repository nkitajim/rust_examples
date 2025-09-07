# std::ops
ジェネリック関数を使う場合には場合には型シグネチャが必要
```
- 算術演算系
Add (a + b)
Sub (a - b)
Mul (a * b)
Div (a / b)
Rem (a % b)
Neg (-a)

- 代入付き算術演算
AddAssign (a += b)
SubAssign (a -= b)
MulAssign (a *= b)
DivAssign (a /= b)
RemAssign (a %= b)

- ビット演算系
BitAnd (a & b)
BitOr (a | b)
BitXor (a ^ b)
Not (!a)
Shl (a << b)
Shr (a >> b)

- 代入付きビット演算
BitAndAssign (a &= b)
BitOrAssign (a |= b)
BitXorAssign (a ^= b)
ShlAssign (a <<= b)
ShrAssign (a >>= b)

- その他便利系

Index (a[b])
IndexMut (a[b] = x)
RangeBounds (.., a..b, ..=b など範囲式)
Deref (*a)
DerefMut (可変参照の *a)
Drop (スコープを抜けるときの後処理)
Fn, FnMut, FnOnce (クロージャ・関数呼び出し)
Try (? 演算子の仕組み)
```
