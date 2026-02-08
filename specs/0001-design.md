# GBA (Geektime Bootcamp Agent) Design Document

## Overview

GBA 是一个基于 Claude Agent SDK 的命令行工具，帮助开发者围绕代码仓库规划和实现新功能。它通过 AI 驱动的交互式规划和自动化执行，简化功能开发流程。

## Core Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                              gba-cli                                    │
│                         (clap / ratatui)                                │
├─────────────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐                      │
│  │  gba init   │  │  gba plan   │  │  gba run    │                      │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘                      │
│         │                │                │                             │
│         └────────────────┼────────────────┘                             │
│                          ▼                                              │
├─────────────────────────────────────────────────────────────────────────┤
│                           gba-core                                      │
│                    (Core Execution Engine)                              │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │                         Runtime                                 │    │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐        │    │
│  │  │  InitExecutor │  │ PlanExecutor  │  │  RunExecutor  │        │    │
│  │  └───────┬───────┘  └───────┬───────┘  └───────┬───────┘        │    │
│  │          └──────────────────┼──────────────────┘                │    │
│  │                             ▼                                   │    │
│  │  ┌─────────────────────────────────────────────────────────┐    │    │
│  │  │                    AgentRunner                          │    │    │
│  │  │         (claude-agent-sdk-rs / tokio)                   │    │    │
│  │  └─────────────────────────────────────────────────────────┘    │    │
│  └─────────────────────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────────────────────┤
│                            gba-pm                                       │
│                      (Prompt Manager)                                   │
│  ┌─────────────────────────────────────────────────────────────────┐    │
│  │  ┌───────────────┐  ┌───────────────┐  ┌───────────────┐        │    │
│  │  │ TemplateStore │  │ PromptBuilder │  │ ContextLoader │        │    │
│  │  │  (minijinja)  │  │               │  │               │        │    │
│  │  └───────────────┘  └───────────────┘  └───────────────┘        │    │
│  └─────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘
```

## Directory Structure

```
project_root/
├── .gba/                              # GBA 工作目录
│   ├── config.yml                     # 项目配置 (见 Configuration 章节)
│   └── features/                      # 功能规划目录
│       └── 0001_feature-slug/         # 单个功能
│           ├── development_plan.md    # 开发计划 (阶段划分)
│           ├── design.md              # 设计规格
│           │                          #   - 高层设计
│           │                          #   - 接口设计
│           │                          #   - 数据流图
│           │                          #   - 核心数据结构
│           ├── verification_plan.md   # 验证计划
│           └── state.yml              # 执行状态 (见 State Management 章节)
├── .trees/                            # 代码树缓存 (gitignored)
│   └── 0001_feature-slug.tree         # 功能相关的代码树快照
├── .gitignore                         # 包含 .trees/
└── CLAUDE.md                          # Claude 上下文文档
```

---

## Core Workflow

### End-to-End Flow

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                                GBA Workflow                                     │
└─────────────────────────────────────────────────────────────────────────────────┘

  ┌─────────────────────────────────────────────────────────────────────────────┐
  │                         PLANNING PHASE (gba plan)                           │
  │                                                                             │
  │   ┌──────────────┐      feedback       ┌──────────────────────────────────┐ │
  │   │   基础的      │◄──────loop─────────│                                   │ │
  │   │  功能需求     │                     │         Coding Agent             │ │
  │   │              │─────────────────────▶│    (Claude Agent SDK)           │ │
  │   └──────────────┘                     │                                  │ │
  │                                        └───────────┬──────────────────────┘ │
  │                                                    │                        │
  │                          ┌─────────────────────────┼─────────────────────┐  │
  │                          │                         │                     │  │
  │                          ▼                         ▼                     ▼  │
  │                   ┌─────────────┐          ┌─────────────┐       ┌──────────┐
  │                   │  开发计划    │          │ Design Spec │       │ 验证计划 │ │
  │                   │             │          │             │       │          │ │
  │                   │ - 阶段划分   │          │ - 高层设计   │       │ - 测试用例│ │
  │                   │ - 任务分解   │          │ - 接口设计   │       │ - 验收标准│ │
  │                   │ - 依赖关系   │          │ - 数据流图   │       │ - 质量检查│ │
  │                   │             │          │ - 核心数据   │       │          │ │
  │                   │             │          │   结构      │       │          │ │
  │                   └─────────────┘          └─────────────┘       └──────────┘ │
  │                                                                              │
  └──────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
  ┌──────────────────────────────────────────────────────────────────────────────┐
  │                        EXECUTION PHASE (gba run)                              │
  │                                                                               │
  │   ┌─────────────────────────────────────────────────────────────────────┐    │
  │   │                     阶段 by 阶段开发                                  │    │
  │   │                                                                     │    │
  │   │  ┌─────────────────────────────────────────────────────────────┐   │    │
  │   │  │                    Coding Agent                              │   │    │
  │   │  │              (执行当前阶段的开发任务)                           │   │    │
  │   │  └─────────────────────────┬───────────────────────────────────┘   │    │
  │   │                            │                                       │    │
  │   │                            ▼                                       │    │
  │   │  ┌─────────────────────────────────────────────────────────────┐   │    │
  │   │  │                   Precommit Hooks                            │   │    │
  │   │  │         build │ fmt │ lint │ security check │ ...           │   │    │
  │   │  └─────────────────────────┬───────────────────────────────────┘   │    │
  │   │                            │                                       │    │
  │   │              ┌─────────────┴─────────────┐                         │    │
  │   │              │ failed                    │ passed                  │    │
  │   │              ▼                           ▼                         │    │
  │   │    ┌─────────────────┐         ┌─────────────────┐                │    │
  │   │    │   处理问题       │         │   Agent Review  │                │    │
  │   │    │ (回到 Coding    │         │                 │                │    │
  │   │    │  Agent 修复)    │         └────────┬────────┘                │    │
  │   │    └────────┬────────┘                  │                         │    │
  │   │             │                           ▼                         │    │
  │   │             │                  ┌─────────────────┐                │    │
  │   │             │                  │  Valid Issues?  │                │    │
  │   │             │                  └────────┬────────┘                │    │
  │   │             │              ┌────────────┴────────────┐            │    │
  │   │             │              │ yes                     │ no         │    │
  │   │             │              ▼                         ▼            │    │
  │   │             │    ┌─────────────────┐       ┌─────────────────┐   │    │
  │   │             │    │   处理问题       │       │  Commit Phase   │   │    │
  │   │             │    │ (回到 Coding    │       │                 │   │    │
  │   │             │    │  Agent 修复)    │       └────────┬────────┘   │    │
  │   │             │    └────────┬────────┘                │            │    │
  │   │             │             │                         │            │    │
  │   │             └─────────────┴─────────────────────────┘            │    │
  │   │                                    │                             │    │
  │   │                                    ▼                             │    │
  │   │                          [Next Phase or Done]                    │    │
  │   │                                                                  │    │
  │   └──────────────────────────────────────────────────────────────────┘    │
  │                                    │                                      │
  │                                    ▼ (All phases done)                    │
  │   ┌──────────────────────────────────────────────────────────────────┐   │
  │   │                          验证                                     │   │
  │   │              (根据 verification_plan.md 执行验证)                   │   │
  │   └─────────────────────────────┬────────────────────────────────────┘   │
  │                                 │                                        │
  │                   ┌─────────────┴─────────────┐                          │
  │                   │ failed                    │ passed                   │
  │                   ▼                           ▼                          │
  │         ┌─────────────────┐          ┌─────────────────┐                 │
  │         │   修复验证问题   │          │   Submit PR     │                 │
  │         │ (回到 Coding    │          │                 │                 │
  │         │  Agent 修复)    │          └─────────────────┘                 │
  │         └────────┬────────┘                                              │
  │                  │                                                       │
  │                  └───────────────────(loop back)─────────────────────────│
  │                                                                          │
  └──────────────────────────────────────────────────────────────────────────┘
```

