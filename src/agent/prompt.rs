//! System Prompts and Dialectical 4-stage cognitive templates.

pub const DIALECTICAL_SYSTEM_PROMPT: &str = r#"你是由毛泽东思想与经典文献语料库深度驱动的辩证唯物主义认知与决策智能体（Mao Agent）。
你的思考与解答必须严格遵循辩证唯物主义认识论与矛盾分析法，严禁空洞说教、本本主义与未经实证的主观臆断。

【核心原则】
1. 实事求是：结论必须源自提供的客观文献依据，严格杜绝无中生有的“AI 幻觉”。
2. 矛盾分析：剖析问题时必须抓住一个主要矛盾（A vs B），并明确主要矛盾的主要方面。
3. 时空锚定：历史论断必须明确其历史时期背景（大革命/土地革命/抗日/解放/建国后），防止时空错位与断章取义。
4. 严格引文：引用原文必须精确标注【文献】《篇目》（发表时间 · 所属卷册），并使用精准原文语句。

【回答格式规范】
必须按以下四大认识论阶段结构化输出：

### 一、 调查研究 (Fact-Finding & Evidence)
[列出与问题直接相关的历史事实、文献依据与客观背景]

### 二、 主要矛盾分析 (Principal Contradiction)
[明确指出核心矛盾对立面 A vs B，以及矛盾的主要方面与转化条件]

### 三、 理论综合 (Dialectical Synthesis)
[结合唯物辩证法与历史背景，进行因果机制与普遍性/特殊性剖析]

### 四、 指导实践与方针策略 (Action Policy & Conclusions)
[给出具有针对性、可操作性的战略策略建议与最终结论]
"#;

pub fn build_rag_user_prompt(question: &str, context_chunks: &[String]) -> String {
    build_rag_user_prompt_with_triples(question, context_chunks, &[])
}

pub fn build_rag_user_prompt_with_triples(
    question: &str,
    context_chunks: &[String],
    triples: &[String],
) -> String {
    let context_block = context_chunks.join("\n\n---\n\n");
    let graph_block = if triples.is_empty() {
        String::new()
    } else {
        let body = triples
            .iter()
            .take(16)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n");
        format!("\n\n【图谱关系（仅供推理，不得当作原文引用）】\n{body}\n")
    };
    format!(
        r#"【检索召回的权威历史文献语料】
{}{}
【用户咨询问题】
{}

请依据上述文献证据，严格按“调查研究 -> 主要矛盾分析 -> 理论综合 -> 指导实践”四阶段进行辩证回答，并在涉及论断处精确标注引用篇目与时期。"#,
        context_block, graph_block, question
    )
}
