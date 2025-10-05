
### `README.md`

```markdown
# 🩸 Qual é a sua Máscara? - VTM Clan Quiz

<div align="center">

![Screenshot](screenshot.png)

**Um quiz interativo para descobrir qual clã de Vampire: The Masquerade melhor representa sua natureza vampírica**

[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Slint UI](https://img.shields.io/badge/UI-Slint-blue.svg)](https://slint.dev)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

[Características](#características) • [Instalação](#instalação) • [Como Usar](#como-usar) • [Clãs](#clãs) • [Tecnologias](#tecnologias)

</div>

## 📖 Sobre

**Qual é a sua Máscara?** é uma aplicação desktop interativa desenvolvida em Rust que guia jogadores através de 20 perguntas cuidadosamente elaboradas para determinar qual dos 13 clãs de Vampire: The Masquerade mais se alinha com sua personalidade e filosofia.

Cada resposta revela aspectos da sua natureza vampírica, calculando afinidades com diferentes linhagens até revelar seu clã verdadeiro, acompanhado de uma frase de efeito característica.

## ✨ Características

- 🎭 **20 Perguntas Imersivas** - Questões que exploram filosofia, moralidade, poder e identidade
- 🧛 **13 Clãs Únicos** - Brujah, Gangrel, Malkavian, Nosferatu, Toreador, Tremere, Ventrue, Lasombra, Tzimisce, Hecata, Banu Haqim, Ministry e Caitiff
- 🎨 **Interface Moderna** - Design dark com tema vampírico usando Slint UI
- 💬 **Frases de Efeito** - Cada clã possui uma citação única que captura sua essência
- 🔄 **Reiniciável** - Refaça o quiz quantas vezes quiser
- 🖼️ **Background Customizável** - Suporte para imagem de fundo personalizada
- 📊 **Sistema de Pontuação** - Algoritmo que calcula a afinidade com base nas respostas

## 🚀 Instalação

### Pré-requisitos

- Rust 1.70+ ([Instalar Rust](https://www.rust-lang.org/tools/install))
- Cargo (incluído com Rust)

### Compilar e Executar

```
# Clone o repositório
git clone https://github.com/seu-usuario/vtm_clan_quiz.git
cd vtm_clan_quiz

# Adicione uma imagem de fundo (opcional)
# Coloque sua imagem como 'ui/background.jpg'

# Compile e execute
cargo run --release
```

### Instalação do Binário

```
# Compile o projeto
cargo build --release

# O binário estará em
./target/release/vtm_clan_quiz
```

## 🎮 Como Usar

1. Execute a aplicação
2. Leia cada pergunta cuidadosamente
3. Selecione a resposta que mais ressoa com você
4. Ao final das 20 perguntas, seu clã será revelado
5. Clique em "Reiniciar" para fazer o quiz novamente

## 🧛‍♂️ Clãs

### Camarilla
- **Brujah** - Revolucionários apaixonados e idealistas radicais
- **Gangrel** - Nômades selvagens conectados à natureza
- **Malkavian** - Profetas loucos com visões perturbadoras
- **Nosferatu** - Espiões das sombras, mestres da informação
- **Toreador** - Artistas obcecados pela beleza eterna
- **Tremere** - Feiticeiros de sangue, mestres do arcano
- **Ventrue** - Líderes natos, pilares da ordem vampírica

### Anarquistas/Independentes
- **Banu Haqim** - Juízes implacáveis e executores da lei
- **Ministry** - Libertadores que mostram os desejos ocultos
- **Hecata** - Necromantes que caminham entre mundos

### Sabbat
- **Lasombra** - Manipuladores das trevas, mestres do poder
- **Tzimisce** - Escultores de carne, senhores da transformação

### Sem Clã
- **Caitiff** - Vampiros sem linhagem, livres de correntes

## 🛠️ Tecnologias

- **[Rust](https://www.rust-lang.org/)** - Linguagem de programação segura e performática
- **[Slint](https://slint.dev)** - Framework UI declarativo e reativo
- **HashMap** - Sistema de pontuação baseado em acúmulo de afinidades

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

Substitua `ui/background.jpg` por qualquer imagem de sua preferência. A imagem será automaticamente ajustada com opacidade reduzida.

### Modificar Perguntas

Edite a função `get_quiz_questions()` em `src/main.rs` para adicionar ou modificar perguntas:

```
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

```
global ThemeColors {
    in-out property <color> window-background: #1a1a1aee;
    in-out property <color> border-color: #8b0000;
    // ...
}
```

## 🤝 Contribuindo

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Fazer fork do projeto
2. Criar uma branch para sua feature (`git checkout -b feature/NovaFuncionalidade`)
3. Commit suas mudanças (`git commit -m 'Adiciona nova funcionalidade'`)
4. Push para a branch (`git push origin feature/NovaFuncionalidade`)
5. Abrir um Pull Request

### Ideias para Contribuição

- Adicionar mais perguntas
- Implementar sistema de salvamento de resultados
- Criar modo multiplayer para comparar resultados
- Adicionar animações de transição
- Suporte para múltiplos idiomas
- Integração com Wiki de VTM para mais informações sobre clãs

## 📜 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## 🙏 Agradecimentos

- **White Wolf Publishing / Paradox Interactive** - Criadores do universo Vampire: The Masquerade
- **Slint Team** - Pelo excelente framework UI
- **Comunidade Rust** - Pelo suporte e ferramentas incríveis


Link do Projeto: [https://github.com/tlsilva89/vtm_clan_quiz](https://github.com/tlsilva89/vtm_clan_quiz)

---

<div align="center">

**Feito com 🩸 e Rust**

</div>
```

### Arquivos Adicionais Recomendados

#### `LICENSE`

```
MIT License

Copyright (c) 2025 Thiago

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