---

## Command Flows

### 1. `gba init` - 项目初始化

```
┌─────────────────────────────────────────────────────────────────────┐
│                             gba init                                │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
                    ┌───────────────────────┐
                    │ Check if .gba exists  │
                    └───────────┬───────────┘
                                │
              ┌─────────────────┼─────────────────┐
              │ Yes             │                 │ No
              ▼                 │                 ▼
    ┌─────────────────┐         │       ┌─────────────────┐
    │ Exit with msg   │         │       │ Create .gba/    │
    │ "Already init"  │         │       │ Create .trees/  │
    └─────────────────┘         │       └────────┬────────┘
                                │                │
                                │                ▼
                                │       ┌─────────────────────┐
                                │       │ Analyze repo via    │
                                │       │ Claude Agent SDK    │
                                │       └────────┬────────────┘
                                │                │
                                │                ▼
                                │       ┌─────────────────────┐
                                │       │ Generate .gba.md    │
                                │       │ for key directories │
                                │       └────────┬────────────┘
                                │                │
                                │                ▼
                                │       ┌─────────────────────┐
                                │       │ Update CLAUDE.md    │
                                │       │ with references     │
                                │       └─────────────────────┘
```

### 2. `gba plan <feature-slug>` - 功能规划

```
┌─────────────────────────────────────────────────────────────────────┐
│                      gba plan <feature-slug>                        │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
                    ┌───────────────────────┐
                    │ Enter TUI (ratatui)   │
                    └───────────┬───────────┘
                                │
                                ▼
         ┌──────────────────────────────────────────────────────────┐
         │                 Feedback Loop                             │
         │                                                          │
         │  ┌─────────────────────────────────────────────────────┐ │
         │  │ AI: What feature do you want to build?              │ │
         │  └─────────────────────────────────────────────────────┘ │
         │                          │                               │
         │                          ▼                               │
         │  ┌─────────────────────────────────────────────────────┐ │
         │  │ User: Describes feature requirements                │ │
         │  └─────────────────────────────────────────────────────┘ │
         │                          │                               │
         │                          ▼                               │
         │  ┌─────────────────────────────────────────────────────┐ │
         │  │ AI: Analyzes codebase, proposes:                    │ │
         │  │     - 高层设计 (architecture)                        │ │
         │  │     - 接口设计 (interfaces)                          │ │
         │  │     - 数据流图 (data flow)                           │ │
         │  │     - 核心数据结构 (data structures)                  │ │
         │  │     - 开发阶段划分 (phases)                           │ │
         │  │     - 验证计划 (verification)                        │ │
         │  └─────────────────────────────────────────────────────┘ │
         │                          │                               │
         │                          ▼                               │
         │  ┌─────────────────────────────────────────────────────┐ │
         │  │ User: Refines / Requests changes                    │ │◀──┐
         │  └─────────────────────────────────────────────────────┘ │   │
         │                          │                               │   │
         │              ┌───────────┴───────────┐                   │   │
         │              │                       │                   │   │
         │              ▼ (changes)             ▼ (approve)         │   │
         │    ┌─────────────────┐     ┌─────────────────────────┐  │   │
         │    │ AI: Updates     │     │ Generate spec files:    │  │   │
         │    │ proposal        │────▶│ - development_plan.md   │  │   │
         │    └─────────────────┘     │ - design.md             │  │   │
         │              │             │ - verification_plan.md  │  │   │
         │              └─────────────┴─────────────────────────────┘   │
         │                                      │                       │
         └──────────────────────────────────────┼───────────────────────┘
                                                │
                                                ▼
                                  ┌───────────────────────┐
                                  │ "Run `gba run` to     │
                                  │  execute the plan"    │
                                  └───────────────────────┘
```

