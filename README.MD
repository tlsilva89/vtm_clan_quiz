# 🩸 Qual é a sua Máscara? - VTM Clan Quiz

<div align="center">

**Um quiz interativo para descobrir qual clã de Vampire: The Masquerade melhor representa sua natureza vampírica**

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Slint UI](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

[Características](#-características) • [Instalação](#-instalação) • [Como Usar](#-como-usar) • [Clãs](#-clãs) • [Tecnologias](#-tecnologias)

</div>

## 📖 Sobre

**Qual é a sua Máscara?** é uma aplicação desktop interativa desenvolvida em Rust que guia jogadores por 20 perguntas cuidadosamente elaboradas para determinar qual dos 13 clãs de *Vampire: The Masquerade* mais se alinha com sua personalidade e filosofia.

Cada resposta revela aspectos da sua natureza vampírica, calculando afinidades com diferentes linhagens até revelar seu verdadeiro clã, acompanhado de uma frase de efeito característica.

## ✨ Características

- 🎭 **20 Perguntas Imersivas** – Questões que exploram filosofia, moralidade, poder e identidade.
- 🧛 **13 Clãs Únicos** – Brujah, Gangrel, Malkavian, Nosferatu, Toreador, Tremere, Ventrue, Lasombra, Tzimisce, Hecata, Banu Haqim, Ministry e Caitiff.
- 🎨 **Interface Moderna** – Design dark com tema vampírico, usando Slint UI.
- 💬 **Frases de Efeito** – Cada clã possui uma citação única que captura sua essência.
- 🔄 **Reiniciável** – Refaça o quiz quantas vezes quiser.
- 🖼️ **Background Personalizável** – Suporte para imagem de fundo customizada.
- 📊 **Sistema de Pontuação** – Algoritmo que calcula a afinidade com base nas respostas.

## 🚀 Instalação

### Pré-requisitos

- Rust 1.70+ ([Instalar Rust](https://www.rust-lang.org/tools/install))
- Cargo (incluso com Rust)

### Compilar e Executar

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/vtm_clan_quiz.git
cd vtm_clan_quiz

# Adicione uma imagem de fundo (opcional)
# Coloque sua imagem como 'ui/background.jpg'

# Compile e execute
cargo run --release


### Instalação do Binário

```bash
# Compile o projeto
cargo build --release

# O binário estará em:
./target/release/vtm_clan_quiz
```

## 🎮 Como Usar

1. Execute a aplicação.
2. Leia cada pergunta com atenção.
3. Selecione a resposta que mais ressoa com você.
4. Ao final das 20 perguntas, seu clã será revelado.
5. Clique em "Reiniciar" para refazer o quiz.

## 🧛‍♂️ Clãs

### Camarilla

* **Brujah** – Revolucionários apaixonados e idealistas radicais.
* **Gangrel** – Nômades selvagens conectados à natureza.
* **Malkavian** – Profetas insanos com visões perturbadoras.
* **Nosferatu** – Espiões das sombras, mestres da informação.
* **Toreador** – Artistas obcecados pela beleza eterna.
* **Tremere** – Feiticeiros de sangue, mestres do arcano.
* **Ventrue** – Líderes natos, pilares da ordem vampírica.

### Anarquistas / Independentes

* **Banu Haqim** – Juízes implacáveis e executores da lei.
* **Ministry** – Libertadores que expõem desejos ocultos.
* **Hecata** – Necromantes que caminham entre os mundos.

### Sabbat

* **Lasombra** – Manipuladores das trevas, mestres do poder.
* **Tzimisce** – Escultores de carne, senhores da transformação.

### Sem Clã

* **Caitiff** – Vampiros sem linhagem, livres de correntes.

## 🛠️ Tecnologias

* **[Rust](https://www.rust-lang.org/)** – Linguagem de programação segura e performática.
* **[Slint](https://slint.dev)** – Framework UI declarativo e reativo.
* **HashMap** – Sistema de pontuação baseado em acúmulo de afinidades.

## 📁 Estrutura do Projeto

```
vtm_clan_quiz/
├── src/
│   └── main.rs          # Lógica do quiz e gerenciamento de estado
├── ui/
│   ├── app.slint        # Interface declarativa
│   └── background.jpg   # Imagem de fundo (opcional)
├── build.rs             # Script de build do Slint
├── Cargo.toml           # Dependências do projeto
└── README.md
```

## 🎨 Customização

### Alterar o Background

Substitua `ui/background.jpg` por qualquer imagem de sua preferência. A imagem será ajustada automaticamente com opacidade reduzida.

### Modificar Perguntas

Edite a função `get_quiz_questions()` em `src/main.rs` para adicionar ou modificar perguntas:

```rust
Question { 
    text: "Sua pergunta aqui?", 
    answers: vec![
        ("Resposta 1", Clan::Brujah),
        ("Resposta 2", Clan::Gangrel),
        // ...
    ]
}
```

### Personalizar Cores

Modifique o `ThemeColors` global em `ui/app.slint`:

```slint
global ThemeColors {
    in-out property <color> window-background: #1a1a1aee;
    in-out property <color> border-color: #8b0000;
    // ...
}
```

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Fazer fork do projeto.
2. Criar uma branch para sua feature (`git checkout -b feature/NovaFuncionalidade`).
3. Commitar suas mudanças (`git commit -m 'Adiciona nova funcionalidade'`).
4. Fazer push da branch (`git push origin feature/NovaFuncionalidade`).
5. Abrir um Pull Request.

### Ideias para Contribuição

* Adicionar mais perguntas.
* Implementar sistema de salvamento de resultados.
* Criar modo multiplayer para comparar resultados.
* Adicionar animações de transição.
* Suporte para múltiplos idiomas.
* Integração com Wiki de VTM para mais informações sobre os clãs.

## 📜 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## 🙏 Agradecimentos

* **White Wolf Publishing / Paradox Interactive** – Criadores do universo *Vampire: The Masquerade*.
* **Equipe Slint** – Pelo excelente framework de UI.
* **Comunidade Rust** – Pelo suporte e ferramentas incríveis.

---

<div align="center">

**Feito com 🩸 e Rust**

[🔗 GitHub: tlsilva89/vtm_clan_quiz](https://github.com/tlsilva89/vtm_clan_quiz)

</div>
```

---
