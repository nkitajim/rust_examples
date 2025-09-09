# attribute
```
#[derive(...)]	アイテム	構造体や列挙型にトレイト自動実装	#[derive(Debug, Clone)] struct Point { x:i32, y:i32 }
#[test]	関数	テスト関数としてマーク	#[test] fn it_works() { assert_eq!(2+2,4); }
#[should_panic]	関数	panic が発生することを期待	#[test] #[should_panic] fn test_fail() { panic!("fail"); }
#[should_panic(expected = "...")]	関数	panic メッセージをチェック	#[test] #[should_panic(expected="zero")] fn div_zero() { panic!("zero"); }
#[ignore]	関数	デフォルトでテストをスキップ	#[test] #[ignore] fn long_test() {}
#[allow(...)]	アイテム / クレート	指定した警告を抑制	#[allow(dead_code)] fn foo() {}
#![allow(...)]	クレート	クレート全体の警告を抑制	#![allow(non_snake_case)]
#[cfg(...)]	アイテム	条件付きコンパイル	#[cfg(target_os="linux")] fn linux_only() {}
#![cfg(...)]	クレート	クレート全体で条件付きコンパイル	#![cfg(feature="awesome")]
#[inline]	関数	インライン化をヒント	#[inline] fn add(a:i32,b:i32)->i32 { a+b }
#[inline(always)]	関数	常にインライン化するよう指示	#[inline(always)] fn fast() {}
#[inline(never)]	関数	インライン化を避ける	#[inline(never)] fn slow() {}
#[macro_use]	クレート / モジュール	マクロをスコープに取り込む	#[macro_use] extern crate serde_derive;
#[no_mangle]	関数	名前をそのまま出力（FFI用）	#[no_mangle] pub extern "C" fn foo() {}
#[repr(...)]	構造体 / 列挙型	メモリ表現を指定	#[repr(C)] struct S { a:i32 }
#[allow(unused)]	アイテム	未使用の警告を抑制	#[allow(unused)] fn foo() {}
#[warn(...)]	アイテム / クレート	指定警告を有効化	#[warn(non_snake_case)] fn Foo() {}
#![warn(...)]	クレート	クレート全体で警告有効化	#![warn(missing_docs)]
#[path = "xxx.rs"]	モジュール	モジュールのファイルパスを指定	#[path="other.rs"] mod other;
#[doc(...)]	アイテム	ドキュメント属性	#[doc = "This is a point"] struct Point {}
#[must_use]	関数 / 型	戻り値を必ず使うよう警告	#[must_use] fn compute() -> i32 { 42 }
#[deprecated]	アイテム	非推奨と表示	#[deprecated(note="use new_fn")] fn old_fn() {}
```
