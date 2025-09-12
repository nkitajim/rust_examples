# 4. アイデア系（Rust らしい型設計・列挙型活用）
```
- 簡易ゲーム状態管理
enum GameState { Start, Playing, GameOver }
状態遷移を関数で表現し、パターンマッチで処理。

- カスタム JSON パーサー
小さな subset JSON ({ "key": "value" }) を enum JsonValue { Object(...), String(...) } で表現
再帰的 enum とパターンマッチの練習。

- オブジェクトプール
struct Object { ... } と Vec<Option<Object>> でリソース管理
所有権とライフタイムの管理の練習。
```
