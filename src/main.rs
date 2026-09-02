use clap::Parser;
use mao_agent::cli::{
    AskArgs, Cli, Commands, EmbedderArgs, IngestArgs, InitSamplesArgs, SearchArgs, StatsArgs,
};
use mao_agent::config::{ProjectConfig, nonempty_key};
use mao_agent::corpus::chunker::ChunkerConfig;
use mao_agent::corpus::ingest::CorpusScanner;
use mao_agent::model::{HistoricalPeriod, VectorFilter};
#[cfg(feature = "local-embed")]
use mao_agent::vector::embedder::FastEmbedder;
use mao_agent::vector::embedder::{
    COHERE_COMPAT_BASE_URL, DeterministicEmbedder, Embedder, OpenAIEmbedder, create_embedder_arc,
};
use mao_agent::vector::store::VectorStore;
use std::sync::Arc;
use tracing::{Level, info};
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

fn resolve_chat_api_key(cli_key: Option<String>) -> Option<String> {
    if let Some(key) = nonempty_key(cli_key.as_deref()) {
        return Some(key.to_string());
    }
    config_cohere_api_key()
}

fn get_embedder(args: &EmbedderArgs) -> Arc<dyn Embedder> {
    if args.offline {
        info!(
            "Using offline Deterministic Embedder ({}-dim)",
            args.embed_dim
        );
        return create_embedder_arc(DeterministicEmbedder::new(args.embed_dim));
    }

    let api_key = resolve_embed_api_key(args);
    let base_url = args
        .embed_base_url
        .clone()
        .or_else(|| api_key.as_ref().map(|_| COHERE_COMPAT_BASE_URL.to_string()));

    if let Some(base_url) = base_url {
        info!(
            "Using remote OpenAI-compatible embedder {} at {} ({}-dim)",
            args.embed_model, base_url, args.embed_dim
        );
        return create_embedder_arc(OpenAIEmbedder::new(
            base_url,
            api_key,
            args.embed_model.clone(),
            args.embed_dim,
        ));
    }

    #[cfg(feature = "local-embed")]
    {
        match FastEmbedder::try_new() {
            Ok(fe) => {
                info!(
                    "Using local FastEmbed ONNX BGE-small-zh-v1.5 ({}-dim)",
                    fe.dimension()
                );
                create_embedder_arc(fe)
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to initialize FastEmbed: {e}. Falling back to deterministic embedder."
                );
                create_embedder_arc(DeterministicEmbedder::new(args.embed_dim))
            }
        }
    }
    #[cfg(not(feature = "local-embed"))]
    {
        info!(
            "local-embed feature disabled, using Deterministic Embedder ({}-dim)",
            args.embed_dim
        );
        create_embedder_arc(DeterministicEmbedder::new(args.embed_dim))
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
    let embedder = get_embedder(&args.embedder);
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
    let embedder = get_embedder(&args.embedder);
    let store = VectorStore::load_from_file(&args.index_file, embedder)?;
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
        println!(
            "🏆 [Rank {}] RRF得分: {:.5} (向量: {}, BM25: {}) | 《{}》 ({})",
            res.rank, res.rrf_score, vec_str, bm25_str, res.chunk.doc_title, res.chunk.date
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
    let start = std::time::Instant::now();
    let embedder = get_embedder(&args.embedder);
    let store = VectorStore::load_from_file(&args.index_file, embedder)?;
    let vec_results = store.search(&args.query, args.top_k * 2, filter).await?;

    let bm25_results = if args.tantivy_dir.exists() {
        let ft_index = mao_agent::index::FullTextIndex::new_in_dir(&args.tantivy_dir)?;
        ft_index
            .search(&args.query, args.top_k * 2, filter)
            .unwrap_or_default()
    } else {
        Vec::new()
    };

    let coordinator = mao_agent::index::HybridSearchCoordinator::default();
    let hybrid_results = coordinator.fuse(vec_results, bm25_results, args.top_k);
    let duration = start.elapsed();
    println!(
        "⚡ 双路混合 (BM25 + 向量 RRF) 检索耗时: {:.2?}，融合召回 {} 条结果\n",
        duration,
        hybrid_results.len()
    );

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

    let embedder = get_embedder(&args.embedder);
    let store = VectorStore::load_from_file(&args.index_file, embedder)?;
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
            "lun_chi_jiu_zhan.md",
            include_str!("assets/samples/lun_chi_jiu_zhan.md"),
        ),
        (
            "mao_dun_lun.md",
            include_str!("assets/samples/mao_dun_lun.md"),
        ),
        (
            "shi_jian_lun.md",
            include_str!("assets/samples/shi_jian_lun.md"),
        ),
        (
            "xing_xing_zhi_huo.md",
            include_str!("assets/samples/xing_xing_zhi_huo.md"),
        ),
    ];

    for (filename, content) in samples {
        let file_path = args.target_dir.join(filename);
        std::fs::write(&file_path, content)?;
        println!("✨ 生成示例语料文档: {}", file_path.display());
    }

    println!(
        "\n已在 `{}` 目录下初始化 4 篇经典文献示例语料！",
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

    let embedder = get_embedder(&args.embedder);
    let store = Arc::new(VectorStore::load_from_file(&args.index_file, embedder)?);

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
    let agent = mao_agent::agent::DialecticalAgent::new(
        store,
        ft_index,
        Some(args.base_url.clone()),
        resolve_chat_api_key(args.api_key.clone()),
        Some(args.model.clone()),
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
