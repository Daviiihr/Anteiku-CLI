# Anteiku — Documentación del proyecto

> Monitor de recursos del sistema para la terminal, escrito en Rust, con interfaz TUI en vivo, alertas, historial y arte ASCII integrado.
> **Nombre provisional:** `Anteiku` (cambiable; ver sección 15, decisiones).
> **Estado del documento:** v0.1 — diseño inicial, antes de escribir código.

---

## 0. Cómo usar este documento con una IA

Este archivo está pensado para pegarlo (completo o por secciones) al inicio de una conversación con una IA y trabajar el proyecto por etapas.

**Reglas de colaboración (copiar a la IA):**

1. Lee primero las secciones 1 a 4 (contexto, objetivos, alcance, requisitos).
2. Respeta la arquitectura de la sección 6. Si propones cambiarla, explica por qué y espera mi aprobación.
3. Trabajamos **un hito a la vez** (sección 12). No adelantes funciones de hitos posteriores.
4. Explícame los conceptos nuevos de Rust (ownership, borrowing, traits, lifetimes) cuando aparezcan, de forma breve y con ejemplos.
5. Antes de escribir código largo, muestra el plan (archivos que tocas y funciones que agregas).
6. Todo código debe compilar con `cargo build`, pasar `cargo clippy -- -D warnings` y estar formateado con `cargo fmt`.
7. Si una API de una crate puede haber cambiado entre versiones (por ejemplo `sysinfo` o `ratatui`), dímelo y verifica contra la documentación de la versión que usamos.
8. Al terminar cada sesión, actualiza el **registro de sesiones** (sección 16) y marca las casillas del hito.

**Plantilla de prompt para cada sesión:**

```
Contexto: proyecto "Anteiku" (ver documento adjunto).
Hito actual: M3 — TUI con gráficos en vivo.
Ya hecho: M0, M1, M2 (todas las casillas marcadas).
Hoy quiero: implementar el panel de CPU con historial de 60 muestras.
Restricciones: respetar la arquitectura de la sección 6; sin dependencias nuevas sin avisarme.
Entrega: plan breve + código + cómo probarlo + qué aprendí/qué concepto de Rust usamos.
```

---

## 1. Contexto

### 1.1 Autor y propósito

- **Autor:** David (GitHub: `daviihr`). Estudiante de informática.
- **Objetivo personal:** construir un proyecto de **portafolio** en GitHub que sea divertido, útil, técnico y visualmente atractivo, y aprender Rust a fondo en el proceso.
- **Idioma:** documentación y comunicación en español. El README público tendrá versión en español e inglés.
- **Entorno de desarrollo:** Arch Linux (plataforma principal de pruebas). Se desea que funcione también en macOS y Windows, pero Linux es la prioridad.
- **Stack previo del autor:** Python, JavaScript/TypeScript, Node.js, C#, Java, Spring Boot, PostgreSQL, Docker, HTML/CSS. **Nivel en Rust:** principiante (ajustar según avance).

### 1.2 Qué es Anteiku

Un programa de terminal que muestra, en tiempo real, el estado de la computadora (CPU, memoria, disco, red, procesos, temperaturas), con un panel visual tipo `htop` y una esquina con **arte ASCII** estilo `fastfetch`. Además guarda un historial, avisa cuando algo se sale de lo normal y puede exportar datos.

### 1.3 Por qué este proyecto

- Es visual (se ve bien en un GIF del README).
- Cubre muchos conceptos: concurrencia, manejo de errores, parsing de argumentos, TUI, persistencia, testing, CI, empaquetado.
- Es escalable: hay una versión mínima rápida y muchas extensiones.
- Es útil: el propio autor lo usaría.

---

## 2. Objetivos

### 2.1 Objetivos principales

1. Mostrar métricas del sistema en vivo en una TUI fluida.
2. Incluir arte ASCII predeterminado dentro del binario, sin que el usuario deba configurar nada.
3. Ofrecer subcomandos útiles además de la TUI (`snapshot`, `history`, `export`).
4. Guardar historial en SQLite y emitir alertas por umbrales.
5. Tener calidad de portafolio: tests, CI, README con GIF, releases con binarios.

### 2.2 No objetivos (por ahora)

- No reemplazar a `htop`, `btop` o `glances`; es un proyecto de aprendizaje y portafolio.
- No hacer monitoreo remoto/multi-equipo en la v1.
- No soportar imágenes reales en terminal (Kitty, Sixel); solo arte ASCII.
- No ser una herramienta de seguridad ni de administración de procesos avanzada.

### 2.3 Criterios de éxito

- Instalable con `cargo install` y mediante binarios de GitHub Releases.
- Se ejecuta en Linux sin configuración previa y muestra datos correctos.
- Consumo propio bajo (idealmente menos de 2% de CPU en reposo con refresco de 1 s).
- Cobertura razonable de tests en la lógica (alertas, cálculos, formateo, configuración).
- README con GIF, instrucciones claras y badges de CI.

---

## 3. Alcance por versiones

