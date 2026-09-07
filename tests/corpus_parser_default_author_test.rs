//! C2: Author default by category (TDD).
//!
//! Verifies that MarkdownParser defaults the author field based on the
//! frontmatter `category` value when no explicit author is present.

use mao_agent::MarkdownParser;

#[test]
fn test_history_category_defaults_author_to_mao() {
    let md = "---
title: \"测试文献\"
date: \"1938-05\"
category: \"history\"
---

正文内容。
";
    let doc = MarkdownParser::parse_str(md, Some("test.md")).unwrap();
    assert_eq!(doc.metadata.author, "毛泽东");
}

#[test]
fn test_engineering_category_defaults_author_to_unknown() {
    let md = "---
title: \"测试文献\"
date: \"2024-01\"
category: \"engineering\"
---

正文内容。
";
    let doc = MarkdownParser::parse_str(md, Some("test.md")).unwrap();
    assert_eq!(doc.metadata.author, "未知");
}

#[test]
fn test_explicit_author_overrides_default() {
    let md = "---
title: \"测试文献\"
author: \"张三\"
date: \"1938-05\"
category: \"history\"
---

正文内容。
";
    let doc = MarkdownParser::parse_str(md, Some("test.md")).unwrap();
    assert_eq!(doc.metadata.author, "张三");
}
