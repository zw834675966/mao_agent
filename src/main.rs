use clap::Parser;
use mao_agent::cli::{
    AskArgs, Cli, Commands, EmbedderArgs, IngestArgs, InitSamplesArgs, SearchArgs, ServeArgs,
    StatsArgs,
};
use mao_agent::config::{ProjectConfig, nonempty_key};
use mao_agent::corpus::chunker::ChunkerConfig;
use mao_agent::corpus::ingest::CorpusScanner;
use mao_agent::model::{HistoricalPeriod, VectorFilter};
use mao_agent::vector::embedder::{
    Embedder, EmbedderSelection, resolve_embed_dimension, resolve_embedder,
};
use mao_agent::vector::store::VectorStore;
use std::path::Path;
use std::sync::Arc;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let log_level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).ok();

    match cli.command {
        Commands::InitSamples(args) => handle_init_samples(&args)?,
        Commands::Ingest(args) => handle_ingest(&args).await?,
        Commands::Search(args) => handle_search(&args).await?,
        Commands::Stats(args) => handle_stats(&args).await?,
        Commands::Ask(args) => handle_ask(&args).await?,
        Commands::Serve(args) => handle_serve(&args).await?,
    }

    Ok(())
}

fn config_cohere_api_key() -> Option<String> {
    ProjectConfig::try_load_default()?
        .cohere_api_key()
        .map(str::to_string)
}

