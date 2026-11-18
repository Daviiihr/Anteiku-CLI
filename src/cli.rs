use clap::{Parser, Subcommand};

/// Definición de la interfaz de línea de comandos de Anteiku.
#[derive(Debug, Parser)]
#[command(
    name = "anteiku",
    version,
    about = "Monitor de recursos del sistema para la terminal"
)]
pub struct Cli {
    /// Subcomando a ejecutar.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Subcomandos disponibles en Anteiku.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Toma una única medición del sistema y la muestra por pantalla.
    Snapshot {
        /// Imprime la medición en formato JSON.
        #[arg(long)]
        json: bool,
    },
}