| Versión | Contenido |
|---|---|
| **v0.1 (MVP)** | CLI con snapshot en texto: CPU, RAM, disco, red, top procesos. Argumentos con `clap`. |
| **v0.2** | TUI en vivo con `ratatui`: gráficos de CPU/RAM/red, tabla de procesos, atajos de teclado. |
| **v0.3** | Arte ASCII integrado, panel de información tipo fastfetch, temas de color. |
| **v0.4** | Archivo de configuración TOML, alertas por umbral y notificaciones de escritorio. |
| **v0.5** | Historial en SQLite, subcomandos `history` y `export` (CSV/JSON). |
| **v1.0** | Pulido, tests, CI, releases, README bilingüe, publicación en crates.io. |
| **Futuro** | Modo servidor con `/metrics` (Prometheus), dashboard web, monitoreo de contenedores Docker, GPU, árbol de procesos. |

---

## 4. Requisitos

### 4.1 Requisitos funcionales

| ID | Requisito | Prioridad |
|---|---|---|
| RF-01 | Mostrar uso de CPU total y por núcleo. | Alta |
| RF-02 | Mostrar memoria RAM y swap (usada, total, porcentaje). | Alta |
| RF-03 | Mostrar uso de disco por punto de montaje. | Alta |
| RF-04 | Mostrar tráfico de red (subida/bajada por segundo) por interfaz. | Alta |
| RF-05 | Listar procesos con PID, nombre, CPU %, memoria; ordenable y filtrable. | Alta |
| RF-06 | Mostrar temperaturas si el sistema las expone. | Media |
| RF-07 | Mostrar información del sistema: SO, kernel, host, uptime, usuario. | Media |
| RF-08 | Mostrar arte ASCII seleccionado automáticamente. | Alta |
| RF-09 | Permitir ocultar el arte con `--no-art`. | Media |
| RF-10 | Cargar configuración desde `config.toml` con valores por defecto sensatos. | Media |
| RF-11 | Evaluar reglas de alerta y notificar (en pantalla y escritorio). | Media |
| RF-12 | Guardar muestras en SQLite en intervalos configurables. | Media |
| RF-13 | Consultar historial por rango de tiempo y exportar a CSV/JSON. | Media |
| RF-14 | Terminar procesos desde la TUI con confirmación (opcional). | Baja |
| RF-15 | Salida `--json` en `snapshot` para uso en scripts. | Media |

### 4.2 Requisitos no funcionales

| ID | Requisito |
|---|---|
| RNF-01 | **Rendimiento:** bajo consumo propio; la TUI no debe parpadear. |
| RNF-02 | **Robustez:** nunca `panic` por datos faltantes del sistema; degradar con elegancia (por ejemplo, sin temperaturas). |
| RNF-03 | **Portabilidad:** compila en Linux, macOS y Windows (CI lo verifica). |
| RNF-04 | **Restaurar la terminal:** al salir o ante un fallo, se debe restaurar el modo normal de la terminal. |
| RNF-05 | **Mantenibilidad:** módulos pequeños, tipos explícitos, `clippy` sin advertencias. |
| RNF-06 | **Testabilidad:** la lógica de negocio no depende de la terminal ni del sistema real. |
| RNF-07 | **Accesibilidad básica:** modo sin colores (`NO_COLOR`) y arte opcional. |

---

## 5. Diseño de la experiencia (UX)

### 5.1 Interfaz de línea de comandos

```text
anteiku                         # abre la TUI en vivo (comportamiento por defecto)
anteiku snapshot [--json]       # una sola medición y termina
anteiku top [--sort cpu|mem] [--limit 10]
anteiku history --last 24h [--metric cpu|mem|net]
anteiku export --format csv|json --last 7d --output datos.csv
anteiku config path             # muestra la ruta del config
anteiku config init             # crea un config.toml con valores por defecto
anteiku completions bash|zsh|fish # genera autocompletado para la shell (M7)
anteiku --no-art                # oculta el arte ASCII
anteiku --demo                  # ejecuta la TUI con datos simulados (FakeCollector)
anteiku --interval 500          # refresco en milisegundos
anteiku --version | --help
```

**Códigos de salida:** `0` éxito; `1` error de uso o configuración; `2` error del sistema (por ejemplo, base de datos inaccesible).

### 5.2 Diseño de la pantalla (TUI)

```text
┌─ Anteiku ─────────────────────────────────────────────────────────────┐
│ ┌──────────────┐  ┌─ CPU ───────────────────────────────────────┐   │
│ │              │  │ ▁▂▃▅▆▇▅▃▂▁▂▃▅▆  total: 42%                  │   │
│ │  (arte       │  │ core0 ██████░░░░ 61%   core1 ███░░░░░░░ 30% │   │
│ │   ASCII)     │  └─────────────────────────────────────────────┘   │
│ │              │  ┌─ Memoria ───────┐ ┌─ Red ──────────────────┐    │
│ └──────────────┘  │ RAM  6.2/16 GB  │ │ ↓ 1.2 MB/s  ↑ 120 KB/s │    │
│ Usuario: david    │ ███████░░░ 39%  │ │ ▁▂▂▃▅▇▅▃▂▁▂▃           │    │
│ SO: Arch Linux    │ Swap 0.0/8 GB   │ └────────────────────────┘    │
│ Kernel: 6.x       └─────────────────┘                               │
│ Uptime: 3h 12m    ┌─ Procesos ─────────────────────────────────┐    │
│                   │ PID    NOMBRE        CPU%    MEM            │    │
│ ┌─ Disco ───────┐ │ 1234   firefox       18.2    1.2 GB         │    │
│ │ /  60%        │ │ 2210   code          9.4     800 MB         │    │
│ └───────────────┘ └─────────────────────────────────────────────┘    │
│ [q] salir  [Tab] panel  [s] ordenar  [/] buscar  [?] ayuda          │
└─────────────────────────────────────────────────────────────────────┘
```

