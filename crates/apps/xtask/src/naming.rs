//! 旧资源名 → 新资源路径的命名规则。
//!
//! 规则（`docs/4-migration/4.2-assets.md`）：
//!
//! ```text
//! constant  COMMISSION_START          (module = commission)
//!   │ 小写（常量本来就是 UPPER_SNAKE，小写即 snake_case）
//!   ▼
//! base      commission_start
//!   ▼
//! snake     commission_start
//!   │ 若不以 module 开头，则加 module_ 前缀
//!   ▼
//! node      commission_start
//!
//! 落盘（**每个服务器一个资源包**，路径里不再有 server 段）：
//!   PNG  resource/<server>/image/<module>/<node>.png
//!   GIF  resource/<server>/image/<module>/<node>/frame_00.png …   （template 支持目录）
//! ```
//!
//! 这条规则的目的是**可逆**：从图片路径能推出节点名，从节点名能推回图片路径，
//! `pipeline-check` 才能校验「节点引用的图片是否存在」。

/// 合法的服务器目录名。
pub const SERVERS: [&str; 4] = ["cn", "en", "jp", "tw"];

/// 是否为服务器目录名。
pub fn is_server(s: &str) -> bool {
    SERVERS.contains(&s)
}

/// 由旧常量名推导节点名。
///
/// **不剥掉 `TEMPLATE_` 前缀**：同一模块里 `TEMPLATE_RUNNING` 与 `RESEARCH_RUNNING`
/// 是两张不同的图，剥前缀会合并成同一个节点（实测 9 处冲突）。保留前缀让命名是
/// `(module, constant)` 的纯函数——这是可逆性的前提。
pub fn node_name(module: &str, constant: &str) -> String {
    let base = sanitize(&constant.to_ascii_lowercase());

    if base == module || base.starts_with(&format!("{module}_")) {
        base
    } else {
        format!("{module}_{base}")
    }
}

/// 把非 `[a-z0-9_]` 字符压成 `_`，并合并连续下划线。
fn sanitize(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut last_underscore = false;
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_underscore = false;
        } else if !last_underscore {
            out.push('_');
            last_underscore = true;
        }
    }
    out.trim_matches('_').to_string()
}

/// 从 `./assets/<server>/<rest>` 拆出 (server, rest)。
///
/// 非服务器目录（如 `./assets/island/...`）返回 `None`——这类路径需要人工确认归属，
/// 转换器不猜。
pub fn split_server_path(path: &str) -> Option<(String, String)> {
    let p = path.trim_start_matches("./");
    let mut it = p.splitn(3, '/');
    if it.next()? != "assets" {
        return None;
    }
    let server = it.next()?;
    let rest = it.next()?;
    if !is_server(server) {
        return None;
    }
    Some((server.to_string(), rest.to_string()))
}

/// 静态图片在**包内**的相对路径（相对 `resource/<server>/`）。
///
/// 也是 Pipeline 节点的 `template` 值——MaaFW 的 `template` 相对资源包的 `image/` 解析。
pub fn image_path(module: &str, node: &str) -> String {
    format!("image/{module}/{node}.png")
}

/// 动图目录在包内的相对路径。`template` 支持目录，会递归加载其中所有图片。
pub fn gif_dir(module: &str, node: &str) -> String {
    format!("image/{module}/{node}")
}

/// 资源包根（相对 `resource/`）。
pub fn bundle_path(server: &str) -> String {
    server.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_name_prefixed_when_missing_module() {
        assert_eq!(
            node_name("commission", "COMMISSION_START"),
            "commission_start"
        );
        assert_eq!(node_name("commission", "ADVICE"), "commission_advice");
    }

    #[test]
    fn template_prefix_is_kept_to_avoid_collisions() {
        // 同一模块里 TEMPLATE_RUNNING 与 RESEARCH_RUNNING 是两张图，不能合并
        // module 恰好叫 template 时，base 已带前缀，不会重复
        assert_eq!(
            node_name("template", "TEMPLATE_DORM_COIN"),
            "template_dorm_coin"
        );
        assert_eq!(
            node_name("azur_stats", "TEMPLATE_MEOWFFICER_NAME"),
            "azur_stats_template_meowfficer_name"
        );
        assert_ne!(
            node_name("research", "TEMPLATE_RUNNING"),
            node_name("research", "RESEARCH_RUNNING")
        );
    }

    #[test]
    fn node_name_keeps_digits() {
        assert_eq!(node_name("commission", "REWARD_1"), "commission_reward_1");
    }

    #[test]
    fn server_path_split() {
        assert_eq!(
            split_server_path("./assets/cn/commission/COMMISSION_START.png"),
            Some((
                "cn".to_string(),
                "commission/COMMISSION_START.png".to_string()
            ))
        );
        // 非服务器目录不猜
        assert_eq!(split_server_path("./assets/island/foo.png"), None);
        assert_eq!(split_server_path("./assets/mask/m.png"), None);
    }

    #[test]
    fn dest_paths_are_bundle_relative() {
        // 每个服务器一个包，包内路径不含 server 段
        assert_eq!(
            image_path("commission", "commission_start"),
            "image/commission/commission_start.png"
        );
        assert_eq!(
            gif_dir("template", "template_dorm_coin"),
            "image/template/template_dorm_coin"
        );
        assert_eq!(bundle_path("jp"), "jp");
    }
}