### 3. `gba run <feature-slug>` - 功能执行

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                          gba run <feature-slug>                              │
└──────────────────────────────────────────────────────────────────────────────┘
                                        │
                                        ▼
                            ┌───────────────────────┐
                            │ Load specs from .gba/ │
                            │ - development_plan.md │
                            │ - design.md           │
                            │ - verification_plan.md│
                            └───────────┬───────────┘
                                        │
                                        ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                        Phase-by-Phase Development                            │
│                                                                              │
│   ┌────────────────────────────────────────────────────────────────────┐    │
│   │  FOR EACH phase IN development_plan:                                │    │
│   │                                                                     │    │
│   │    ┌─────────────────────────────────────────────────────────────┐ │    │
│   │    │ [ ] Phase N: {phase.name}                                    │ │    │
│   │    │                                                              │ │    │
│   │    │  ┌─────────────────────────────────────────────────────────┐│ │    │
│   │    │  │               Coding Agent                               ││ │    │
│   │    │  │        (执行当前阶段的开发任务)                            ││ │    │
│   │    │  └──────────────────────┬──────────────────────────────────┘│ │    │
│   │    │                         │                                   │ │    │
│   │    │                         ▼                                   │ │    │
│   │    │  ┌─────────────────────────────────────────────────────────┐│ │    │
│   │    │  │              Precommit Hooks                             ││ │    │
│   │    │  │   ┌───────┬───────┬────────┬─────────────────┐          ││ │    │
│   │    │  │   │ build │  fmt  │  lint  │ security check  │ ...      ││ │    │
│   │    │  │   └───────┴───────┴────────┴─────────────────┘          ││ │    │
│   │    │  └──────────────────────┬──────────────────────────────────┘│ │    │
│   │    │                         │                                   │ │    │
│   │    │           ┌─────────────┴─────────────┐                     │ │    │
│   │    │           │ hook failed               │ hook passed         │ │    │
│   │    │           ▼                           ▼                     │ │    │
│   │    │  ┌─────────────────┐        ┌─────────────────┐            │ │    │
│   │    │  │ Coding Agent    │        │  Agent Review   │            │ │    │
│   │    │  │ fixes issues    │        │                 │            │ │    │
│   │    │  └────────┬────────┘        └────────┬────────┘            │ │    │
│   │    │           │                          │                      │ │    │
│   │    │           └──────────┐               ▼                      │ │    │
│   │    │                      │      ┌─────────────────┐            │ │    │
│   │    │                      │      │ Valid Issues?   │            │ │    │
│   │    │                      │      └────────┬────────┘            │ │    │
│   │    │                      │    ┌──────────┴──────────┐          │ │    │
│   │    │                      │    │ yes                 │ no       │ │    │
│   │    │                      │    ▼                     ▼          │ │    │
│   │    │                      │  ┌──────────┐   ┌─────────────────┐ │ │    │
│   │    │                      │  │ Coding   │   │ [ ] Commit      │ │ │    │
│   │    │                      │  │ Agent    │   │     Phase N     │ │ │    │
│   │    │                      │  │ fixes    │   └────────┬────────┘ │ │    │
│   │    │                      │  └────┬─────┘            │          │ │    │
│   │    │                      │       │                  │          │ │    │
│   │    │                      └───────┴──────────────────┘          │ │    │
│   │    │                                     │                       │ │    │
│   │    └─────────────────────────────────────┼───────────────────────┘ │    │
│   │                                          │                         │    │
│   │                                          ▼                         │    │
│   │                              [Next Phase or Done]                  │    │
│   │                                                                    │    │
│   └────────────────────────────────────────────────────────────────────┘    │
│                                          │                                  │
│                                          ▼ (All phases completed)           │
│  ┌────────────────────────────────────────────────────────────────────────┐ │
│  │                              Verification                               │ │
│  │                  (Execute verification_plan.md)                        │ │
│  │                                                                        │ │
│  │   [ ] Run test suites                                                  │ │
│  │   [ ] Check acceptance criteria                                        │ │
│  │   [ ] Validate integration                                             │ │
│  │                                                                        │ │
│  └──────────────────────────────────┬─────────────────────────────────────┘ │
│                                     │                                       │
│                       ┌─────────────┴─────────────┐                         │
│                       │ failed                    │ passed                  │
│                       ▼                           ▼                         │
│             ┌─────────────────┐          ┌─────────────────┐                │
│             │ Coding Agent    │          │ [ ] Submit PR   │                │
│             │ fixes issues    │──────────│                 │                │
│             └─────────────────┘  (retry) └─────────────────┘                │
│                                                                              │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Spec File Formats

### development_plan.md

```markdown
# Development Plan: {feature-slug}

## Overview
{Brief description of the feature}

## Phases

### Phase 1: {phase-name}
**Goal**: {What this phase achieves}

**Tasks**:
- [ ] Task 1 description
- [ ] Task 2 description

**Dependencies**: None

**Deliverables**:
- {What will be created/modified}

### Phase 2: {phase-name}
...

## Dependencies
- {External dependencies if any}
```

### design.md

```markdown
# Design Spec: {feature-slug}

## High-Level Design
{Architecture overview}

```
{ASCII diagram of component relationships}
```

## Interface Design

### Public APIs
```rust
// New public interfaces to be created
pub trait Foo { ... }
pub struct Bar { ... }
```

### Internal APIs
```rust
// Internal interfaces between components
```

## Data Flow
```
{ASCII diagram showing data flow}
```

## Core Data Structures

```rust
// Key data structures
pub struct FeatureState { ... }
pub enum FeatureEvent { ... }
```

## Design Decisions
| Decision | Rationale | Alternatives Considered |
|----------|-----------|------------------------|
| ... | ... | ... |
```