fn resolve_embed_api_key(args: &EmbedderArgs) -> Option<String> {
    if let Some(key) = nonempty_key(args.embed_api_key.as_deref()) {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("COHERE_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    config_cohere_api_key()
}

fn resolve_chat_api_key(cli_key: Option<String>, offline: bool) -> Option<String> {
    if offline {
        return None;
    }
    if let Some(key) = nonempty_key(cli_key.as_deref()) {
        return Some(key.to_string());
    }
    config_cohere_api_key()
}

fn resolve_rerank_api_key(cli_key: Option<&str>, offline: bool, no_rerank: bool) -> Option<String> {
    if offline || no_rerank {
        return None;
    }
    if let Some(key) = nonempty_key(cli_key) {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("COHERE_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    if let Ok(key) = std::env::var("EMBED_API_KEY")
        && let Some(key) = nonempty_key(Some(key.as_str()))
    {
        return Some(key.to_string());
    }
    config_cohere_api_key()
}

fn make_reranker(
    offline: bool,
    no_rerank: bool,
    rerank_model: Option<String>,
    api_key_hint: Option<&str>,
) -> Option<std::sync::Arc<dyn mao_agent::Reranker>> {
    let key = resolve_rerank_api_key(api_key_hint, offline, no_rerank)?;
    Some(std::sync::Arc::new(mao_agent::CohereReranker::new(
        key,
        rerank_model,
        None,
    )))
}

fn get_embedder(
    args: &EmbedderArgs,
    cache_path: Option<&Path>,
) -> Result<Arc<dyn Embedder>, Box<dyn std::error::Error>> {
    let selection = EmbedderSelection {
        offline: args.offline,
        api_key: resolve_embed_api_key(args),
        base_url: args.embed_base_url.clone(),
        model: args.embed_model.clone(),
        dimension: resolve_embed_dimension(args.offline, args.embed_dim),
    };
    Ok(resolve_embedder(&selection, cache_path)?)
}

fn load_store_interactive(
    path: &Path,
    embedder: Arc<dyn Embedder>,
) -> Result<Option<VectorStore>, Box<dyn std::error::Error>> {
    match VectorStore::load_from_file(path, embedder) {
        Ok(s) => Ok(Some(s)),
        Err(mao_agent::VectorError::IdentityMismatch {
            snapshot_model,
            snapshot_dimension,
            source_model,
            source_dimension,
        }) => {
            eprintln!(
                "❌ 向量模型不匹配：当前索引 snapshot 采用模型 `{}` ({} 维)，而检索请求配置为 `{}` ({} 维)。\n💡 提示：若需在离线模式下检索，请先运行 `cargo run -- ingest --offline` 重建离线索引；若需在线检索，请移除 `--offline` 参数。",
                snapshot_model, snapshot_dimension, source_model, source_dimension
            );
            Ok(None)
        }
        Err(e) => Err(Box::new(e)),
    }
}

fn build_filter(
    period: Option<&str>,
    volume: Option<&str>,
    category: Option<&str>,
) -> Option<VectorFilter> {
    if period.is_none() && volume.is_none() && category.is_none() {
        return None;
    }
    let mut filter = VectorFilter::new();
    if let Some(p) = period {
        filter = filter.with_period(HistoricalPeriod::from_str_or_date(p));
    }
    if let Some(v) = volume {
        filter = filter.with_volume(v);
    }
    if let Some(c) = category {
        filter = filter.with_category(c);
    }
    Some(filter)
}

async fn handle_ingest(args: &IngestArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("📚 开始摄取语料库: {}", args.corpus_dir.display());
    let embedder = get_embedder(&args.embedder, Some(&args.index_file))?;
    let chunker_config = ChunkerConfig {
        max_chars: args.max_chars,
        min_chars: 100,
        overlap_chars: 50,
        inject_context_header: true,
    };

    let store = VectorStore::new(embedder, Some(chunker_config.clone()));
    let docs = CorpusScanner::load_documents_from_dir(&args.corpus_dir)?;

    if docs.is_empty() {
        println!("⚠️  在指定目录下未找到有效的 Markdown 文档！");
        return Ok(());
    }

    println!(
        "📄 成功加载 {} 篇文献文档，正在生成分块与向量嵌入...",
        docs.len()
    );
    let _total_chunks = store.index_documents(&docs, args.batch_size).await?;

    println!("💾 正在将向量索引持久化至: {}", args.index_file.display());
    store.save_to_file(&args.index_file).await?;

    println!(
        "🔎 正在构建 Tantivy 全文 BM25 倒排索引: {}",
        args.tantivy_dir.display()
    );
    if args.tantivy_dir.exists() {
        std::fs::remove_dir_all(&args.tantivy_dir)?;
    }
    let chunker = mao_agent::corpus::ChineseSemanticChunker::new(chunker_config);
    let ft_index = mao_agent::index::FullTextIndex::new_in_dir(&args.tantivy_dir)?;
    for doc in &docs {
        let chunks = chunker.chunk_document(doc);
        ft_index.insert_batch(&chunks)?;
    }

    let stats = store.stats().await;
    println!("\n========================================================");
    println!("🎉 混合向量与全文数据库构建完成！");
    println!(" • 索引文档数 (Documents): {}", stats.total_documents);
    println!(" • 索引分块数 (Chunks):    {}", stats.total_vectors);
    println!(" • 向量维度 (Dimension):   {}", stats.vector_dimension);
    println!(
        " • 字符总数 (Characters):  {}",
        stats.total_characters_indexed
    );
    println!(
        " • 向量索引文件:           {} KB",
        std::fs::metadata(&args.index_file)?.len() / 1024
    );
    println!(" • BM25 倒排索引目录:      {}", args.tantivy_dir.display());
    println!("========================================================\n");

    Ok(())
}

fn print_search_header(args: &SearchArgs) {
    println!(
        "\n🔍 执行检索 [模式: {}]: \"{}\" (Top-{})",
        args.mode, args.query, args.top_k
    );
    if let Some(ref p) = args.period {
        println!(" • 历史时期过滤: {}", p);
    }
    if let Some(ref v) = args.volume {
        println!(" • 卷册过滤:     {}", v);
    }
}

fn print_bm25_results(results: &[mao_agent::index::FullTextSearchResult]) {
    for res in results {
        println!(
            "--------------------------------------------------------------------------------"
        );
        println!(
            "🏆 [Rank {}] BM25得分: {:.4} | 《{}》 ({})",
            res.rank, res.score, res.chunk.doc_title, res.chunk.date
        );
        println!(
            "📌 时期: {} | 卷册: {}",
            res.chunk.period.as_str(),
            if res.chunk.volume.is_empty() {
                "无"
            } else {
                &res.chunk.volume
            }
        );
        println!("\n📖 原文段落:\n{}\n", res.chunk.raw_text);
    }
}

fn search_bm25(
    args: &SearchArgs,
    filter: Option<&VectorFilter>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !args.tantivy_dir.exists() {
        eprintln!(
            "❌ Tantivy 索引目录未找到: {}。请先运行 `mao_agent ingest` 构建索引。",
            args.tantivy_dir.display()
        );
        return Ok(());
    }
    let start = std::time::Instant::now();
    let ft_index = mao_agent::index::FullTextIndex::new_in_dir(&args.tantivy_dir)?;
    let results = ft_index.search(&args.query, args.top_k, filter)?;
    let duration = start.elapsed();
    println!(
        "⚡ BM25 检索耗时: {:.2?}，召回 {} 条候选结果\n",
        duration,
        results.len()
    );

    print_bm25_results(&results);
    Ok(())
}

fn print_vector_results(results: &[mao_agent::model::VectorSearchResult], min_score: f32) {
    for res in results {
        if res.score < min_score {
            continue;
        }
        println!(
            "--------------------------------------------------------------------------------"
        );
        println!(
            "🏆 [Rank {}] 相似度得分: {:.4} | 《{}》 ({})",
            res.rank, res.score, res.chunk.doc_title, res.chunk.date
        );
        println!(
            "📌 时期: {} | 卷册: {}",
            res.chunk.period.as_str(),
            if res.chunk.volume.is_empty() {
                "无"
            } else {
                &res.chunk.volume
            }
        );
        println!("\n📖 原文段落:\n{}\n", res.chunk.raw_text);
    }
}

async fn search_vector(
    args: &SearchArgs,
    filter: Option<&VectorFilter>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !args.index_file.exists() {
        eprintln!(
            "❌ 向量索引文件未找到: {}。请先运行 `mao_agent ingest` 构建索引。",
            args.index_file.display()
        );
        return Ok(());
    }
    let start = std::time::Instant::now();
    let embedder = get_embedder(&args.embedder, Some(&args.index_file))?;
    let store = match load_store_interactive(&args.index_file, embedder)? {
        Some(s) => s,
        None => return Ok(()),
    };
    let results = store.search(&args.query, args.top_k, filter).await?;
    let duration = start.elapsed();
    println!(
        "⚡ 向量检索耗时: {:.2?}，召回 {} 条候选结果\n",
        duration,
        results.len()
    );

    print_vector_results(&results, args.min_score);
    Ok(())
}

fn print_hybrid_results(results: &[mao_agent::index::HybridSearchResult]) {
    for res in results {
        println!(
            "--------------------------------------------------------------------------------"
        );
        let vec_str = res
            .vector_score
            .map(|s| format!("{:.4}", s))
            .unwrap_or_else(|| "N/A".into());
        let bm25_str = res
            .bm25_score
            .map(|s| format!("{:.2}", s))
            .unwrap_or_else(|| "N/A".into());
        let rerank_str = res
            .rerank_score
            .map(|s| format!("{:.4}", s))
            .unwrap_or_else(|| "N/A".into());
        println!(
            "🏆 [Rank {}] RRF得分: {:.5} (向量: {}, BM25: {}, Rerank: {}) | 《{}》 ({})",
            res.rank,
            res.rrf_score,
            vec_str,
            bm25_str,
            rerank_str,
            res.chunk.doc_title,
            res.chunk.date
        );
        println!(
            "📌 时期: {} | 卷册: {}",
            res.chunk.period.as_str(),
            if res.chunk.volume.is_empty() {
                "无"
            } else {
                &res.chunk.volume
            }
        );
        println!("\n📖 原文段落:\n{}\n", res.chunk.raw_text);
    }
}

async fn search_hybrid(
    args: &SearchArgs,
    filter: Option<&VectorFilter>,
) -> Result<(), Box<dyn std::error::Error>> {
    if !args.index_file.exists() {
        eprintln!(
            "❌ 向量索引文件未找到: {}。请先运行 `mao_agent ingest` 构建索引。",
            args.index_file.display()
        );
        return Ok(());
    }
    let start = std::time::Instant::now();
    let embedder = get_embedder(&args.embedder, Some(&args.index_file))?;
    let store = match load_store_interactive(&args.index_file, embedder)? {
        Some(s) => s,
        None => return Ok(()),
    };
    let vec_results = store.search(&args.query, args.top_k * 2, filter).await?;

    let bm25_results = if args.tantivy_dir.exists() {
        let ft_index = mao_agent::index::FullTextIndex::new_in_dir(&args.tantivy_dir)?;
        match ft_index.search(&args.query, args.top_k * 2, filter) {
            Ok(results) => results,
            Err(e) => {
                tracing::warn!("BM25 search failed: {e}, continuing with vector-only results.");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };

    let coordinator = mao_agent::index::HybridSearchCoordinator::default();
    let fused = coordinator.fuse(vec_results, bm25_results, args.top_k * 2);
    let reranker = make_reranker(
        args.embedder.offline,
        args.no_rerank,
        args.rerank_model.clone(),
        args.embedder.embed_api_key.as_deref(),
    );
    let rerank_start = std::time::Instant::now();
    let hybrid_results =
        mao_agent::rerank_or_fallback(fused, reranker.as_deref(), &args.query, args.top_k).await;
    let duration = start.elapsed();
    println!(
        "⚡ 双路混合 (BM25 + 向量 RRF) 检索耗时: {:.2?}，融合召回 {} 条结果\n",
        duration,
        hybrid_results.len()
    );
    if reranker.is_some() {
        println!("⚡ Rerank 耗时: {:.2?}\n", rerank_start.elapsed());
    }

    print_hybrid_results(&hybrid_results);
    Ok(())
}

async fn handle_search(args: &SearchArgs) -> Result<(), Box<dyn std::error::Error>> {
    let filter = build_filter(
        args.period.as_deref(),
        args.volume.as_deref(),
        args.category.as_deref(),
    );
    print_search_header(args);

    match args.mode.as_str() {
        "bm25" => search_bm25(args, filter.as_ref())?,
        "vector" => search_vector(args, filter.as_ref()).await?,
        _ => search_hybrid(args, filter.as_ref()).await?,
    }

    Ok(())
}

async fn handle_stats(args: &StatsArgs) -> Result<(), Box<dyn std::error::Error>> {
    if !args.index_file.exists() {
        eprintln!("❌ 索引文件未找到: {}", args.index_file.display());
        return Ok(());
    }

    let embedder = get_embedder(&args.embedder, Some(&args.index_file))?;
    let store = match load_store_interactive(&args.index_file, embedder)? {
        Some(s) => s,
        None => return Ok(()),
    };
    let stats = store.stats().await;

    println!("\n================ 📊 向量数据库状态报告 ================");
    println!(" • 索引文件路径:       {}", args.index_file.display());
    println!(" • 索引分块总量:       {} chunks", stats.total_vectors);
    println!(" • 收录历史文献数:     {} documents", stats.total_documents);
    println!(" • 向量嵌入维度:       {} dims", stats.vector_dimension);
    println!(
        " • 索引字符总数:       {} chars",
        stats.total_characters_indexed
    );
    println!(
        " • 预估内存占用:       {:.2} MB",
        stats.estimated_memory_bytes as f64 / 1_048_576.0
    );

    println!("\n🏛️  历史时期分布:");
    let mut periods: Vec<_> = stats.period_distribution.into_iter().collect();
    periods.sort_by_key(|a| std::cmp::Reverse(a.1));
    for (period, count) in periods {
        println!("   - {:<28} : {} chunks", period, count);
    }

    if !stats.volume_distribution.is_empty() {
        println!("\n📚 卷册文献分布:");
        let mut volumes: Vec<_> = stats.volume_distribution.into_iter().collect();
        volumes.sort_by_key(|a| std::cmp::Reverse(a.1));
        for (vol, count) in volumes {
            println!("   - {:<20} : {} chunks", vol, count);
        }
    }
    println!("========================================================\n");

    Ok(())
}

fn handle_init_samples(args: &InitSamplesArgs) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&args.target_dir)?;

    let samples = [
        (
            "fan_dui_ben_ben_zhu_yi.md",
            include_str!("assets/samples/fan_dui_ben_ben_zhu_yi.md"),
        ),
        (
            "gai_zao_wo_men_de_xue_xi.md",
            include_str!("assets/samples/gai_zao_wo_men_de_xue_xi.md"),
        ),
        (
            "guan_yu_ling_dao_fang_fa.md",
            include_str!("assets/samples/guan_yu_ling_dao_fang_fa.md"),
        ),
        (
            "guan_yu_zheng_que_chu_li_ren_min_nei_bu_mao_dun.md",
            include_str!("assets/samples/guan_yu_zheng_que_chu_li_ren_min_nei_bu_mao_dun.md"),
        ),
        (
            "hu_nan_nong_min_yun_dong.md",
            include_str!("assets/samples/hu_nan_nong_min_yun_dong.md"),
        ),
        (
            "lun_chi_jiu_zhan.md",
            include_str!("assets/samples/lun_chi_jiu_zhan.md"),
        ),
        (
            "lun_ren_min_min_zhu_zhuan_zheng.md",
            include_str!("assets/samples/lun_ren_min_min_zhu_zhuan_zheng.md"),
        ),
        (
            "mao_dun_lun.md",
            include_str!("assets/samples/mao_dun_lun.md"),
        ),
        (
            "qi_jie_er_zhong_quan_hui.md",
            include_str!("assets/samples/qi_jie_er_zhong_quan_hui.md"),
        ),
        (
            "ren_de_zheng_que_si_xiang.md",
            include_str!("assets/samples/ren_de_zheng_que_si_xiang.md"),
        ),
        (
            "scholarship_domestic_studies.md",
            include_str!("assets/samples/scholarship_domestic_studies.md"),
        ),
        (
            "scholarship_international_studies.md",
            include_str!("assets/samples/scholarship_international_studies.md"),
        ),
        (
            "shi_jian_lun.md",
            include_str!("assets/samples/shi_jian_lun.md"),
        ),
        (
            "xing_xing_zhi_huo.md",
            include_str!("assets/samples/xing_xing_zhi_huo.md"),
        ),
        (
            "zhong_guo_she_hui_ge_jie_ji.md",
            include_str!("assets/samples/zhong_guo_she_hui_ge_jie_ji.md"),
        ),
    ];

    for (filename, content) in samples {
        let file_path = args.target_dir.join(filename);
        std::fs::write(&file_path, content)?;
        println!("✨ 生成示例语料文档: {}", file_path.display());
    }

    println!(
        "\n已在 `{}` 目录下初始化 15 篇经典文献与权威学术研究语料！",
        args.target_dir.display()
    );
    Ok(())
}

fn render_citation_reports(reports: &[mao_agent::agent::VerificationReport]) {
    println!("\n🔍 引用溯源与真子串核验报告 (Attribution Verification):");
    if reports.is_empty() {
        println!("  • 未抽取到显式双引号引语。");
    } else {
        for (i, rep) in reports.iter().enumerate() {
            let status_icon = if rep.is_verified {
                "✅ [真子串核验通过]"
            } else {
                "⚠️ [存疑/未匹配]"
            };
            println!(
                "  [{}] {} 置信度: {:.1}% | 《{}》\n      引文: “{}”",
                i + 1,
                status_icon,
                rep.match_confidence * 100.0,
                rep.claimed_doc_title,
                rep.quote
            );
            if let Some(ref warn) = rep.warning {
                println!("      提示: {}", warn);
            }
        }
    }
}

fn render_retrieved_chunks(chunks: &[mao_agent::model::DocumentChunk]) {
    println!("\n📚 支撑文献依据 (Retrieved Context):");
    for chunk in chunks {
        println!(
            "  - 《{}》 ({}) · {} | Chunk ID: {}",
            chunk.doc_title,
            chunk.date,
            chunk.period.as_str(),
            chunk.chunk_id
        );
    }
}

async fn handle_ask(args: &AskArgs) -> Result<(), Box<dyn std::error::Error>> {
    if !args.index_file.exists() {
        eprintln!(
            "❌ 索引文件未找到: {}。请先运行 `mao_agent ingest` 构建索引。",
            args.index_file.display()
        );
        return Ok(());
    }

    let embedder = get_embedder(&args.embedder, Some(&args.index_file))?;
    let store = match load_store_interactive(&args.index_file, embedder)? {
        Some(s) => Arc::new(s),
        None => return Ok(()),
    };

    let ft_index = if args.tantivy_dir.exists() {
        match mao_agent::index::FullTextIndex::new_in_dir(&args.tantivy_dir) {
            Ok(idx) => Some(Arc::new(idx)),
            Err(e) => {
                tracing::warn!(
                    "Failed to load Tantivy index: {e}, falling back to vector-only retrieval."
                );
                None
            }
        }
    } else {
        None
    };

    let filter = build_filter(args.period.as_deref(), None, None);
    let reranker = make_reranker(
        args.embedder.offline,
        args.no_rerank,
        args.rerank_model.clone(),
        args.api_key
            .as_deref()
            .or(args.embedder.embed_api_key.as_deref()),
    );
    let agent = mao_agent::agent::DialecticalAgent::new(
        store,
        ft_index,
        Some(args.base_url.clone()),
        resolve_chat_api_key(args.api_key.clone(), args.embedder.offline),
        Some(args.model.clone()),
        reranker,
    );

    println!("\n🧠 【Mao Agent 辩证认知推理引擎】");
    println!("❓ 提问: \"{}\"", args.question);
    println!(
        "⚙️  检索 Top-{} 核心文献证据并执行四阶段认识论推演...\n",
        args.top_k
    );

    let start = std::time::Instant::now();
    let answer = agent
        .ask(&args.question, args.top_k, filter.as_ref())
        .await?;
    let duration = start.elapsed();

    println!("================================================================================");
    println!("{}", answer.content);
    println!("================================================================================");

    render_citation_reports(&answer.citation_reports);
    render_retrieved_chunks(&answer.retrieved_chunks);
    println!("\n⚡ 推演总耗时: {:.2?}", duration);

    Ok(())
}

async fn handle_serve(args: &ServeArgs) -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = args.bind.parse().map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Invalid --bind '{}': {e}", args.bind),
        )
    })?;

    // 1) Embedder + VectorStore (身份校验与 search 共用同一 embedder)
    let embedder = get_embedder(&args.embedder, Some(&args.index_file))?;
    let store = match load_store_interactive(&args.index_file, embedder)? {
        Some(s) => Arc::new(s),
        None => return Ok(()),
    };
    let stats = store.stats().await;
    if stats.total_vectors == 0 {
        eprintln!(
            "⚠️  向量索引为空 ({}). 建议先运行 `mao_agent ingest` 构建索引。",
            args.index_file.display()
        );
    }

    // 2) Tantivy（可选，缺失时降级为纯向量）
    let tantivy = if args.tantivy_dir.exists() {
        match mao_agent::index::FullTextIndex::new_in_dir(&args.tantivy_dir) {
            Ok(idx) => Some(Arc::new(idx)),
            Err(e) => {
                eprintln!("⚠️  Tantivy 索引加载失败: {e}，将以纯向量模式提供服务。");
                None
            }
        }
    } else {
        eprintln!(
            "⚠️  Tantivy 目录未找到 ({}), 混合检索将降级为纯向量。",
            args.tantivy_dir.display()
        );
        None
    };

    // 3) LLM 兼容配置
    let chat_base_url = args.base_url.clone();
    let chat_api_key = resolve_chat_api_key(args.api_key.clone(), args.embedder.offline);
    let chat_model = args.model.clone();

    println!("\n🚀 Mao Agent API 服务启动中...");
    println!(" • 监听地址:          http://{addr}");
    println!(
        " • 向量索引:          {} ({} chunks, {} 维)",
        args.index_file.display(),
        stats.total_vectors,
        stats.vector_dimension
    );
    println!(
        " • 全文索引:          {} ({})",
        args.tantivy_dir.display(),
        if tantivy.is_some() {
            "已加载"
        } else {
            "未加载/降级"
        }
    );
    println!(" • LLM Base URL:      {chat_base_url}");
    println!(" • 模型:              {chat_model}");
    println!(
        " • API Key:           {}",
        if chat_api_key.is_some() {
            "已配置"
        } else {
            "未配置 (离线推演)"
        }
    );
    println!();

    let hybrid = mao_agent::index::HybridSearchCoordinator::default();
    let reranker = make_reranker(
        args.embedder.offline,
        args.no_rerank,
        args.rerank_model.clone(),
        args.api_key
            .as_deref()
            .or(args.embedder.embed_api_key.as_deref()),
    );
    println!(
        " • Rerank:             {}",
        if reranker.is_some() {
            "已启用 (Cohere)"
        } else {
            "未启用 (offline / --no-rerank / 无 key)"
        }
    );
    mao_agent::server::serve(
        store,
        tantivy,
        hybrid,
        reranker,
        chat_base_url,
        chat_api_key,
        chat_model,
        addr,
    )
    .await?;
    Ok(())
}