**Comportamiento del diseño:**

- **Terminal ancha (≥ 100 columnas):** arte a la izquierda, paneles a la derecha.
- **Terminal media (60–99):** arte oculto, solo paneles.
- **Terminal angosta (< 60):** una sola columna con CPU, RAM y procesos.
- Los colores cambian según la carga: verde (< 60%), amarillo (60–85%), rojo (> 85%).

### 5.3 Atajos de teclado

| Tecla | Acción |
|---|---|
| `q` / `Ctrl+C` | Salir |
| `Tab` / `Shift+Tab` | Cambiar panel enfocado |
| `↑` `↓` / `j` `k` | Mover selección en la tabla de procesos |
| `s` | Cambiar criterio de orden (CPU, memoria, PID, nombre) |
| `/` | Buscar proceso por nombre |
| `p` | Pausar/reanudar actualización |
| `+` / `-` | Aumentar/disminuir intervalo de refresco |
| `a` | Mostrar/ocultar arte |
| `?` | Ayuda |
| `K` | Terminar proceso seleccionado (con confirmación) |

### 5.4 Sistema de arte ASCII

- Los dibujos viven en `assets/art/*.txt` y se incrustan en el binario con `include_str!`.
- **El usuario no elige:** la selección es automática según una regla (ver tabla).
- Cada dibujo mide como máximo **34 columnas × 18 filas**, con líneas del mismo ancho medido en **caracteres/columnas visuales** (`.chars().count()` o `unicode-width`, nunca `.len()` en bytes UTF-8).
- **Identidad temática:** aprovechando el nombre *Anteiku*, el arte principal puede incluir una **taza de café** (humo tranquilo en estado normal; hirviendo o derramándose cuando la CPU supera el 85%), además de opciones como gato o cohete.
- Puede tener **variantes por estado** (por ejemplo, tranquilo y estresado).

| Regla | Efecto |
|---|---|
| Estado de CPU alto (> 85% sostenido) | Muestra la variante "estresada". |
| Hora del día | Variante de día o de noche (opcional, v1.x). |
| Sistema operativo | Logo del SO detectado (Linux/Windows/macOS) como respaldo. |
| Sin regla aplicable | Aleatoria entre los dibujos predeterminados al iniciar. |

- **Coloreado:** cada dibujo puede tener un archivo de máscara de color opcional, o se colorea por rangos de líneas. Decisión inicial: color único por tema (simple), refinar después.
- **Animación opcional:** alternar 2–3 cuadros por tick.
- Los dibujos se crean **una vez** con herramientas externas (`jp2a`, `chafa`, `ascii-image-converter`) o a mano, y se revisan manualmente.

---

## 6. Arquitectura

### 6.1 Visión general

Aplicación **monolítica en un solo crate binario** (`anteiku` en minúsculas por convención de Cargo/Unix; con módulos internos, y opcionalmente una librería `anteiku_core` si se quiere separar la lógica). Arquitectura en capas, con dependencias en una sola dirección:

```text
                ┌──────────┐
   argumentos → │   cli    │  (clap: parsea comandos y flags)
                └────┬─────┘
                     ▼
                ┌──────────┐      ┌────────────┐
                │   app    │ ───▶ │   config   │
                │ (estado, │      └────────────┘
                │  bucle)  │      ┌────────────┐
                └──┬─┬─┬───┘ ───▶ │   alerts   │
                   │ │ │          └────────────┘
        ┌──────────┘ │ └──────────┐
        ▼            ▼            ▼
  ┌───────────┐ ┌──────────┐ ┌──────────┐
  │ collector │ │    ui    │ │ storage  │
  │ (sysinfo) │ │(ratatui) │ │ (SQLite) │
  └───────────┘ └────┬─────┘ └──────────┘
                     ▼
                ┌──────────┐
                │   art    │
                └──────────┘
```

**Principio clave:** el módulo `collector` está detrás de un **trait**, de modo que en los tests —y en el modo `--demo` de la TUI— se reemplaza por un `FakeCollector` con datos controlados. La `ui` solo recibe datos ya calculados y no sabe de dónde vienen.

### 6.2 Estructura de carpetas

