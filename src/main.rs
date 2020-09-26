use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Inkdropの「勉強」ノートブックにあるノートをNotionへ移行するための整形
fn main() {
    let file = File::open(r"D:\temp\data\inkdrop.txt").unwrap();
    let mut lines: Vec<String> = Vec::new();

    let reg_head2 = { Regex::new(r"^## (?P<title>.+)").unwrap() };
    let reg_head3 = { Regex::new(r"^### (?P<title>.+)").unwrap() };
    let reg_quote = { Regex::new(r"^> (?P<quote>.+)").unwrap() };
    let reg_date = { Regex::new(r"(?P<date>（\d{4}/\d{2}/\d{2}）) {2}").unwrap() };

    for buf_lines in BufReader::new(file).lines() {
        let text = buf_lines.unwrap();

        // <br/> を削除
        let text = text
            .replace("<br>", "")
            .replace("<br/>", "")
            .replace("<br />", "");

        // ## タイトル → # タイトル
        if reg_head2.is_match(&text) {
            lines.push(reg_head2.replace(&text, "# $title").to_string());
            continue;
        }

        // ### 感想 → 感想
        if text == "### 感想" {
            lines.push(text.replace("### 感想", "感想"));
            continue;
        }
        if text == "## 感想" {
            lines.push(text.replace("## 感想", "感想"));
            continue;
        }

        // ### タイトル → ## タイトル
        if reg_head3.is_match(&text) {
            lines.push(reg_head3.replace(&text, "## $title").to_string());
            continue;
        }

        // 引用記号(>)を削除
        if reg_quote.is_match(&text) {
            lines.push(reg_quote.replace(&text, "$quote").to_string());
            continue;
        }
        if text == ">" || text == "> " {
            lines.push("".to_string());
            continue;
        }

        // * カテゴリ: を削除
        if text.starts_with("* カテゴリ:") {
            continue;
        }

        // * ソース: - を* ソース: 書籍に変換
        if text.starts_with("* ソース: -") {
            lines.push("* ソース: 書籍".to_string());
            continue;
        }

        // 日付の下に1つ空行を入れる
        if reg_date.is_match(&text) {
            lines.push(reg_date.replace(&text, "$date").to_string());
            lines.push(String::from(""));
            continue;
        }

        lines.push(text);
    }

    let mut write_file = File::create(r"D:\temp\data\inkdrop.txt").unwrap();
    let write_text = lines.join("\r\n");
    write!(write_file, "{}\r\n", write_text).unwrap();
    write_file.flush().unwrap();
}
