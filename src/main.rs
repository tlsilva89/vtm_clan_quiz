// Inclui o código gerado pelo build.rs a partir do arquivo .slint
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    // Cria uma instância da nossa janela definida no .slint
    let app = AppWindow::new()?;

    // Executa a aplicação
    app.run()
}