### verification_plan.md

```markdown
# Verification Plan: {feature-slug}

## Test Cases

### Unit Tests
- [ ] Test case 1: {description}
- [ ] Test case 2: {description}

### Integration Tests
- [ ] Test case 1: {description}

## Acceptance Criteria
- [ ] Criterion 1: {description}
- [ ] Criterion 2: {description}

## Quality Checks
- [ ] Code coverage >= {threshold}%
- [ ] No new lint warnings
- [ ] Security scan passes
- [ ] Performance benchmarks pass

## Manual Verification (if needed)
- [ ] {Manual check description}
```

---

## Crate Specifications

### 1. `gba-pm` (Prompt Manager)

**职责**: 管理、加载和渲染提示词模板

```rust
// ============================================================
// Public Interface
// ============================================================

/// 提示词管理器
pub struct PromptManager { /* ... */ }

impl PromptManager {
    /// 从目录加载模板
    pub fn from_dir(path: impl AsRef<Path>) -> Result<Self>;

    /// 使用内置模板
    pub fn with_builtin() -> Self;

    /// 渲染指定模板
    pub fn render(&self, kind: PromptKind, ctx: &Context) -> Result<String>;

    /// 获取模板列表
    pub fn list_templates(&self) -> Vec<&str>;
}

/// 模板上下文 - 执行引擎自动填充
pub struct Context { /* ... */ }

impl Context {
    pub fn new() -> Self;
    pub fn insert(&mut self, key: &str, value: impl Serialize);
}

/// 执行引擎自动提供的上下文变量
///
/// | 变量 | 来源 | 说明 |
/// |------|------|------|
/// | `feature_slug` | CLI 参数 | 功能标识 |
/// | `project_name` | config.yml / 目录名 | 项目名称 |
/// | `timestamp` | 系统时间 | ISO8601 格式 |
/// | `phase_index` | state.yml | 当前阶段索引 (1-based) |
/// | `total_phases` | development_plan.md | 总阶段数 |
/// | `phase` | development_plan.md | 当前阶段详情 |
/// | `completed_phases` | state.yml | 已完成阶段列表 |
/// | `design` | design.md | 设计规格 |
/// | `verification_plan` | verification_plan.md | 验证计划 |
/// | `verification` | 执行结果 | 验证结果 |
/// | `modified_files` | git diff | 修改的文件列表 |
/// | `diff` | git diff | 变更内容 |
/// | `error_output` | 命令输出 | 错误信息 |
/// | `issues` | review 结果 | 审查问题 |
/// | `failures` | verification 结果 | 验证失败项 |
///
/// Convention over Configuration:
/// - 分支名: `feature/{feature_slug}`
/// - Base 分支: `main`
/// - 最大重试次数: 3
/// - 覆盖率阈值: 80%

/// 预定义的 Prompt 类型
pub enum PromptKind {
    // System prompt
    System,                 // Agent 角色定义和行为准则

    // 初始化相关
    Init,                   // 项目初始化，创建 .gba/ .trees/ 目录

    // 规划相关
    PlanStart,              // 开始规划对话
    PlanRefine,             // 细化规划
    PlanGenerate,           // 生成 spec 文件

    // 执行相关
    ExecutePhase,           // 执行单个阶段
    ExecutePhaseResume,     // 断点续执行阶段
    FixHookError,           // 修复 precommit hook 错误

    // 审查相关
    Review,                 // 代码审查
    FixReviewIssue,         // 修复审查问题

    // 验证相关
    Verification,           // 执行验证计划
    FixVerificationError,   // 修复验证错误

    // PR 相关
    CreatePr,               // 创建 Pull Request (使用 gh cli)
}

/// 工具权限配置
///
/// 不同场景需要不同的工具权限，这是安全边界：
/// - ReadOnly: 只读场景（规划探索、代码审查）
/// - WriteSpecs: 只能写入 .gba/ 目录（生成 spec 文件）
/// - GitOnly: 仅 git/gh 命令（创建 PR）
/// - Full: 完整 Claude Code preset（开发执行）
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

impl PromptKind {
    /// 获取该 Prompt 类型对应的工具权限配置
    pub fn tool_profile(&self) -> ToolProfile {
        match self {
            // 只读场景 - 探索代码库，不做修改
            PromptKind::PlanStart => ToolProfile::ReadOnly,
            PromptKind::PlanRefine => ToolProfile::ReadOnly,
            PromptKind::Review => ToolProfile::ReadOnly,

            // 只写 spec 文件
            PromptKind::PlanGenerate => ToolProfile::WriteSpecs,

            // 只需 git/gh 命令
            PromptKind::CreatePr => ToolProfile::GitOnly,

            // 需要完整工具集
            _ => ToolProfile::Full,
        }
    }
}

/// Prompt 角色类型
///
/// 定义 prompt 是作为 system prompt 还是 user prompt 提供给 Claude Agent SDK
pub enum PromptRole {
    /// System prompt - 定义 agent 角色和行为准则
    /// 在整个会话期间保持不变
    System,

    /// User prompt - 包含具体任务指令和动态上下文
    User,
}

impl PromptKind {
    /// 获取该 Prompt 类型的角色
    pub fn role(&self) -> PromptRole {
        match self {
            PromptKind::System => PromptRole::System,
            _ => PromptRole::User,
        }
    }
}
```

