use rig::agent::AgentBuilder;
use rig::completion::Prompt;
use rig::providers::ollama;
use std::error::Error;

use super::ctx::Context;

pub async fn prompt(prompt: &str) -> Result<String, Box<dyn Error>> {
	let agent = ollama::Client::new()
		.agent("llama3.2")
		.preamble("Be precise and concise.")
		.temperature(0.5)
		.build();
	let response = agent.prompt(prompt).await?;
	Ok(response)
}
pub async fn generate_suggestion(ctx: &Context) -> Result<String, Box<dyn Error>> {
	// System prompt adaptado
	let system_prompt = format!(
		r#"
Você é um sistema de autocompletar absurdamente inteligente, embutido em teclados de celular.
Seu papel é prever o que o usuário está prestes a digitar, com base no que ele já escreveu e no que viveu ao longo do dia.

Diferente de autocompletes comuns, você entende contexto real: o que o usuário leu, escreveu ou comentou hoje em diferentes aplicativos (como redes sociais, mensageiros, e-mails, etc).
Você lembra de memes recentes, nomes de marcas, projetos, pessoas e codenomes usados pelo usuário.

Você recebe entradas com a marcação <CURSOR> indicando onde o usuário parou de digitar.

Seu objetivo é sugerir uma continuação natural da frase, como o próprio usuário escreveria.
A sugestão pode ser:

- Uma palavra curta
- Uma pequena expressão (até 2 palavras)
- Ou uma **continuação de no máximo uma frase curta** (1 linha no celular, 12~15 palavras no máximo)

Regras:
- Não repita palavras que já estão antes de <CURSOR>.
- Evite frases longas, genéricas ou formais demais.
- A sugestão deve parecer humana, natural e contextualizada.
- Não complete o texto inteiro, apenas a próxima parte que o usuário poderia digitar agora.
- Responda **somente** com a sugestão. Nada mais.

{}

Texto atual:
\"{} <CURSOR>\"
"#,
		ctx.context_block(),
		ctx.current_input.trim()
	);
	println!("[log] prompt: {}, context: {}", ctx.context_block(), ctx.current_input.trim());
	// let clean_input = format!("{} <CURSOR>", input.trim());
	let agent = ollama::Client::from_url("https://d434-197-235-74-26.ngrok-free.app")
		.completion_model("llama3.2");

	let agent = AgentBuilder::new(agent)
		.temperature(0.5)
		.max_tokens(24) // ~15 palavras
		.build();

	let response = agent.prompt(system_prompt.to_string()).await?.trim().to_owned();
	if response.starts_with('"') {
		let response = response.trim().replace('"', "");
		Ok(response)
	} else {
		Ok(response.trim().to_owned())
	}
}
// use rig::{providers::openai, agent::AgentBuilder};

// let openai = openai::Client::from_env();

// let gpt4o = openai.completion_model("gpt-4o");

// // Configure the agent
// let agent = AgentBuilder::new(model)
//     .preamble("System prompt")
//     .context("Context document 1")
//     .context("Context document 2")
//     .tool(tool1)
//     .tool(tool2)
//     .temperature(0.8)
//     .additional_params(json!({"foo": "bar"}))
//     .build();