```text
Anteiku/
├── Cargo.toml
├── README.md
├── README.en.md
├── LICENSE
├── CHANGELOG.md
├── docs/
│   ├── arquitectura.md
│   ├── decisiones.md
│   └── img/                  # capturas y GIFs
├── assets/
│   └── art/
│       ├── logo.txt
│       ├── taza.txt
│       ├── taza_estres.txt
│       ├── gato.txt
│       ├── gato_estres.txt
│       └── cohete.txt
├── src/
│   ├── main.rs               # punto de entrada, manejo de errores de alto nivel
│   ├── cli.rs                # definición de comandos con clap
│   ├── error.rs              # tipos de error propios
│   ├── config.rs             # carga, validación y valores por defecto
│   ├── model.rs              # Snapshot, CpuInfo, MemInfo, etc.
│   ├── collector/
│   │   ├── mod.rs            # trait Collector
│   │   ├── system.rs         # implementación con sysinfo
│   │   └── fake.rs           # implementación para tests
│   ├── app/
│   │   ├── mod.rs            # estado de la aplicación y bucle principal
│   │   ├── history.rs        # buffers circulares para gráficos
│   │   └── input.rs          # manejo de teclas → acciones
│   ├── alerts.rs             # reglas, evaluación y estado de alertas
│   ├── storage/
│   │   ├── mod.rs
│   │   └── sqlite.rs         # esquema, inserción, consultas
│   ├── ui/
│   │   ├── mod.rs            # composición del layout
│   │   ├── theme.rs          # colores y estilos
│   │   └── widgets/
│   │       ├── cpu.rs
│   │       ├── memory.rs
│   │       ├── network.rs
│   │       ├── disks.rs
│   │       ├── processes.rs
│   │       └── sysinfo_panel.rs
│   ├── art.rs                # selección y carga de dibujos ASCII
│   └── util.rs               # formateo de bytes, duraciones, porcentajes
├── tests/
│   ├── cli.rs                # tests de integración con assert_cmd
│   └── storage.rs
└── .github/
    └── workflows/
        ├── ci.yml
        └── release.yml
```

### 6.3 Modelo de datos (en memoria)

```rust
pub struct Snapshot {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub cpu: CpuInfo,
    pub memory: MemInfo,
    pub disks: Vec<DiskInfo>,
    pub networks: Vec<NetInfo>,
    pub processes: Vec<ProcInfo>,
    pub temperatures: Vec<TempInfo>,   // puede venir vacío
    pub system: SystemInfo,
}

pub struct CpuInfo {
    pub total_percent: f32,
    pub per_core: Vec<f32>,
    pub frequency_mhz: Option<u64>,
}

pub struct MemInfo {
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
}

pub struct DiskInfo {
    pub mount_point: String,
    pub used_bytes: u64,
    pub total_bytes: u64,
}

pub struct NetInfo {
    pub interface: String,
    pub rx_bytes_per_sec: u64,
    pub tx_bytes_per_sec: u64,
}

pub struct ProcInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
}

pub struct TempInfo {
    pub label: String,
    pub celsius: f32,
}

pub struct SystemInfo {
    pub os_name: String,
    pub kernel: String,
    pub hostname: String,
    pub username: String,
    pub uptime_secs: u64,
}
```

### 6.4 Interfaces principales (traits)

```rust
pub trait Collector {
    /// Refresca los datos y devuelve una nueva medición.
    fn collect(&mut self) -> Result<Snapshot, CollectError>;
}
```

```rust
pub trait Store {
    fn insert(&mut self, snap: &Snapshot) -> Result<(), StoreError>;
    fn query_range(&self, from: DateTime<Utc>, to: DateTime<Utc>)
        -> Result<Vec<SampleRow>, StoreError>;
}
```

### 6.5 Flujo de la aplicación (modo TUI)

```text
1. main        → parsea argumentos (cli) y carga configuración (config)
2. main        → crea Collector (SystemCollector o FakeCollector con --demo), Store (opcional), Alerts, App
3. app::run    → inicializa terminal con TerminalGuard (modo raw, pantalla alterna)
4. bucle (con next_tick: Instant):
     a. timeout = next_tick.saturating_duration_since(Instant::now())
     b. esperar evento de teclado con crossterm::event::poll(timeout)
     c. si hay tecla → input.rs la traduce a una Acción, actualiza el estado y redibuja al instante con el último Snapshot
     d. si Instant::now() >= next_tick:
          - collector.collect() → Snapshot (sin bloquear por pulsaciones de tecla)
          - history.push(snapshot)         (buffers de 60–120 muestras)
          - alerts.evaluate(&snapshot)     (puede producir eventos)
          - storage.insert(...) cada N ticks (según configuración)
          - next_tick += intervalo_refresco
          - ui::draw(frame, &estado)       (redibuja)
5. al salir    → TerminalGuard (Drop) restaura la terminal siempre, incluso ante error o pánico
```

> **Nota sobre `sysinfo`:** en `sysinfo`, la CPU y las tasas de red se calculan por diferencia entre dos lecturas. Al inicializar `SystemCollector` (y en los comandos puntuales `snapshot` y `top`), realizar una primera lectura de CPU, esperar `sysinfo::MINIMUM_CPU_UPDATE_INTERVAL` (~200 ms) y tomar la segunda medición para no reportar `0.0%`. Además, usar refrescos selectivos (`RefreshKind` / `ProcessRefreshKind`) en lugar de `refresh_all()` en cada tick para mantener el consumo propio por debajo del 2%.

### 6.6 Concurrencia

- **Fase inicial:** un solo hilo. El bucle desacopla la respuesta al teclado (redibujado inmediato con el último `Snapshot`) del tick de recolección (`next_tick: Instant`), usando `crossterm::event::poll(timeout)`. Es simple, fluido y suficiente.
- **Fase posterior (si la UI se siente lenta):** mover `collector` a un hilo propio que envía `Snapshot` por un canal (`std::sync::mpsc` o `crossbeam-channel`), y otro hilo para escritura en SQLite. La UI solo recibe y dibuja.
- **Regla:** no introducir `async`/`tokio` salvo que se agregue el modo servidor (futuro).