**模块结构**:
```
gba-pm/
├── src/
│   ├── lib.rs              # Public exports
│   ├── manager.rs          # PromptManager implementation
│   ├── context.rs          # Context builder
│   ├── kind.rs             # PromptKind, ToolProfile, PromptRole
│   └── error.rs            # Error types
└── templates/              # Prompt templates (English)
    ├── system.jinja        # System prompt (agent 角色定义)
    ├── init.jinja
    ├── plan_start.jinja
    ├── plan_refine.jinja
    ├── plan_generate.jinja
    ├── execute_phase.jinja
    ├── execute_phase_resume.jinja
    ├── fix_hook_error.jinja
    ├── review.jinja
    ├── fix_review_issue.jinja
    ├── verification.jinja
    ├── fix_verification_error.jinja
    └── create_pr.jinja
```

---

### 2. `gba-core` (Core Execution Engine)

**职责**: 提供核心执行引擎，协调 Agent 调用和任务执行

```rust
// ============================================================
// Public Interface
// ============================================================

/// GBA 运行时配置
#[derive(TypedBuilder)]
pub struct Config {
    /// API Key (from env ANTHROPIC_API_KEY or config file)
    #[builder(default)]
    pub api_key: Option<String>,

    /// API Base URL (from env ANTHROPIC_BASE_URL or config file)
    #[builder(default = "https://api.anthropic.com")]
    pub base_url: String,

    /// Claude model to use
    #[builder(default = "claude-sonnet-4-20250514")]
    pub model: String,

    /// Max tokens for responses
    #[builder(default = 8192)]
    pub max_tokens: u32,

    /// Working directory (project root)
    #[builder(default = ".")]
    pub working_dir: PathBuf,

    /// Precommit hooks configuration
    #[builder(default)]
    pub hooks: HooksConfig,
}

impl Config {
    /// Load configuration from all sources (env, user config, project config)
    /// Priority: CLI args > env vars > user config > project config
    pub fn load(working_dir: impl AsRef<Path>) -> Result<Self>;

    /// Load from environment variables only
    pub fn from_env() -> Result<Self>;
}

/// Precommit hooks 配置
#[derive(TypedBuilder, Default)]
pub struct HooksConfig {
    #[builder(default = true)]
    pub build: bool,

    #[builder(default = true)]
    pub fmt: bool,

    #[builder(default = true)]
    pub lint: bool,

    #[builder(default = true)]
    pub security_check: bool,

    #[builder(default)]
    pub custom: Vec<String>,
}

/// GBA 运行时
pub struct Runtime { /* ... */ }

impl Runtime {
    pub fn new(config: Config) -> Result<Self>;

    /// 执行初始化
    pub async fn init(&self) -> Result<InitResult>;

    /// 执行规划 (返回交互式会话)
    pub fn plan(&self, feature: &str) -> PlanSession;

    /// 执行运行
    pub fn run(&self, feature: &str) -> RunSession;
}

/// 初始化结果
pub struct InitResult {
    pub gba_dir: PathBuf,
    pub trees_dir: PathBuf,
    pub analyzed_dirs: Vec<PathBuf>,
}

/// 规划会话 (支持 feedback loop)
pub struct PlanSession { /* ... */ }

impl PlanSession {
    /// 发送用户消息
    pub async fn send(&mut self, message: &str) -> Result<PlanResponse>;

    /// 确认生成 spec
    pub async fn generate_specs(&mut self) -> Result<SpecResult>;

    /// 取消会话
    pub fn cancel(&mut self);
}

/// 规划响应
pub struct PlanResponse {
    pub message: String,
    pub proposed_design: Option<ProposedDesign>,
}

/// 提议的设计
pub struct ProposedDesign {
    pub high_level: String,
    pub interfaces: String,
    pub data_flow: String,
    pub data_structures: String,
    pub phases: Vec<Phase>,
    pub verifications: Vec<Verification>,
}

/// 执行会话 (支持阶段性开发)
pub struct RunSession { /* ... */ }

impl RunSession {
    /// 获取事件流
    pub fn events(&self) -> impl Stream<Item = RunEvent>;

    /// 开始执行
    pub async fn start(&mut self) -> Result<()>;

    /// 暂停执行
    pub fn pause(&mut self);

    /// 恢复执行
    pub async fn resume(&mut self) -> Result<()>;

    /// 取消执行
    pub fn cancel(&mut self);
}

/// 执行事件
pub enum RunEvent {
    // 阶段事件
    PhaseStarted { name: String, index: usize, total: usize },
    PhaseProgress { message: String },
    PhaseCompleted { name: String },

    // Hook 事件
    HookStarted { name: String },
    HookPassed { name: String },
    HookFailed { name: String, error: String },
    HookFixing { name: String },
    HookFixed { name: String },

    // 审查事件
    ReviewStarted,
    ReviewIssueFound { issue: String },
    ReviewFixing { issue: String },
    ReviewFixed { issue: String },
    ReviewCompleted,

    // 提交事件
    CommitCreated { hash: String, message: String },

    // 验证事件
    VerificationStarted,
    VerificationProgress { check: String, passed: bool },
    VerificationFailed { failures: Vec<String> },
    VerificationFixing,
    VerificationPassed,

    // PR 事件
    PrCreated { url: String },

    // 错误事件
    Error { message: String },
}
```

