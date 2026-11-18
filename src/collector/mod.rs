pub mod system;

pub use system::SystemCollector;

use crate::error::CollectorError;
use crate::model::Snapshot;

/// Interfaz para recolectar metricas del sistema en un instante dado.
pub trait Collector {
    /// Refresca los datos del sistema y devuelve una nueva medicion.
    fn collect(&mut self) -> Result<Snapshot, CollectorError>;
}