### 6.7 Manejo de errores

- `anyhow` en `main.rs` y subcomandos (errores con contexto para el usuario).
- `thiserror` en módulos (`CollectError`, `StoreError`, `ConfigError`) para errores tipados.
- Datos faltantes (por ejemplo sin sensores) se modelan con `Option` o `Vec` vacío, nunca con `unwrap`.
- Instalar un **hook de pánico** y usar un guard RAII (`TerminalGuard` implementando `Drop`) que restaure la terminal antes de imprimir el error o al salir por cualquier rama (`?`).
- Los mensajes de error al usuario deben ser claros y accionables (qué pasó y qué hacer).

### 6.8 Alertas

**Reglas (en configuración):**

```toml
[[alerts]]
name = "CPU alta"
metric = "cpu_total"
operator = ">"
threshold = 90.0
duration_secs = 30        # debe mantenerse este tiempo
cooldown_secs = 300       # no repetir antes de este tiempo

[[alerts]]
name = "Disco casi lleno"
metric = "disk_used_percent"
operator = ">"
threshold = 90.0
duration_secs = 0
cooldown_secs = 3600
```

**Lógica (`alerts.rs`):**

- Cada regla mantiene un estado: `Normal`, `Pendiente(desde)`, `Activa(desde)`, `Enfriamiento(hasta)`.
- Se evalúa con cada `Snapshot`. Al pasar a `Activa`, se emite un `AlertEvent`.
- Los eventos se muestran en un panel de la TUI, se guardan en la tabla `alert_log` y, opcionalmente, se envían como notificación de escritorio.
- Es **lógica pura** (recibe snapshot + hora, devuelve eventos), por lo que se prueba fácilmente.

### 6.9 Persistencia (SQLite)

```sql
CREATE TABLE IF NOT EXISTS samples (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    ts            INTEGER NOT NULL,          -- epoch en segundos (UTC)
    cpu_total     REAL    NOT NULL,
    mem_used      INTEGER NOT NULL,
    mem_total     INTEGER NOT NULL,
    swap_used     INTEGER NOT NULL,
    net_rx_bps    INTEGER NOT NULL,
    net_tx_bps    INTEGER NOT NULL,
    disk_used_pct REAL
);
CREATE INDEX IF NOT EXISTS idx_samples_ts ON samples(ts);

CREATE TABLE IF NOT EXISTS alert_log (
    id        INTEGER PRIMARY KEY AUTOINCREMENT,
    ts        INTEGER NOT NULL,
    rule_name TEXT    NOT NULL,
    value     REAL    NOT NULL,
    message   TEXT    NOT NULL
);

CREATE TABLE IF NOT EXISTS meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);   -- guarda, por ejemplo, schema_version
```

**Política de retención:** borrar muestras con más de N días (configurable, por defecto 30). Considerar **downsampling** (promedios por minuto) para historiales largos.
**Concurrencia SQLite:** activar `PRAGMA journal_mode = WAL;` al abrir la base de datos para permitir lecturas simultáneas (`anteiku history`, `anteiku export`) mientras la TUI escribe sin bloqueos (`SQLITE_BUSY`).
**Ubicación:** carpeta de datos del usuario obtenida con la crate `directories`.

### 6.10 Configuración

Ubicaciones (con `directories`): `~/.config/anteiku/config.toml` en Linux. Si no existe, se usan valores por defecto y no se crea nada hasta `anteiku config init`.

```toml
# config.toml (valores por defecto)
[general]
refresh_ms = 1000
history_points = 60

[ui]
theme = "default"        # default | mono | solarized (futuro)
show_art = true
show_cores = true

[processes]
default_sort = "cpu"     # cpu | mem | pid | name
limit = 20

[storage]
enabled = true
sample_every_secs = 10
retention_days = 30

[notifications]
desktop = true
```

**Validación:** rangos válidos (por ejemplo `refresh_ms` ≥ 200) y mensajes de error con la línea del problema. Los campos desconocidos generan advertencia, no error.

---

## 7. Dependencias

> Usar la **última versión estable** de cada crate al crear el proyecto y fijarla en `Cargo.lock`. Verificar la documentación de la versión usada, porque algunas APIs cambian entre versiones mayores.

| Crate | Uso | Hito |
|---|---|---|
| `sysinfo` | CPU, RAM, disco, red, procesos, temperaturas | M1 |
| `clap` (feature `derive`) | Argumentos y subcomandos | M1 |
| `anyhow` | Errores con contexto en `main` | M1 |
| `thiserror` | Errores tipados en módulos | M1 |
| `serde`, `serde_json` | Serialización (`--json`, exportación) | M1/M6 |
| `chrono` | Fechas y horas | M1 |
| `ratatui` | Widgets y layout de la TUI | M3 |
| `crossterm` | Terminal en modo raw, teclado, eventos | M3 |
| `rand` | Selección aleatoria del arte | M4 |
| `toml` | Leer configuración | M5 |
| `directories` | Rutas de config y datos por sistema | M5 |
| `notify-rust` | Notificaciones de escritorio | M5 |
| `rusqlite` (feature `bundled`) | Base de datos SQLite | M6 |
| `csv` | Exportación a CSV | M6 |
| `tracing`, `tracing-subscriber` | Registros de depuración a archivo | M7 |
| `clap_complete` | Generación de scripts de autocompletado (`completions`) | M7 |