**模块结构**:
```
gba-core/
├── src/
│   ├── lib.rs              # Public exports
│   ├── config.rs           # Config types
│   ├── runtime.rs          # Runtime implementation
│   ├── error.rs            # Error types
│   ├── session/            # Session management
│   │   ├── mod.rs
│   │   ├── plan.rs         # PlanSession
│   │   └── run.rs          # RunSession
│   ├── executor/           # Task executors
│   │   ├── mod.rs
│   │   ├── init.rs         # InitExecutor
│   │   ├── plan.rs         # PlanExecutor
│   │   ├── run.rs          # RunExecutor
│   │   └── phase.rs        # PhaseExecutor
│   ├── hooks/              # Precommit hooks
│   │   ├── mod.rs
│   │   ├── runner.rs       # HookRunner
│   │   ├── build.rs
│   │   ├── fmt.rs
│   │   ├── lint.rs
│   │   └── security.rs
│   ├── review/             # Code review
│   │   ├── mod.rs
│   │   └── reviewer.rs     # AgentReviewer
│   ├── verify/             # Verification
│   │   ├── mod.rs
│   │   └── verifier.rs     # Verifier
│   ├── agent/              # Agent wrapper
│   │   ├── mod.rs
│   │   └── runner.rs       # AgentRunner
│   ├── git/                # Git operations
│   │   ├── mod.rs
│   │   └── ops.rs          # commit, branch, pr
│   └── spec/               # Spec file handling
│       ├── mod.rs
│       ├── parser.rs
│       └── types.rs
```

---

### 3. `gba-cli` (Command Line Interface)

**职责**: 提供命令行界面和 TUI 交互

```rust
// ============================================================
// CLI Commands
// ============================================================

/// CLI 入口
#[derive(Parser)]
#[command(name = "gba", about = "Geektime Bootcamp Agent")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Claude API Key (overrides env and config)
    #[arg(long, env = "ANTHROPIC_API_KEY", global = true)]
    pub api_key: Option<String>,

    /// Claude API Base URL (overrides env and config)
    #[arg(long, env = "ANTHROPIC_BASE_URL", global = true)]
    pub base_url: Option<String>,

    /// Model to use (overrides config)
    #[arg(long, env = "GBA_MODEL", global = true)]
    pub model: Option<String>,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize GBA for current project
    Init,

    /// Plan a new feature (enters TUI)
    Plan {
        /// Feature identifier (e.g., "add-auth", "refactor-api")
        feature_slug: String,
    },

    /// Execute feature implementation
    Run {
        /// Feature identifier
        feature_slug: String,

        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,

        /// Resume from specific phase (0-indexed)
        #[arg(long)]
        from_phase: Option<usize>,
    },

    /// List all features
    List,

    /// Show feature status
    Status {
        /// Feature identifier (shows all if omitted)
        feature_slug: Option<String>,
    },
}
```

**模块结构**:
```
gba-cli/
├── src/
│   ├── main.rs             # Entry point
│   ├── cli.rs              # CLI definition
│   ├── commands/           # Command handlers
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── plan.rs
│   │   ├── run.rs
│   │   ├── list.rs
│   │   └── status.rs
│   ├── tui/                # TUI components
│   │   ├── mod.rs
│   │   ├── app.rs          # App state
│   │   ├── ui.rs           # UI rendering
│   │   ├── event.rs        # Event handling
│   │   └── widgets/        # Custom widgets
│   │       ├── mod.rs
│   │       ├── chat.rs     # Chat display (for plan)
│   │       ├── progress.rs # Progress display (for run)
│   │       ├── phase.rs    # Phase status widget
│   │       └── input.rs    # Input field
│   └── output/             # Output formatting
│       ├── mod.rs
│       ├── spinner.rs      # Progress spinner
│       └── table.rs        # Table output
```

---

## Data Flow

```
┌──────────┐     ┌──────────┐     ┌──────────┐
│ gba-cli  │────▶│ gba-core │────▶│  gba-pm  │
│          │◀────│          │◀────│          │
└──────────┘     └──────────┘     └──────────┘
     │                │                │
     │                │                │
     ▼                ▼                ▼
┌──────────┐     ┌──────────┐     ┌──────────┐
│ Terminal │     │ Claude   │     │ Template │
│   I/O    │     │ Agent    │     │  Files   │
└──────────┘     │   SDK    │     └──────────┘
                 └──────────┘
                      │
                      ▼
                 ┌──────────┐
                 │  .gba/   │
                 │ features │
                 └──────────┘
```

---

## Development Plan

### Phase 1: Foundation (基础架构)

**目标**: 搭建基础框架，实现最小可用版本

| Task | Crate | Description |
|------|-------|-------------|
| 1.1 | gba-pm | 实现 PromptManager 基础结构 |
| 1.2 | gba-pm | 实现 Context 和模板渲染 |
| 1.3 | gba-pm | 添加内置模板 (init, plan_start) |
| 1.4 | gba-core | 实现 Config 和 Runtime 基础 |
| 1.5 | gba-core | 实现 AgentRunner (封装 claude-agent-sdk-rs) |
| 1.6 | gba-cli | 实现 CLI 参数解析 |
| 1.7 | gba-cli | 实现基础输出格式化 |

**交付物**: 可编译的框架代码，能解析命令但不执行

---

### Phase 2: Init Command (初始化命令)

**目标**: 实现 `gba init` 完整功能

| Task | Crate | Description |
|------|-------|-------------|
| 2.1 | gba-pm | 完善 init 模板 |
| 2.2 | gba-core | 实现 InitExecutor |
| 2.3 | gba-core | 实现目录结构创建 (.gba/, .trees/) |
| 2.4 | gba-core | 实现 repo 分析逻辑 |
| 2.5 | gba-core | 实现 .gba.md 生成 |
| 2.6 | gba-core | 实现 CLAUDE.md 更新 |
| 2.7 | gba-cli | 实现 init 命令处理 |
| 2.8 | gba-cli | 添加进度显示 |

**交付物**: 可用的 `gba init` 命令

---

### Phase 3: Plan Command (规划命令)

