//! Prompt kind, tool profile, and role definitions

/// 预定义的 Prompt 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromptKind {
    // System prompt
    /// Agent 角色定义和行为准则
    System,

    // 初始化相关
    /// 项目初始化，创建 .gba/ .trees/ 目录
    Init,

    // 规划相关
    /// 开始规划对话
    PlanStart,
    /// 细化规划
    PlanRefine,
    /// 生成 spec 文件
    PlanGenerate,

    // 执行相关
    /// 执行单个阶段
    ExecutePhase,
    /// 断点续执行阶段
    ExecutePhaseResume,
    /// 修复 precommit hook 错误
    FixHookError,

    // 审查相关
    /// 代码审查
    Review,
    /// 修复审查问题
    FixReviewIssue,

    // 验证相关
    /// 执行验证计划
    Verification,
    /// 修复验证错误
    FixVerificationError,

    // PR 相关
    /// 创建 Pull Request (使用 gh cli)
    CreatePr,
}

/// 工具权限配置
///
/// 不同场景需要不同的工具权限，这是安全边界：
/// - ReadOnly: 只读场景（规划探索、代码审查）
/// - WriteSpecs: 只能写入 .gba/ 目录（生成 spec 文件）
/// - GitOnly: 仅 git/gh 命令（创建 PR）
/// - Full: 完整 Claude Code preset（开发执行）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolProfile {
    /// 只读工具: Read, Glob, Grep
    /// 用于: PlanStart, PlanRefine, Review
    ReadOnly,

    /// 只读 + 写入 .gba/* 目录
    /// 用于: PlanGenerate
    WriteSpecs,

    /// 仅 git/gh 命令
    /// 用于: CreatePr
    GitOnly,

    /// 完整 Claude Code preset
    /// 用于: Init, ExecutePhase, ExecutePhaseResume, FixHookError,
    ///       FixReviewIssue, Verification, FixVerificationError
    Full,
}

/// Prompt 角色类型
///
/// 定义 prompt 是作为 system prompt 还是 user prompt 提供给 Claude Agent SDK
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptRole {
    /// System prompt - 定义 agent 角色和行为准则
    /// 在整个会话期间保持不变
    System,

    /// User prompt - 包含具体任务指令和动态上下文
    User,
}

impl PromptKind {
    /// 获取该 Prompt 类型对应的工具权限配置
    pub fn tool_profile(&self) -> ToolProfile {
        match self {
            // System prompt 不需要工具
            PromptKind::System => ToolProfile::ReadOnly,

            // 只读场景 - 探索代码库，不做修改
            PromptKind::PlanStart => ToolProfile::ReadOnly,
            PromptKind::PlanRefine => ToolProfile::ReadOnly,
            PromptKind::Review => ToolProfile::ReadOnly,

            // 只写 spec 文件
            PromptKind::PlanGenerate => ToolProfile::WriteSpecs,

            // 只需 git/gh 命令
            PromptKind::CreatePr => ToolProfile::GitOnly,

            // 需要完整工具集
            PromptKind::Init
            | PromptKind::ExecutePhase
            | PromptKind::ExecutePhaseResume
            | PromptKind::FixHookError
            | PromptKind::FixReviewIssue
            | PromptKind::Verification
            | PromptKind::FixVerificationError => ToolProfile::Full,
        }
    }

    /// 获取该 Prompt 类型的角色
    pub fn role(&self) -> PromptRole {
        match self {
            PromptKind::System => PromptRole::System,
            _ => PromptRole::User,
        }
    }

    /// 获取模板文件名
    pub fn template_name(&self) -> &'static str {
        match self {
            PromptKind::System => "system",
            PromptKind::Init => "init",
            PromptKind::PlanStart => "plan_start",
            PromptKind::PlanRefine => "plan_refine",
            PromptKind::PlanGenerate => "plan_generate",
            PromptKind::ExecutePhase => "execute_phase",
            PromptKind::ExecutePhaseResume => "execute_phase_resume",
            PromptKind::FixHookError => "fix_hook_error",
            PromptKind::Review => "review",
            PromptKind::FixReviewIssue => "fix_review_issue",
            PromptKind::Verification => "verification",
            PromptKind::FixVerificationError => "fix_verification_error",
            PromptKind::CreatePr => "create_pr",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_profile() {
        assert_eq!(PromptKind::PlanStart.tool_profile(), ToolProfile::ReadOnly);
        assert_eq!(PromptKind::Review.tool_profile(), ToolProfile::ReadOnly);
        assert_eq!(
            PromptKind::PlanGenerate.tool_profile(),
            ToolProfile::WriteSpecs
        );
        assert_eq!(PromptKind::CreatePr.tool_profile(), ToolProfile::GitOnly);
        assert_eq!(PromptKind::ExecutePhase.tool_profile(), ToolProfile::Full);
        assert_eq!(PromptKind::Init.tool_profile(), ToolProfile::Full);
    }

    #[test]
    fn test_role() {
        assert_eq!(PromptKind::System.role(), PromptRole::System);
        assert_eq!(PromptKind::Init.role(), PromptRole::User);
        assert_eq!(PromptKind::ExecutePhase.role(), PromptRole::User);
    }

    #[test]
    fn test_template_name() {
        assert_eq!(PromptKind::System.template_name(), "system");
        assert_eq!(PromptKind::ExecutePhase.template_name(), "execute_phase");
        assert_eq!(
            PromptKind::FixVerificationError.template_name(),
            "fix_verification_error"
        );
    }
}
