# Hayaemon-rs
**現在開発中です**
現在の機能は

* 再生・停止
* 再生速度の変更

です。また、ASCII文字以外の文字を含む場合は直接入力できない場合があるので、そのファイルのパスをコピー&ペーストをしてみてください。

## About this
[聞々ハヤえもん for Windows](https://github.com/ryotayama/Hayaemon)のCUI版をRustで作ろうとしています。

## How to use
1. このリポジトリの`master`ブランチをクローンしてください(`hayaemon-rs`以下にクローンされたとします)。
2. `hayaemon-rs`直下に`lib/{OS}/{x86 or x86_64}/`という名前のディレクトリを作成してください(`{OS}`は`windows`、`linux`、`osx`のいずれか、`{x86 or x86_64}`は適切な方を入れてください)。
3. 2で作成したライブラリ用ディレクトリにBASS audio library及びそのアドオンのBASS FXのライブラリファイルを格納してください(例: `lib/linux/x86_64/{libbass.so,libbass_fx.so}`)。
4. `hayaemon-rs`ディレクトリで`cargo build --release`を実行してください。

以上の手順で`hayaemon-rs/target/release`に`hayaemon`という実行ファイルが作成されます(Windowsの場合は拡張子(.exe)が付きます)。
(注: UNIX系の場合はcursesライブラリが事前にインストールされているか確認してください。)

## Dependencies
* [BASS](https://www.un4seen.com/) (Audio library)
* [BASS FX](https://www.un4seen.com/) (BASS's add-on)
* [pancurses](https://github.com/ihalila/pancurses) (Curses library)

## License
[GPL v3](https://www.gnu.org/licenses/gpl-3.0.html)
