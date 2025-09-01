use mdbook::book::{Book, BookItem};
use mdbook::preprocess::{Preprocessor, PreprocessorContext, CmdPreprocessor};
use mdbook::errors::{Error, Result};
use pulldown_cmark::{Event, Parser, Tag};
use std::collections::HashMap;
use std::io;
use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    match args.next().as_deref() {
        Some("supports") => {
            // Supports all renderers.
            return;
        }
        Some(arg) => {
            eprintln!("unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }

    if let Err(e) = handle_preprocessing() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

struct AddReferences;

impl Preprocessor for AddReferences {
    fn name(&self) -> &str {
        "add-references"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        // 第一遍：收集所有引用关系
        let references = collect_references(&book)?;

        // 第二遍：为卡牌文档添加引用段落
        add_reference_sections(&mut book, &references)?;

        Ok(book)
    }
}

// 收集所有引用关系
fn collect_references(book: &Book) -> Result<HashMap<String, Vec<String>>> {
    let mut references: HashMap<String, Vec<String>> = HashMap::new();

    for item in book.iter() {
        if let BookItem::Chapter(ch) = item {
            if ch.is_draft_chapter() {
                continue;
            }

            let chapter_path = ch.path.as_ref()
                .ok_or_else(|| Error::msg("Chapter has no path"))?
                .to_str()
                .ok_or_else(|| Error::msg("Path is not valid UTF-8"))?;

            // 跳过CHANGELOG.md和SUMMARY.md
            if chapter_path == "CHANGELOG.md" || chapter_path == "SUMMARY.md" {
                continue;
            }

            // 解析章节中的链接
            let links = extract_links(&ch.content)?;

            for link in &links {
                // 标准化链接路径
                let normalized_link = normalize_link(&link, chapter_path);
                references.entry(normalized_link)
                    .or_insert_with(Vec::new)
                    .push(chapter_path.to_string());
            }
        }
    }

    Ok(references)
}

// 提取章节中的所有链接
fn extract_links(content: &str) -> Result<Vec<String>> {
    let mut links = Vec::new();
    let parser = Parser::new(content);

    for event in parser {
        if let Event::Start(Tag::Link { dest_url, .. }) = event {
            if dest_url.ends_with(".md") {
                links.push(dest_url.to_string());
            }
        }
    }

    Ok(links)
}

// 标准化链接路径为相对于src/的路径
fn normalize_link(link: &str, chapter_path: &str) -> String {
    // 如果链接已经是绝对路径（相对于src/），直接返回
    if !link.starts_with("../") && !link.starts_with("./") && !link.starts_with('/') {
        // 获取章节目录
        let chapter_dir = chapter_path.rsplitn(2, '/').nth(1).unwrap_or("");
        if !chapter_dir.is_empty() {
            format!("{}/{}", chapter_dir, link)
        } else {
            link.to_string()
        }
    } else {
        // 处理相对路径
        let chapter_dir = chapter_path.rsplitn(2, '/').nth(1).unwrap_or("");

        // 解析相对路径
        let mut result_parts = Vec::new();
        if !chapter_dir.is_empty() {
            result_parts.extend(chapter_dir.split('/'));
        }

        let link_parts: Vec<&str> = link.split('/').collect();

        for part in link_parts {
            match part {
                ".." => { result_parts.pop(); },
                "." => {},
                _ => result_parts.push(part),
            }
        }

        result_parts.join("/")
    }
}

// 为卡牌文档添加引用段落
fn add_reference_sections(book: &mut Book, references: &HashMap<String, Vec<String>>) -> Result<()> {
    book.for_each_mut(|item| {
        if let BookItem::Chapter(ch) = item {
            if ch.is_draft_chapter() {
                return;
            }

            let chapter_path = ch.path.as_ref()
                .and_then(|p| p.to_str())
                .unwrap_or("");

            // 只处理src/卡牌/下的文档
            if !chapter_path.starts_with("卡牌/") {
                return;
            }

            // 获取引用这个文档的其他文档
            if let Some(refs) = references.get(chapter_path) {
                // 使用HashSet去重引用文档
                use std::collections::HashSet;
                let unique_refs: HashSet<_> = refs.iter().collect();

                let mut ref_links = Vec::new();

                for ref_path in unique_refs {
                    // 计算相对路径
                    let relative_path = calculate_relative_path(chapter_path, ref_path);
                    let file_stem = Path::new(ref_path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or(ref_path);

                    ref_links.push(format!("[{}]({})", file_stem, relative_path));
                }

                if !ref_links.is_empty() {
                    // 在文档末尾添加引用段落
                    ch.content.push_str("\n\n## 被引用\n\n");
                    ch.content.push_str(&ref_links.join(" "));
                    ch.content.push('\n');
                }
            }
        }
    });

    Ok(())
}

// 计算从当前文档到引用文档的相对路径
fn calculate_relative_path(from: &str, to: &str) -> String {
    let from_parts: Vec<&str> = from.split('/').collect();
    let to_parts: Vec<&str> = to.split('/').collect();

    // 找到共同前缀的长度
    let mut common_len = 0;
    for i in 0..from_parts.len().min(to_parts.len()) {
        if from_parts[i] == to_parts[i] {
            common_len = i + 1;
        } else {
            break;
        }
    }

    // 计算需要向上走的层数
    let up_count = from_parts.len().saturating_sub(common_len).saturating_sub(1); // -1 因为要去掉文件名

    // 构建相对路径
    let mut relative_parts = Vec::new();

    // 添加向上走的 ..
    for _ in 0..up_count {
        relative_parts.push("..");
    }

    // 添加向下走的路径
    for &part in &to_parts[common_len..] {
        relative_parts.push(part);
    }

    if relative_parts.is_empty() {
        ".".to_string()
    } else {
        relative_parts.join("/")
    }
}

pub fn handle_preprocessing() -> Result<()> {
    let pre = AddReferences;
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
