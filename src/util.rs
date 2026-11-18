/// Formatea una cantidad de bytes en una unidad legible (`B`, `KB`, `MB`, `GB`, `TB`).
/// el numero de bytes recibidos, si es mayor a 1024 lo empieza a dividir por 1024
/// hasta que sea menor a este numero, por cada vez que se divide va pasando kb, gb y asi hasta tb
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    if bytes < 1024 {
        return format!("{bytes} B");
    }

    let mut value = bytes as f64;
    let mut unit_idx = 0;

    while value >= 1024.0 && unit_idx < UNITS.len() - 1 {
        value /= 1024.0;
        unit_idx += 1;
    }

    format!("{value:.1} {}", UNITS[unit_idx])
}

/// Calcula el porcentaje de uso evitando divisiones por cero.
pub fn calc_percent(used: u64, total: u64) -> f32 {
    if total == 0 {
        return 0.0;
    }

    ((used as f64 / total as f64) * 100.0) as f32
}

/// Formatea un porcentaje con un decimal (por ejemplo, `42.5%`).
pub fn format_percent(value: f32) -> String {
    format!("{value:.1}%")
}

/// Formatea una duración en segundos de forma compacta (`45s`, `12m 30s`, `3h 12m`, `2d 4h`).
pub fn format_duration(secs: u64) -> String {
    const MINUTE: u64 = 60;
    const HOUR: u64 = 60 * MINUTE;
    const DAY: u64 = 24 * HOUR;

    if secs < MINUTE {
        format!("{secs}s")
    } else if secs < HOUR {
        let mins = secs / MINUTE;
        let rem_secs = secs % MINUTE;
        format!("{mins}m {rem_secs}s")
    } else if secs < DAY {
        let hours = secs / HOUR;
        let mins = (secs % HOUR) / MINUTE;
        format!("{hours}h {mins}m")
    } else {
        let days = secs / DAY;
        let hours = (secs % DAY) / HOUR;
        format!("{days}d {hours}h")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1_610_612_736), "1.5 GB");
        assert_eq!(format_bytes(1024_u64.pow(4)), "1.0 TB");
    }

    #[test]
    fn test_calc_percent() {
        assert_eq!(calc_percent(0, 0), 0.0);
        assert_eq!(calc_percent(50, 200), 25.0);
        assert_eq!(calc_percent(100, 100), 100.0);
    }

    #[test]
    fn test_format_percent() {
        assert_eq!(format_percent(0.0), "0.0%");
        assert_eq!(format_percent(42.56), "42.6%");
        assert_eq!(format_percent(100.0), "100.0%");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(0), "0s");
        assert_eq!(format_duration(45), "45s");
        assert_eq!(format_duration(90), "1m 30s");
        assert_eq!(format_duration(3 * 3600 + 12 * 60), "3h 12m");
        assert_eq!(format_duration(2 * 86400 + 4 * 3600), "2d 4h");
    }
}
