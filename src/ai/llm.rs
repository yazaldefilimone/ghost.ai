use rig::completion::Prompt;
use rig::providers::ollama;
use std::error::Error;

pub async fn prompt(prompt: &str) -> Result<String, Box<dyn Error>> {
	let agent = ollama::Client::new()
		.agent("llama3.2")
		.preamble("Be precise and concise.")
		.temperature(0.5)
		.build();
	let response = agent.prompt(prompt).await?;
	Ok(response)
}
pub async fn generate_suggestion(input: &str) -> Result<String, Box<dyn Error>> {
	// System prompt adaptado
	let system_prompt = r#"
    Você é um assistente de autocompletar para teclados de celular.
    Dada uma entrada com <CURSOR> no final, sugira a próxima palavra curta (máximo 8 letras) ou expressão curta (máximo 2 palavras pequenas) para substituir <CURSOR>.
    Analise o contexto antes de <CURSOR> e sugira algo natural para conversas casuais.
    Não repita palavras do input antes de <CURSOR>, não complete frases inteiras e mantenha a sugestão simples.
    Responda apenas com a sugestão, sem texto extra.
    "#;

	let clean_input = format!("{} <CURSOR>", input.trim());
	let agent = ollama::Client::new()
		.agent("llama3.2")
		.preamble(system_prompt) // Define o comportamento
		.temperature(0.1) // Previsibilidade
		.max_tokens(4) // Palavras curtas
		.build();

	let response = agent.prompt(clean_input).await?;
	Ok(response.trim().to_string())
}