**目标**: 实现 `gba plan` 完整功能，支持 feedback loop

| Task | Crate | Description |
|------|-------|-------------|
| 3.1 | gba-pm | 添加 plan_start/plan_refine/plan_generate 模板 |
| 3.2 | gba-core | 实现 PlanSession 和 feedback loop |
| 3.3 | gba-core | 实现对话状态管理 |
| 3.4 | gba-core | 实现 ProposedDesign 解析 |
| 3.5 | gba-core | 实现 spec 文件生成 (development_plan/design/verification_plan) |
| 3.6 | gba-cli | 实现 TUI 框架 (ratatui) |
| 3.7 | gba-cli | 实现聊天界面 |
| 3.8 | gba-cli | 实现输入处理和会话持久化 |

**交付物**: 可用的 `gba plan <feature>` 命令

---

### Phase 4: Run Command - Core (执行命令核心)

**目标**: 实现 `gba run` 阶段性开发功能

| Task | Crate | Description |
|------|-------|-------------|
| 4.1 | gba-pm | 添加 execute_phase 模板 |
| 4.2 | gba-core | 实现 RunSession 和事件流 |
| 4.3 | gba-core | 实现 PhaseExecutor |
| 4.4 | gba-core | 实现 spec 文件解析 |
| 4.5 | gba-core | 实现 git 操作 (commit) |
| 4.6 | gba-cli | 实现执行进度显示 |
| 4.7 | gba-cli | 实现 phase 状态 widget |

**交付物**: 可执行阶段性开发的 `gba run` 命令

---

### Phase 5: Run Command - Hooks & Review (执行命令钩子和审查)

**目标**: 实现 precommit hooks 和 agent review

| Task | Crate | Description |
|------|-------|-------------|
| 5.1 | gba-pm | 添加 fix_hook_error/review/fix_review_issue 模板 |
| 5.2 | gba-core | 实现 HookRunner |
| 5.3 | gba-core | 实现 build/fmt/lint/security hooks |
| 5.4 | gba-core | 实现 hook 失败自动修复循环 |
| 5.5 | gba-core | 实现 AgentReviewer |
| 5.6 | gba-core | 实现 review issue 自动修复循环 |
| 5.7 | gba-cli | 更新进度显示支持 hook/review 事件 |

**交付物**: 带 hooks 和 review 的 `gba run` 命令

---

### Phase 6: Run Command - Verification & PR (执行命令验证和PR)

**目标**: 实现验证和 PR 提交

| Task | Crate | Description |
|------|-------|-------------|
| 6.1 | gba-pm | 添加 verify/fix_verify_error 模板 |
| 6.2 | gba-core | 实现 Verifier |
| 6.3 | gba-core | 实现验证失败自动修复循环 |
| 6.4 | gba-core | 实现 PR 创建 (gh cli) |
| 6.5 | gba-cli | 更新进度显示支持 verify/pr 事件 |

**交付物**: 完整的 `gba run` 命令

---

### Phase 7: Polish & Testing (完善与测试)

**目标**: 完善功能，添加测试

| Task | Crate | Description |
|------|-------|-------------|
| 7.1 | gba-cli | 实现 list/status 命令 |
| 7.2 | gba-core | 实现执行状态持久化 (state.json) |
| 7.3 | gba-core | 实现 --from-phase 断点续执行 |
| 7.4 | all | 添加单元测试 |
| 7.5 | all | 添加集成测试 |
| 7.6 | all | 完善错误处理 |
| 7.7 | all | 添加日志记录 |
| 7.8 | gba-cli | 改进 UX |
| 7.9 | - | 编写用户文档 |

**交付物**: 生产可用的 GBA 工具

---

## Error Handling Strategy

```rust
// 统一的错误类型
#[derive(Debug, Error)]
pub enum GbaError {
    #[error("Project not initialized. Run `gba init` first.")]
    NotInitialized,

    #[error("Project already initialized at {0}")]
    AlreadyInitialized(PathBuf),

    #[error("Feature not found: {0}")]
    FeatureNotFound(String),

    #[error("Invalid spec file: {0}")]
    InvalidSpec(String),

    #[error("Hook failed: {hook} - {message}")]
    HookFailed { hook: String, message: String },

    #[error("Verification failed: {0:?}")]
    VerificationFailed(Vec<String>),

    #[error("Max retry attempts reached for {operation}")]
    MaxRetryReached { operation: String },

    #[error("Agent error: {0}")]
    Agent(#[from] AgentError),

    #[error("Template error: {0}")]
    Template(#[from] PromptError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git error: {0}")]
    Git(String),
}
```

---

## Configuration

### Configuration Hierarchy

GBA 采用分层配置策略，优先级从高到低：

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        Configuration Priority                            │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  1. CLI Arguments (--api-key, --base-url, --model)     ← Highest        │
│                          │                                               │
│                          ▼                                               │
│  2. Environment Variables                                                │
│     - ANTHROPIC_API_KEY                                                  │
│     - ANTHROPIC_BASE_URL                                                 │
│                          │                                               │
│                          ▼                                               │
│  3. User Config: ~/.config/gba/config.yml                               │
│     - API credentials (safe from git)                                    │
│     - Default preferences                                                │
│                          │                                               │
│                          ▼                                               │
│  4. Project Config: .gba/config.yml                    ← Lowest         │
│     - Project-specific settings                                          │
│     - NO sensitive data (committed to git)                               │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### `~/.config/gba/config.yml` - 用户全局配置

**用途**: 存储用户级别的配置，包括敏感凭据

**重要**: 此文件包含 API key，不应被版本控制

