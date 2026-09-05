pub mod engine;
pub mod llm;
pub mod prompt;
pub mod verifier;

pub use engine::{AgentAnswer, DialecticalAgent};
pub use llm::{FallbackLlmClient, LlmClient, OfflineLlmClient, OnlineLlmClient};
pub use prompt::{DIALECTICAL_SYSTEM_PROMPT, build_rag_user_prompt};
pub use verifier::{CitationVerifier, VerificationReport};
