use actix_web::HttpRequest;

pub const LANGUAGE_COOKIE: &str = "microbin_lang";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    English,
    SimplifiedChinese,
}

impl Language {
    pub fn code(self) -> &'static str {
        match self {
            Self::English => "en",
            Self::SimplifiedChinese => "zh-CN",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "en" | "en-US" | "en-GB" => Some(Self::English),
            "zh" | "zh-CN" | "zh-Hans" => Some(Self::SimplifiedChinese),
            _ => None,
        }
    }

    fn from_accept_language(value: &str) -> Option<Self> {
        for item in value.split(',') {
            let language = item.split(';').next().unwrap_or("").trim();
            let language = language.to_ascii_lowercase();
            if matches!(language.as_str(), "zh" | "zh-cn" | "zh-hans" | "zh-sg") {
                return Some(Self::SimplifiedChinese);
            }
            if language == "en" || language.starts_with("en-") {
                return Some(Self::English);
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug)]
pub struct I18n {
    pub language: Language,
}

impl I18n {
    pub fn from_request(request: &HttpRequest) -> Self {
        let language = request
            .cookie(LANGUAGE_COOKIE)
            .and_then(|cookie| Language::from_code(cookie.value()))
            .or_else(|| {
                request
                    .headers()
                    .get("Accept-Language")
                    .and_then(|value| value.to_str().ok())
                    .and_then(Language::from_accept_language)
            })
            .unwrap_or(Language::English);

        Self { language }
    }

    pub fn text(self, key: &str) -> &'static str {
        if self.language == Language::SimplifiedChinese {
            match key {
                "new" => "新建",
                "list" => "列表",
                "info" => "信息",
                "navigation" => "导航",
                "language" => "语言",
                "english" => "English",
                "simplified_chinese" => "简体中文",
                "theme_light" => "切换到浅色主题",
                "theme_dark" => "切换到深色主题",
                "expiration" => "过期时间",
                "expire_after" => "过期于",
                "one_minute" => "1 分钟",
                "ten_minutes" => "10 分钟",
                "one_hour" => "1 小时",
                "twenty_four_hours" => "24 小时",
                "three_days" => "3 天",
                "one_week" => "1 周",
                "never_expire" => "永不过期",
                "burn_after" => "阅读后销毁",
                "burn_after_group" => "销毁于第",
                "first_read" => "首次阅读",
                "tenth_read" => "第 10 次阅读",
                "hundredth_read" => "第 100 次阅读",
                "thousandth_read" => "第 1000 次阅读",
                "ten_thousandth_read" => "第 10000 次阅读",
                "no_limit" => "不限次数",
                "syntax" => "语法",
                "none" => "无",
                "source_code" => "源代码",
                "descriptors" => "描述文件",
                "bash_shell" => "Bash Shell",
                "delphi" => "Delphi",
                "javascript" => "JavaScript",
                "editable" => "可编辑",
                "private" => "私有",
                "other" => "其他",
                "content" => "内容",
                "custom_key" => "自定义 Key（可选）",
                "custom_key_hint" => "仅允许 a-z、0-9、短横线和下划线，长度 3-64",
                "custom_key_empty" => "留空将自动生成 Key",
                "custom_key_valid" => "Key 格式有效",
                "custom_key_invalid" => "Key 格式无效",
                "characters" => "个字符",
                "select_or_drop_file" => "选择或拖放文件附件",
                "drop_file" => "将文件拖放到这里",
                "drop_replace_file" => "将文件拖放到这里以替换",
                "attached" => "已附加：",
                "save" => "保存",
                "saving" => "保存中…",
                "clear_file" => "移除文件",
                "no_file" => "未选择文件",
                "read_only" => "只读",
                "copy_text" => "复制文本",
                "copy_redirect" => "复制重定向地址",
                "raw_text_content" => "原始文本内容",
                "qr" => "二维码",
                "edit" => "编辑",
                "remove" => "删除",
                "copy_url" => "复制 URL",
                "copied" => "已复制",
                "download_attached_file" => "下载附件：",
                "back_to_pasta" => "返回 Pasta",
                "pastas" => "Pastas",
                "no_pastas" => "还没有 Pasta。",
                "create_one" => "创建一个",
                "here" => "这里",
                "key" => "Key",
                "raw" => "原始内容",
                "file" => "文件",
                "never" => "永不",
                "url_redirects" => "URL 重定向",
                "created" => "创建时间",
                "open" => "打开",
                "copy" => "复制",
                "welcome" => "欢迎使用 MicroBin",
                "links" => "链接",
                "documentation_help" => "文档与帮助",
                "feedback" => "反馈",
                "donate_sponsor" => "捐赠与赞助",
                "version" => "版本",
                "status" => "状态",
                "pasta_count" => "Pasta 数量",
                "actions" => "操作",
                "messages" => "消息",
                "not_found" => "未找到",
                "go_home" => "返回首页",
                "just_now" => "刚刚",
                "warning_no_public_url" => {
                    "警告：未通过 --public-path 参数设置公共 URL，二维码和复制 URL 功能已禁用"
                },
                "pasta_not_found" => "未找到 Pasta！:-(",
                "invalid_login" => "登录信息无效。",
                "invalid_locale" => "不支持的语言。",
                "invalid_key" => "Key 必须为 3-64 个小写字母、数字、短横线或下划线。",
                "duplicate_key" => "该 Key 已存在，请选择其他 Key。",
                "confirm_remove" => "确定要删除此 Pasta 吗？",
                _ => Self { language: Language::English }.text(key),
            }
        } else {
            match key {
                "new" => "New",
                "list" => "List",
                "info" => "Info",
                "navigation" => "Navigation",
                "language" => "Language",
                "english" => "English",
                "simplified_chinese" => "简体中文",
                "theme_light" => "Switch to light theme",
                "theme_dark" => "Switch to dark theme",
                "expiration" => "Expiration",
                "expire_after" => "Expire after",
                "one_minute" => "1 minute",
                "ten_minutes" => "10 minutes",
                "one_hour" => "1 hour",
                "twenty_four_hours" => "24 hours",
                "three_days" => "3 days",
                "one_week" => "1 week",
                "never_expire" => "Never Expire",
                "burn_after" => "Burn After",
                "burn_after_group" => "Burn after",
                "first_read" => "First Read",
                "tenth_read" => "10th Read",
                "hundredth_read" => "100th Read",
                "thousandth_read" => "1000th Read",
                "ten_thousandth_read" => "10000th Read",
                "no_limit" => "No Limit",
                "syntax" => "Syntax",
                "none" => "None",
                "source_code" => "Source Code",
                "descriptors" => "Descriptors",
                "bash_shell" => "Bash Shell",
                "delphi" => "Delphi",
                "javascript" => "JavaScript",
                "editable" => "Editable",
                "private" => "Private",
                "other" => "Other",
                "content" => "Content",
                "custom_key" => "Custom Key (optional)",
                "custom_key_hint" => "Use a-z, 0-9, hyphens, and underscores; 3-64 characters",
                "custom_key_empty" => "Leave blank to generate a Key automatically",
                "custom_key_valid" => "Key format is valid",
                "custom_key_invalid" => "Key format is invalid",
                "characters" => "characters",
                "select_or_drop_file" => "Select or drop file attachment",
                "drop_file" => "Drop your file here",
                "drop_replace_file" => "Drop your file here to replace",
                "attached" => "Attached: ",
                "save" => "Save",
                "saving" => "Saving…",
                "clear_file" => "Remove file",
                "no_file" => "No file selected",
                "read_only" => "Read Only",
                "copy_text" => "Copy Text",
                "copy_redirect" => "Copy Redirect",
                "raw_text_content" => "Raw Text Content",
                "qr" => "QR",
                "edit" => "Edit",
                "remove" => "Remove",
                "copy_url" => "Copy URL",
                "copied" => "Copied",
                "download_attached_file" => "Download attached file: ",
                "back_to_pasta" => "Back to Pasta",
                "pastas" => "Pastas",
                "no_pastas" => "No pastas yet.",
                "create_one" => "Create one",
                "here" => "here",
                "key" => "Key",
                "raw" => "Raw",
                "file" => "File",
                "never" => "Never",
                "url_redirects" => "URL Redirects",
                "created" => "Created",
                "open" => "Open",
                "copy" => "Copy",
                "welcome" => "Welcome to MicroBin",
                "links" => "Links",
                "documentation_help" => "Documentation and Help",
                "feedback" => "Feedback",
                "donate_sponsor" => "Donate and Sponsor",
                "version" => "Version",
                "status" => "Status",
                "pasta_count" => "Pastas",
                "actions" => "Actions",
                "messages" => "Messages",
                "not_found" => "Not Found",
                "go_home" => "Go Home",
                "just_now" => "just now",
                "warning_no_public_url" => {
                    "Warning: No public URL set with --public-path parameter. QR code and URL Copying functions have been disabled"
                },
                "pasta_not_found" => "Pasta not found! :-(",
                "invalid_login" => "Invalid login details.",
                "invalid_locale" => "Unsupported language.",
                "invalid_key" => "Key must be 3-64 lowercase letters, numbers, hyphens, or underscores.",
                "duplicate_key" => "That key already exists. Please choose another key.",
                "confirm_remove" => "Are you sure you want to remove this pasta?",
                _ => "",
            }
        }
    }

    pub fn relative_time(self, elapsed_seconds: i64) -> String {
        let elapsed_seconds = elapsed_seconds.max(0);
        if elapsed_seconds >= 2 * 86400 {
            return self.plural(elapsed_seconds / 86400, "day", "天");
        }
        if elapsed_seconds >= 2 * 3600 {
            return self.plural(elapsed_seconds / 3600, "hour", "小时");
        }
        if elapsed_seconds >= 2 * 60 {
            return self.plural(elapsed_seconds / 60, "minute", "分钟");
        }
        if elapsed_seconds >= 2 {
            return self.plural(elapsed_seconds, "second", "秒");
        }
        self.text("just_now").to_string()
    }

    fn plural(self, value: i64, english_unit: &str, chinese_unit: &str) -> String {
        if self.language == Language::SimplifiedChinese {
            format!("{}{}前", value, chinese_unit)
        } else {
            format!("{} {}{} ago", value, english_unit, if value == 1 { "" } else { "s" })
        }
    }

    pub fn read_summary(self, count: &u64, elapsed_seconds: &i64) -> String {
        if self.language == Language::SimplifiedChinese {
            format!(
                "已阅读 {} 次，最后阅读于{}",
                count,
                self.relative_time(*elapsed_seconds)
            )
        } else {
            format!(
                "Read {} time{}, last {}",
                count,
                if *count == 1 { "" } else { "s" },
                self.relative_time(*elapsed_seconds)
            )
        }
    }

    pub fn status(self, status: &str) -> &'static str {
        if self.language == Language::SimplifiedChinese {
            match status {
                "WARNING" => "警告",
                "OK" => "正常",
                _ => "UNKNOWN",
            }
        } else {
            match status {
                "WARNING" => "WARNING",
                "OK" => "OK",
                _ => "UNKNOWN",
            }
        }
    }

    pub fn switch_url(self, language: &str, current_path: &str) -> String {
        format!(
            "/language/{}?next={}",
            language,
            percent_encode_path(current_path)
        )
    }

    pub fn html_lang(self) -> &'static str {
        match self.language {
            Language::English => "en",
            Language::SimplifiedChinese => "zh-CN",
        }
    }

    pub fn is_english(self) -> bool {
        self.language == Language::English
    }

    pub fn is_simplified_chinese(self) -> bool {
        self.language == Language::SimplifiedChinese
    }

}

