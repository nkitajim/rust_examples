```
#![allow(...)]（クレートレベル属性）
#[allow(...)]（アイテムレベル属性）

dead_code	未使用の関数、変数、構造体、メソッドなど	fn unused() {}
unused_variables	使われていない変数	let x = 5;
unused_mut	mut が付いているのに変更されていない変数	let mut x = 5;
unused_imports	使われていない use 文	use std::fmt::Debug;
unused_macros	定義したマクロが使われていない	macro_rules! foo {}
non_snake_case	関数や変数名がスネークケースでない	fn MyFunc() {}
non_camel_case_types	型名がキャメルケースでない	struct my_struct;
non_upper_case_globals	グローバル定数が大文字でない	static myVar: i32 = 1;
missing_docs	public API にドキュメントコメントがない	pub fn foo() {}
unused_assignments	代入したけど使われない変数	let mut x = 0; x = 3;
path_statements	文の途中に式があるだけで使われていない	42;
improper_ctypes	FFI 用の型が不適切	extern "C" { fn foo(x: String); }
overflowing_literals	型の範囲を超えるリテラル	let x: u8 = 300;
unstable_features	安定版で未安定の機能を使おうとした	#![feature(...)]
```
