pub mod arcaea;

pub const BASE_URL: &'static str = "https://q.trap.jp/api/1.0";

pub const HELP_TEXT: &'static str = r#"## このBotの使い方
スラッシュコマンド形式での#gps/times/xecuaへの投稿、あるいはこのbotへのメンションを行うと該当する内容を実行します
+ `/help` : このヘルプを出します
+ `/random [difficulty|level]` : 全曲全譜面から適当にお題を出します
  + `difficulty`に難易度(PAST/PST, PRESENT/PRS, FUTURE/FTR)を空白区切りで指定すると、その中からのみ出題します
  + `level`にレベル値(1~9,9+,10)を指定すると、その中からのみ出題します
## おまけ機能
+ `/stamp [n] [stamp_list]`: n個のスタンプをランダムで召喚します 省略した場合n=1です stamp_listは空白区切りでスタンプ名を置くとそこからn個選択します(ちなみに存在するかはチェックしません) 省略するとtraQから引っ張ってきます
+ /omikuji おみくじ代行サービス代行サービスです
"#;
