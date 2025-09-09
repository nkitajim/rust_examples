```
macro_rules! square {
	{ $e: expr } => { $e * $e }
}

expr	式 (expression)	1 + 2, "hello", some_func(x)
ident	識別子 (identifier)	x, my_var, Result
ty	型 (type)	i32, String, Vec<u8>
path	パス	std::io::Result, my_mod::MyType
block	ブロック	{ println!("hi"); }
stmt	文 (statement)	let x = 3;, return 42;
pat	パターン (pattern)	Some(x), None, (a, b)
tt	トークンツリー (token tree)	ほぼ何でも（{ ... } や foo!() の中身など）
item	アイテム (関数・struct・impl など)	fn foo() {}, struct Point;
meta	属性 (attribute)	cfg(feature = "a")
```
その他に、手続きマクロとして、関数的マクロ、属性マクロ、deriveマクロなどがある
