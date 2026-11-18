use thiserror::Error;

/// Errores que pueden ocurrir al recolectar métricas del sistema.
#[derive(Debug, Error)]
pub enum CollectorError {
    /// Error cuando no se puede obtener una métrica obligatoria.
    #[allow(dead_code)] // Se usará en FakeCollector y validaciones de M2
    #[error("no se pudo leer la métrica del sistema: {0}")]
    MetricUnavailable(String),

    /// Error de entrada/salida al leer datos del sistema.
    #[error("error de E/S al leer datos del sistema: {0}")]
    Io(#[from] std::io::Error),

    /// Fallo al inicializar un componente del recolector.
    #[error("fallo al inicializar el recolector '{component}': {reason}")]
    Initialization {
        component: &'static str,
        reason: String,
    },
}