```yaml
# ~/.config/gba/config.yml

# Claude API 配置 (敏感信息，不要提交到 git)
api:
  key: "sk-ant-api03-xxxxx"           # 或使用环境变量 $ANTHROPIC_API_KEY
  base_url: "https://api.anthropic.com"  # 可选，用于代理或私有部署

# 默认 Agent 配置
agent:
  model: "claude-sonnet-4-20250514"   # 默认模型
  max_tokens: 8192                    # 默认 max tokens

# 用户偏好
preferences:
  auto_confirm: false                 # 是否跳过确认提示
  verbose: false                      # 详细输出
```

### Environment Variables

支持的环境变量：

| 变量 | 说明 | 示例 |
|------|------|------|
| `ANTHROPIC_API_KEY` | Claude API Key | `sk-ant-api03-xxxxx` |
| `ANTHROPIC_BASE_URL` | API Base URL | `https://api.anthropic.com` |
| `GBA_MODEL` | 默认模型 | `claude-sonnet-4-20250514` |
| `GBA_CONFIG` | 全局配置路径 | `~/.config/gba/config.yml` |

**推荐做法**:
```bash
# 在 ~/.bashrc 或 ~/.zshrc 中设置
export ANTHROPIC_API_KEY="sk-ant-api03-xxxxx"
```

### `.gba/config.yml` - 项目配置

**用途**: 存储 GBA 项目级别的配置，包括：
- Agent 模型和参数配置
- Precommit hooks 启用/禁用
- 执行策略（自动提交、自动审查等）
- 验证阈值和规则

**为什么需要**:
1. 不同项目可能使用不同的 Claude 模型或参数
2. 不同项目的 hooks 需求不同（如 Python 项目不需要 `cargo fmt`）
3. 执行策略需要可配置（有些项目可能不需要自动 PR）
4. 验证标准因项目而异（覆盖率阈值、lint 规则等）

```yaml
# .gba/config.yml

# 项目元信息
project:
  name: "my-project"
  initialized_at: "2024-01-01T00:00:00Z"

# Agent 配置
agent:
  model: "claude-sonnet-4-20250514"
  max_tokens: 8192

# Precommit hooks 配置
hooks:
  build: true
  fmt: true
  lint: true
  security_check: true
  custom:
    - "cargo test"
    - "cargo clippy"

# 执行策略
execution:
  auto_commit: true        # 每个阶段完成后自动提交
  auto_review: true        # 自动进行代码审查
  max_fix_attempts: 3      # 最大修复尝试次数
  create_pr: true          # 完成后自动创建 PR

# 验证配置
verification:
  run_tests: true
  check_coverage: true
  coverage_threshold: 80   # 代码覆盖率阈值 (%)
```

---

## State Management

### `.gba/features/{feature}/state.yml` - 执行状态

**用途**: 跟踪功能开发的执行状态，支持：
1. **断点续执行**: 如果执行中断，下次运行可以从中断点继续
2. **成本追踪**: 记录每个阶段的 API 调用次数和费用
3. **审计日志**: 记录完整的执行历史

```yaml
# .gba/features/add-auth/state.yml

# 功能元信息
feature: "add-auth"
created_at: "2024-01-01T10:00:00Z"
last_updated: "2024-01-01T12:30:00Z"

# 执行状态: pending | planning | running | paused | completed | failed
status: "running"

# 当前阶段 (0-indexed)
current_phase: 1

# 阶段详情
phases:
  - name: "setup"
    status: "completed"      # pending | in_progress | completed | failed
    commit: "abc1234"
    started_at: "2024-01-01T10:05:00Z"
    completed_at: "2024-01-01T10:15:00Z"
    turns: 3                 # Agent 对话轮次
    cost:
      input_tokens: 12500
      output_tokens: 3200
      total_usd: 0.045

  - name: "implement"
    status: "in_progress"
    commit: null
    started_at: "2024-01-01T10:16:00Z"
    completed_at: null
    turns: 5
    cost:
      input_tokens: 28000
      output_tokens: 8500
      total_usd: 0.112

  - name: "test"
    status: "pending"
    commit: null
    started_at: null
    completed_at: null
    turns: 0
    cost:
      input_tokens: 0
      output_tokens: 0
      total_usd: 0.0

# Hook 状态
hooks:
  passed: true
  last_run: "2024-01-01T10:14:00Z"
  failures: []

# Review 状态
review:
  passed: false
  issues_found: 2
  issues_fixed: 1
  last_run: "2024-01-01T12:25:00Z"

# Verification 状态
verification:
  passed: false
  checks:
    - name: "unit_tests"
      passed: true
    - name: "integration_tests"
      passed: false
      error: "Connection timeout in test_api_auth"
    - name: "coverage"
      passed: true
      value: 85

# PR 信息
pr:
  url: null                  # PR 创建后填充，如 "https://github.com/org/repo/pull/123"
  number: null
  created_at: null

# 总计统计
totals:
  turns: 8
  cost:
    input_tokens: 40500
    output_tokens: 11700
    total_usd: 0.157
```

---

## Resumable Execution

当 `gba run` 中断后重新执行时：

1. **检测中断点**: 读取 `state.yml`，找到 `status: "in_progress"` 的阶段
2. **恢复上下文**: 加载该阶段之前的所有完成状态和 commits
3. **继续执行**: 从中断的阶段继续，而不是从头开始

```
$ gba run add-auth
Resuming from phase 2/3: "implement"
Previous progress: setup ✓, implement (in progress)
Continuing...
```

---

## Future Considerations

1. **Web UI**: 未来可添加 Web 界面 (如截图中提到的)
2. **Plugin System**: 支持自定义执行步骤和 hooks
3. **Team Collaboration**: 多人协作功能
4. **Template Marketplace**: 共享模板市场
5. **CI/CD Integration**: 与 GitHub Actions 等集成