pub fn percent_encode_path(path: &str) -> String {
    let mut encoded = String::with_capacity(path.len());
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~' | b'/') {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push_str(&format!("{:02X}", byte));
        }
    }
    encoded
}

pub fn current_path(request: &HttpRequest) -> String {
    request
        .uri()
        .path_and_query()
        .map(|path| path.as_str().to_owned())
        .unwrap_or_else(|| "/".to_owned())
}

#[cfg(test)]
mod tests {
    use super::{percent_encode_path, I18n, Language};
    use actix_web::test as actix_test;

    #[test]
    fn parses_supported_languages() {
        assert_eq!(Language::from_code("zh-CN"), Some(Language::SimplifiedChinese));
        assert_eq!(Language::from_code("en"), Some(Language::English));
        assert_eq!(Language::from_code("fr"), None);
    }

    #[test]
    fn prefers_cookie_over_browser_language() {
        let request = actix_test::TestRequest::default()
            .insert_header(("Accept-Language", "zh-CN, en;q=0.8"))
            .cookie(actix_web::cookie::Cookie::new("microbin_lang", "en"))
            .to_http_request();
        assert_eq!(I18n::from_request(&request).language, Language::English);
    }

    #[test]
    fn detects_simplified_chinese_from_browser_language() {
        let request = actix_test::TestRequest::default()
            .insert_header(("Accept-Language", "zh-CN,zh;q=0.9,en;q=0.8"))
            .to_http_request();
        assert_eq!(I18n::from_request(&request).language, Language::SimplifiedChinese);
    }

    #[test]
    fn encodes_query_delimiters_in_current_path() {
        assert_eq!(percent_encode_path("/pasta/key?view=raw"), "/pasta/key%3Fview%3Draw");
    }

    #[test]
    fn localizes_relative_time() {
        assert_eq!(I18n { language: Language::English }.relative_time(1), "just now");
        assert_eq!(I18n { language: Language::SimplifiedChinese }.relative_time(120), "2分钟前");
    }
}
