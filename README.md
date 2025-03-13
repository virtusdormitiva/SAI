# Sistema Administrativo Integral (SAI) del Anglo

## Descripción del Proyecto

El Sistema Administrativo Integral (SAI) es una plataforma desarrollada en Rust diseñada para gestionar de manera eficiente todos los procesos administrativos del Anglo. Esta solución integral combina seguridad, rendimiento y adaptabilidad a las normativas paraguayas para ofrecer una experiencia completa de gestión institucional.

### Características Principales

- **Seguridad Avanzada**: Implementación de los principios de ownership y borrowing de Rust para garantizar la integridad de datos.
- **Alto Rendimiento**: Arquitectura optimizada para operaciones concurrentes y asíncronas.
- **Cumplimiento Legal**: Adaptado a normativas paraguayas de protección de datos y requisitos locales.
- **Arquitectura Modular**: Sistema basado en microservicios para facilitar el mantenimiento y la escalabilidad.
- **Interfaz Web Moderna**: Frontend desarrollado con WebAssembly (WASM) para una experiencia de usuario fluida.

## Requisitos

### Requisitos de Desarrollo

- Rust (versión estable más reciente)
- Cargo (incluido con Rust)
- Git
- Docker y Docker Compose
- PostgreSQL 13+
- Redis
- Acceso a Internet para descarga de dependencias

### Requisitos del Sistema

- Sistema operativo: Linux, macOS, o Windows con WSL2
- Mínimo 4GB de RAM para desarrollo
- Al menos 10GB de espacio en disco
- Conexión a Internet para servicios de autenticación y actualizaciones

## Instalación

1. **Clonar el repositorio**:
   ```bash
   git clone https://github.com/virtusdormitiva/SAI.git
   cd SAI
   ```

2. **Instalar dependencias**:
   ```bash
   cargo build
   ```

3. **Configurar la base de datos**:
   ```bash
   # Iniciar servicios de base de datos con Docker
   docker-compose up -d db redis
   
   # Ejecutar migraciones
   cargo run --bin migrations
   ```

4. **Configurar variables de entorno**:
   ```bash
   cp .env.example .env
   # Editar .env con la configuración local
   ```

5. **Iniciar el servidor de desarrollo**:
   ```bash
   cargo run
   ```

## Uso

### Acceso al Sistema

El sistema estará disponible en `http://localhost:8080` después de iniciar el servidor de desarrollo.

Credenciales por defecto:
- Usuario: `admin`
- Contraseña: `sai_admin_temp`

**IMPORTANTE**: Cambiar la contraseña inmediatamente después del primer inicio de sesión.

### Módulos Principales

- **Gestión Académica**: Administración de estudiantes, profesores y cursos.
- **Finanzas**: Control de pagos, facturación y reportes financieros.
- **Recursos Humanos**: Gestión de personal y nómina.
- **Inventario**: Control de activos y suministros.
- **Reportes**: Generación de informes personalizados y análisis de datos.

## Contribución

Agradecemos y fomentamos las contribuciones al proyecto. Para contribuir:

1. Revisa las [issues abiertas](https://github.com/virtusdormitiva/SAI/issues) o crea una nueva.
2. Bifurca (fork) el repositorio.
3. Crea una rama para tu contribución (`git checkout -b feature/nueva-funcionalidad`).
4. Realiza tus cambios siguiendo las [guías de estilo de Rust](https://doc.rust-lang.org/1.0.0/style/README.html).
5. Ejecuta los tests (`cargo test`).
6. Envía un Pull Request con una descripción detallada de tus cambios.

Para más detalles, consulta el archivo [CONTRIBUTING.md](CONTRIBUTING.md).

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## Contacto

Para preguntas o soporte, contacta al equipo de desarrollo en [desarrollo@anglo.edu.py](mailto:desarrollo@anglo.edu.py).

