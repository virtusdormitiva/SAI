# Guía de Contribución para SAI (Sistema Administrativo Integral)

¡Gracias por tu interés en contribuir al Sistema Administrativo Integral (SAI) del Anglo! Este documento proporciona las pautas y procesos para hacer contribuciones efectivas al proyecto.

## Tabla de Contenidos

- [Código de Conducta](#código-de-conducta)
- [Configuración del Entorno de Desarrollo](#configuración-del-entorno-de-desarrollo)
- [Estructura del Proyecto](#estructura-del-proyecto)
- [Convenciones de Código](#convenciones-de-código)
- [Flujo de Trabajo de Git](#flujo-de-trabajo-de-git)
- [Proceso de Pull Request](#proceso-de-pull-request)
- [Estándares de Commit](#estándares-de-commit)
- [Pruebas](#pruebas)
- [Documentación](#documentación)
- [Gestión de Problemas](#gestión-de-problemas)

## Código de Conducta

Esperamos que todos los contribuyentes mantengan un ambiente respetuoso y colaborativo. Por favor, sé considerado y respetuoso con los demás participantes, valorando la diversidad de perspectivas y experiencias.

## Configuración del Entorno de Desarrollo

### Requisitos Previos

- [Rust](https://www.rust-lang.org/tools/install) (versión estable más reciente)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (incluido con Rust)
- [Git](https://git-scm.com/downloads)
- [PostgreSQL](https://www.postgresql.org/download/) (para desarrollo local)
- [Docker](https://www.docker.com/get-started) (opcional, para contenerización)

### Pasos para Configurar

1. Clonar el repositorio:
   ```bash
   git clone https://github.com/virtusdormitiva/SAI.git
   cd SAI
   ```

2. Instalar dependencias:
   ```bash
   cargo build
   ```

3. Configurar la base de datos (si es necesario):
   ```bash
   # Instrucciones específicas para PostgreSQL
   ```

4. Ejecutar las pruebas para verificar la configuración:
   ```bash
   cargo test
   ```

5. Iniciar el servidor de desarrollo:
   ```bash
   cargo run
   ```

## Estructura del Proyecto

```
SAI/
├── src/              # Código fuente principal
│   ├── main.rs       # Punto de entrada
│   ├── lib.rs        # Biblioteca del proyecto
│   ├── models/       # Modelos de datos
│   ├── routes/       # Definiciones de rutas API
│   ├── services/     # Lógica de negocio
│   └── utils/        # Utilidades y helpers
├── tests/            # Pruebas de integración
├── docs/             # Documentación
├── migrations/       # Migraciones de base de datos
├── .github/          # Configuración de GitHub (workflows CI/CD)
└── configs/          # Archivos de configuración
```

## Convenciones de Código

### Estilo de Código

Seguimos las convenciones oficiales de Rust:

- Utiliza 4 espacios para la indentación, no tabulaciones
- Líneas limitadas a 100 caracteres
- Usa `snake_case` para variables, funciones y módulos
- Usa `CamelCase` para tipos y enums
- Usa `SCREAMING_SNAKE_CASE` para constantes

### Formateo

Usamos [rustfmt](https://github.com/rust-lang/rustfmt) para formatear automáticamente el código:

```bash
cargo fmt
```

### Linting

Usamos [Clippy](https://github.com/rust-lang/rust-clippy) para detectar errores comunes y mejorar la calidad del código:

```bash
cargo clippy -- -D warnings
```

### Gestión de Dependencias

- Mantén las dependencias actualizadas y mínimas
- Justifica la inclusión de nuevas dependencias
- Especifica versiones exactas en el `Cargo.toml`

## Flujo de Trabajo de Git

Seguimos un modelo basado en ramas:

- `main`: Rama principal, siempre estable y lista para producción
- `dev`: Rama de desarrollo, integración continua
- `feature/nombre-caracteristica`: Ramas para nuevas características
- `bugfix/nombre-error`: Ramas para corrección de errores
- `hotfix/nombre-urgente`: Ramas para correcciones urgentes en producción

### Crear una Nueva Característica

```bash
git checkout dev
git pull origin dev
git checkout -b feature/nombre-caracteristica
# Realiza tus cambios
git add .
git commit -m "feat: descripción del cambio"
git push origin feature/nombre-caracteristica
```

## Proceso de Pull Request

1. **Crea un Pull Request (PR)** desde tu rama hacia `dev`
2. **Describe tus cambios** detalladamente:
   - Qué cambia
   - Por qué se necesita
   - Cómo se ha implementado
   - Cómo se ha probado
3. **Vincula cualquier issue** relacionado
4. **Espera revisión** de al menos un miembro del equipo
5. **Aborda los comentarios** de la revisión
6. Una vez aprobado, un mantenedor realizará el merge

### Lista de Verificación para PRs

- [ ] Los tests pasan localmente
- [ ] Se han añadido nuevos tests para la funcionalidad
- [ ] El código sigue las convenciones de estilo
- [ ] La documentación ha sido actualizada
- [ ] Los cambios son compatibles con las versiones anteriores (o se documentan las rupturas)

## Estándares de Commit

Utilizamos [Conventional Commits](https://www.conventionalcommits.org/) para mantener un historial de cambios claro:

```
<tipo>(<ámbito opcional>): <descripción>

[cuerpo opcional]

[pie opcional]
```

### Tipos de Commit

- `feat`: Nueva característica
- `fix`: Corrección de error
- `docs`: Cambios en documentación
- `style`: Cambios que no afectan el código (espacios, formato)
- `refactor`: Refactorización de código
- `perf`: Mejoras de rendimiento
- `test`: Añadir o corregir pruebas
- `chore`: Cambios en el proceso de construcción o herramientas auxiliares

Ejemplos:
```
feat(auth): implementar autenticación JWT
fix(db): corregir error en consulta SQL
docs: actualizar instrucciones de instalación
```

## Pruebas

### Tipos de Pruebas

- **Pruebas Unitarias**: Dentro de los archivos de código con `#[cfg(test)]`
- **Pruebas de Integración**: En el directorio `tests/`
- **Pruebas de Rendimiento**: Usando `criterion` en `benches/`

### Ejecutar Pruebas

```bash
# Todas las pruebas
cargo test

# Pruebas específicas
cargo test nombre_test

# Pruebas con salida verbose
cargo test -- --nocapture
```

### Cobertura de Código

Utilizamos [tarpaulin](https://github.com/xd009642/tarpaulin) para medir la cobertura:

```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Documentación

- Documenta todas las funciones públicas, estructuras y traits
- Usa comentarios de documentación de Rust (`///` para items y `//!` para módulos)
- Incluye ejemplos en la documentación cuando sea útil
- Mantén actualizado el README y otros documentos de nivel superior

Para generar documentación:
```bash
cargo doc --open
```

## Gestión de Problemas

### Reportar Errores

Al reportar un error, incluye:
- Versión de Rust y del proyecto
- Pasos para reproducir
- Comportamiento esperado vs. actual
- Logs relevantes o capturas de pantalla

### Proponer Mejoras

Las propuestas de mejora deben incluir:
- Descripción clara del problema que resuelve
- Beneficios y posibles inconvenientes
- Implementación sugerida (opcional)

## Adaptaciones Específicas para Normativas Paraguayas

- Todos los módulos que manejen datos personales deben cumplir con la Ley N° 6534 de Protección de Datos Personales
- La documentación fiscal debe seguir las normativas de la SET (Subsecretaría de Estado de Tributación)
- Los formatos de fechas deben seguir el estándar paraguayo (DD/MM/YYYY)

---

¡Gracias por contribuir al proyecto SAI! Tu tiempo y esfuerzo ayudan a mejorar este sistema para toda la comunidad educativa del Anglo.