**Dependencias de desarrollo:** `assert_cmd`, `predicates`, `tempfile`, `insta` (tests de snapshot, opcional).

---

## 8. Estándares de código

- **Formato:** `cargo fmt` (configuración por defecto).
- **Lints:** `cargo clippy --all-targets -- -D warnings`.
- **Nombres:** `snake_case` para funciones y variables, `PascalCase` para tipos. Identificadores de código en inglés; textos al usuario y comentarios en español.
- **Documentación:** comentarios `///` en todo lo público.
- **Sin `unwrap()`/`expect()`** en código de producción, salvo invariantes demostrables (con comentario).
- **Funciones pequeñas** y módulos con una sola responsabilidad.
- **Commits:** formato *Conventional Commits* (`feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`).
- **Ramas:** `main` siempre compila; trabajo en ramas `feat/...` con PRs, incluso si trabajo solo (buen historial para portafolio).
- **Versionado:** SemVer; `CHANGELOG.md` siguiendo *Keep a Changelog*.

---

## 9. Estrategia de pruebas

| Nivel | Qué se prueba | Herramienta |
|---|---|---|
| **Unitarias** | Formateo de bytes/duración, cálculo de porcentajes, motor de alertas, orden y filtro de procesos, parseo de configuración, selección de arte | `cargo test` |
| **Con falsos** | Aplicación completa usando `FakeCollector` (sin depender del sistema real) | `cargo test` |
| **UI** | Dibujar sobre `TestBackend` de ratatui y comparar el buffer con lo esperado | `ratatui::backend::TestBackend`, `insta` |
| **Integración CLI** | `anteiku snapshot --json` produce JSON válido; códigos de salida; `--help` | `assert_cmd` |
| **Almacenamiento** | Inserción y consulta en SQLite en memoria o archivo temporal | `tempfile` |
| **Manuales** | Redimensionar terminal, salir con `Ctrl+C`, terminal sin color, SSH, `tmux` | Lista de verificación |

**Regla:** toda lógica nueva que no dependa de la terminal debe llegar con al menos un test.

---

## 10. CI/CD y publicación

**`ci.yml` (en cada push y PR):**

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test --all
```

**Release:** al crear una etiqueta `vX.Y.Z`, un workflow compila binarios para Linux, macOS y Windows y los adjunta a GitHub Releases (evaluar `cargo-dist` para automatizarlo). Publicar en **crates.io** en v1.0.

**Docker (opcional, para el portafolio):** `Dockerfile` multi-etapa que compila el binario y genera una imagen mínima; útil si en el futuro se agrega el modo servidor.

---

## 11. Plan del README (portada del proyecto)

1. Nombre, lema y badges (CI, versión, licencia).
2. **GIF o captura** de la TUI (lo primero que se ve).
3. Características (lista corta).
4. Instalación (`cargo install anteiku`, binarios, compilar desde el código).
5. Uso rápido (3–5 comandos).
6. Atajos de teclado.
7. Configuración (ejemplo de `config.toml`).
8. Capturas del arte y los temas.
9. Arquitectura resumida (diagrama simple) y enlace a `docs/`.
10. Hoja de ruta (roadmap).
11. Cómo contribuir, licencia (MIT) y agradecimientos.
12. Versión en inglés (`README.en.md`).

---

## 12. Plan de trabajo por hitos

> Marcar las casillas a medida que se completan. Cada hito debe terminar con el proyecto compilando y con tests en verde.

### M0 — Preparación

- [x] Instalar Rust con `rustup` y verificar `cargo --version`.
- [x] Crear repositorio en GitHub (`Anteiku-CLI`) con licencia MIT y `.gitignore` de Rust.
- [x] `cargo init --name anteiku` (crate en minúsculas); primer commit con README mínimo.
- [x] Configurar `cargo fmt` y `cargo clippy`; agregar `ci.yml` básico.
- **Aprendizaje:** estructura de un proyecto Cargo, `Cargo.toml`, `cargo run/build/test`.
- **Criterio de aceptación:** el CI se ejecuta en verde en GitHub.

### M1 — Snapshot en texto (MVP)

- [x] Definir `model.rs` con `Snapshot` y structs asociados.
- [x] Trait `Collector` e implementación con `sysinfo` (refrescos selectivos y espera inicial `MINIMUM_CPU_UPDATE_INTERVAL` para evitar `0.0%`).
- [x] `util.rs`: formateo de bytes (`1.2 GB`), porcentajes y duraciones, con tests.
- [x] CLI con `clap`: `anteiku snapshot` y `anteiku snapshot --json`.
- [x] Mostrar CPU, RAM, disco, red y top 5 procesos en texto.
- **Aprendizaje:** structs, enums, `Option`/`Result`, traits, módulos, `serde`.
- **Criterio de aceptación:** `anteiku snapshot` muestra datos correctos comparados con `htop`/`free`/`df`; `--json` es válido.

### M2 — Procesos, errores y pruebas

- [ ] Subcomando `anteiku top --sort --limit`.
- [ ] Tipos de error con `thiserror`; `anyhow` en `main`.
- [ ] `FakeCollector` y tests de ordenamiento/filtrado.
- [ ] Test de integración CLI con `assert_cmd`.
- **Aprendizaje:** ownership y borrowing en colecciones, iteradores, manejo de errores, testing.
- **Criterio de aceptación:** `cargo test` en verde; ningún `unwrap` en producción.

### M3 — TUI en vivo

- [ ] Inicializar/restaurar terminal con `crossterm` mediante `TerminalGuard` (`Drop`) + hook de pánico.
- [ ] Bucle de eventos con `next_tick: Instant` y `poll(timeout)` (redibujado inmediato al pulsar teclas).
- [ ] `history.rs`: buffer circular para gráficos.
- [ ] Widgets: CPU (sparkline + barras por núcleo), memoria, red, procesos.
- [ ] Layout responsivo (3 modos de ancho).
- [ ] Atajos: salir, cambiar panel, ordenar, pausar, y flag `--demo` conectado a `FakeCollector`.
- **Aprendizaje:** bucle de aplicación, patrón RAII (`Drop`), estado mutable, `ratatui` (layouts, widgets), lifetimes básicos.
- **Criterio de aceptación:** la TUI se actualiza cada segundo sin parpadeo, responde al instante al teclado y al salir la terminal queda intacta.

### M4 — Arte ASCII y panel de información

- [ ] Crear 3–4 dibujos en `assets/art/` incluyendo taza de café temática (`taza.txt`, `taza_estres.txt`), con ancho uniforme máx. 34×18 medido en caracteres/columnas.
- [ ] `art.rs` con `include_str!` y selección automática (aleatoria + variante por CPU).
- [ ] Panel de información del sistema (usuario, SO, kernel, uptime).
- [ ] Temas de color (`theme.rs`) y soporte `NO_COLOR`.
- [ ] Flag `--no-art` y ocultar arte en terminales angostas.
- **Aprendizaje:** recursos incrustados, UTF-8 vs. caracteres en Rust, `rand`, estilos de `ratatui`.
- **Criterio de aceptación:** el arte se ve alineado en distintas terminales y se oculta cuando no cabe.

### M5 — Configuración y alertas

- [ ] `config.rs` con `serde` + `toml`, valores por defecto y validación.
- [ ] `anteiku config init` y `anteiku config path`.
- [ ] `alerts.rs` con máquina de estados y tests (duración, enfriamiento).
- [ ] Panel de alertas en la TUI y notificaciones de escritorio.
- **Aprendizaje:** `serde` avanzado, enums con datos, máquinas de estado, pruebas de lógica pura.
- **Criterio de aceptación:** una alerta de CPU se dispara y no se repite durante el enfriamiento (con test).

### M6 — Historial y exportación

- [ ] `storage/sqlite.rs` con migración inicial, `schema_version` y `PRAGMA journal_mode = WAL`.
- [ ] Guardado periódico de muestras y política de retención.
- [ ] `anteiku history --last 24h` con una mini gráfica en texto.
- [ ] `anteiku export --format csv|json`.
- **Aprendizaje:** SQLite desde Rust, SQL, parámetros preparados, manejo de tiempo.
- **Criterio de aceptación:** los datos persisten entre ejecuciones y la exportación se abre en una hoja de cálculo.

### M7 — Pulido y lanzamiento v1.0

- [ ] Revisión de rendimiento (consumo propio de CPU y memoria).
- [ ] Registro de depuración opcional con `tracing` a archivo.
- [ ] Subcomando `anteiku completions bash|zsh|fish` con `clap_complete`.
- [ ] Documentación `docs/arquitectura.md` y `docs/decisiones.md`.
- [ ] README bilingüe con GIF grabado usando `--demo` (`asciinema` + `agg`, o `vhs`).
- [ ] Workflow de release con binarios; publicar en crates.io.
- [ ] Fijar el repositorio en el perfil de GitHub y agregarlo al README del perfil.
- **Criterio de aceptación:** un usuario nuevo instala y usa el programa solo con el README.

### Más allá de v1.0 (ideas)

- Modo servidor con endpoint `/metrics` (Prometheus) y dashboard web con `axum`.
- `docker-compose` con Anteiku + Prometheus + Grafana.
- Monitoreo de contenedores Docker (`bollard`).
- Árbol de procesos, conexiones de red, GPU NVIDIA, batería.
- Detección de anomalías contra una línea base.
- Temas adicionales y animación del arte.

---

## 13. Riesgos y mitigaciones

| Riesgo | Probabilidad | Mitigación |
|---|---|---|
| La curva de Rust frena el avance. | Alta | Hitos pequeños; priorizar claridad sobre optimización; pedir explicaciones de cada concepto. |
| Cambios de API en `sysinfo`/`ratatui` rompen el código generado por la IA. | Media | Fijar versiones; verificar la documentación de la versión exacta. |
| Diferencias entre sistemas (temperaturas, permisos, discos). | Alta | Modelar con `Option`; probar en CI en 3 sistemas; degradar sin fallar. |
| Parpadeo o lentitud de la TUI. | Media | Dibujar solo lo necesario; considerar hilo separado de recolección. |
| Alcance excesivo (nunca se termina). | Alta | Respetar las versiones; todo lo de "futuro" queda fuera hasta v1.0. |
| Arte ASCII desalineado o con licencia dudosa. | Media | Tests de ancho uniforme; crear arte propio o con licencia compatible. |
| Terminal no restaurada tras un fallo. | Media | Hook de pánico y patrón de limpieza (guard con `Drop`). |

---

## 14. Glosario

- **TUI:** interfaz de usuario en terminal con paneles y gráficos.
- **CLI:** programa que se controla con comandos y argumentos.
- **Snapshot:** una medición completa del sistema en un instante.
- **Tick:** cada intervalo de actualización de la aplicación.
- **Buffer circular:** estructura de tamaño fijo que descarta lo más viejo al llenarse (para gráficos).
- **Umbral:** valor límite que activa una alerta.
- **Cooldown (enfriamiento):** tiempo mínimo entre dos alertas de la misma regla.
- **ASCII art:** dibujo hecho con caracteres de texto.
- **ADR:** registro de decisión de arquitectura.

---

## 15. Registro de decisiones (ADR resumido)

| # | Decisión | Motivo | Estado |
|---|---|---|---|
| 1 | Lenguaje: Rust | Aprendizaje profundo y portafolio técnico. | Aprobada |
| 2 | Tipo: CLI/TUI de monitoreo de recursos | Visual, útil y escalable. | Aprobada |
| 3 | Arte: solo ASCII, predeterminado e incluido en el binario | Funciona en cualquier terminal y simplifica el código; el usuario no elige. | Aprobada |
| 4 | TUI con `ratatui` + `crossterm` | Estándar del ecosistema, buena documentación. | Propuesta |
| 5 | Un solo hilo al inicio, hilos/canales solo si hace falta | Simplicidad para aprender; evitar `async`. | Propuesta |
| 6 | Persistencia con SQLite (`rusqlite`, feature `bundled`) | Sin servidor, portable, ideal para historial local. | Propuesta |
| 7 | Configuración en TOML | Es el formato habitual en el ecosistema Rust. | Propuesta |
| 8 | Nombre `Anteiku` (crate/binario `anteiku`) | Corto y descriptivo; minúsculas por convención de Cargo/Unix. Verificar disponibilidad en crates.io. | Pendiente |
| 9 | Licencia MIT | Permisiva y común en portafolios. | Propuesta |

---

## 16. Registro de sesiones de trabajo

> Actualizar al final de cada sesión (con la IA). Sirve para retomar el contexto sin repasar todo.

### Sesión 01 — 2026-11-17
- Hito: M0
- Hecho: revisión del documento de diseño con mejoras técnicas; instalación de `rustc`/`cargo` 1.100.0; inicialización del crate `anteiku`, `.gitignore`, `LICENSE` (MIT), `README.md` inicial y workflow `.github/workflows/ci.yml`.
- Decisiones nuevas: usar `anteiku` en minúsculas para el crate/binario/rutas, añadir `--demo` y temática de cafetería en el arte ASCII.
- Conceptos de Rust aprendidos: estructura de un proyecto Cargo (`Cargo.toml`, `Cargo.lock`), `cargo fmt`, `cargo clippy` y `cargo test`.
- Problemas abiertos: comprobar ejecución del workflow de CI en GitHub tras hacer push de la PR.
- Próximo paso: Hito M1 — Snapshot en texto (MVP) en rama `feat/m1-snapshot`.

### Sesión 02 — 2026-11-18
- Hito: M1
- Hecho: implementados `error.rs` (`CollectorError` con `thiserror`), `model.rs` (`Snapshot` y structs con `serde::Serialize`), `util.rs` (formateo + 4 tests unitarios), `collector/` (`Collector` + `SystemCollector` con `sysinfo 0.40.4`), `cli.rs` (`clap`) y `main.rs` (`snapshot [--json]`).
- Decisiones nuevas: usar `CollectorError` como nombre del enum de errores de recolección; adaptar llamadas a la API de `sysinfo 0.40.4` (`Result` en constructores y `System::name`/`uptime`, `Cpu::usage()`).
- Conceptos de Rust aprendidos: `thiserror`, `#[derive(Serialize, Parser, Subcommand)]`, `Option`/`Result`, traits (`Collector`), `#[cfg(test)]` y tests unitarios.
- Problemas abiertos: ninguno.
- Próximo paso: Hito M2 — Procesos, errores y pruebas (`anteiku top`, `FakeCollector`, tests con `assert_cmd`) en rama `feat/m2-procesos-pruebas`.

---

## 17. Preguntas abiertas

- ¿Nombre definitivo? (comprobar disponibilidad en crates.io y GitHub).
- ¿Qué dibujos ASCII incluir primero y quién los crea (a mano o con conversión)?
- ¿Se agregará el árbol de procesos en v1.0 o queda para después?
- ¿Se publicará una imagen Docker?
- ¿Se soporta Windows desde v0.1 o se prioriza Linux y se agrega después?

---

## 18. Referencias útiles

- *The Rust Programming Language* (el "Book"): doc.rust-lang.org/book
- *Rust by Example*: doc.rust-lang.org/rust-by-example
- Documentación de `ratatui`: ratatui.rs
- Documentación de `sysinfo`, `clap`, `rusqlite` en docs.rs
- *Command Line Applications in Rust*: rust-cli.github.io/book
- Código de proyectos similares para estudiar (sin copiar): `bottom`, `bandwhich`, `fastfetch` (referencia de diseño).
