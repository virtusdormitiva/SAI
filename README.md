# Sistema Administrativo Integral (SAI) 🚀

[![Estado](https://img.shields.io/badge/Estado-Desarrollo%20Activo-brightgreen)](https://github.com/virtusdormitiva/SAI)
[![Versión](https://img.shields.io/badge/Versión-0.1.0--alpha-blue)](https://github.com/virtusdormitiva/SAI/releases)
[![Rust](https://img.shields.io/badge/Rust-1.76+-orange)](https://www.rust-lang.org/)
[![Licencia](https://img.shields.io/badge/Licencia-MIT-yellow)](LICENSE)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-Compatible-5849be)](https://webassembly.org/)
[![Progreso](https://img.shields.io/badge/Progreso-35%25-informational)](ROADMAP.md)

## 📄 Propuesta Comercial

Consulte nuestra [Propuesta Comercial](PROPUESTA.md) detallada para conocer cómo SAI puede transformar la gestión administrativa de su institución educativa, incluyendo nuestro plan de implementación, beneficios competitivos, y proceso de migración de datos.

## 📋 Descripción del Proyecto

El Sistema Administrativo Integral (SAI) es una plataforma desarrollada en Rust diseñada para gestionar de manera eficiente todos los procesos administrativos institucionales. Esta solución integral combina seguridad, rendimiento y adaptabilidad a las normativas paraguayas para ofrecer una experiencia completa de gestión institucional.

## ✨ Características Principales

- ⚡ **Alto Rendimiento**: Arquitectura optimizada para operaciones concurrentes y asíncronas, 200% más rápido que soluciones tradicionales
- 🔒 **Seguridad Avanzada**: Implementación de los principios de ownership y borrowing de Rust para garantizar la integridad de datos
- ⚖️ **Cumplimiento Legal**: Adaptado a normativas paraguayas de protección de datos y requisitos locales
- 🧩 **Arquitectura Modular**: Sistema basado en microservicios para facilitar el mantenimiento y la escalabilidad
- 🌐 **Interfaz Web Moderna**: Frontend desarrollado con WebAssembly (WASM) para una experiencia de usuario fluida

## 🔍 Estado Actual del Proyecto (Marzo 2025)

> **Progreso General: 35%**

### Estructura Implementada

```
SAI/
├── .github/workflows/ (CI/CD configurado)
├── configs/ (Conexión a BD y servidor)
├── docs/ (Documentación técnica)
├── migrations/ (Estructura de BD)
├── src/
│   ├── models/ (Entidades de datos)
│   ├── routes/ (API endpoints)
│   ├── services/ (Lógica de negocio)
│   ├── utils/ (Herramientas comunes)
│   ├── lib.rs (Biblioteca principal)
│   └── main.rs (Punto de entrada)
└── tests/ (Pruebas automatizadas)
```

### Módulos Implementados

- ✅ **Estructura Base**: Arquitectura modular completa
- ✅ **DevOps**: Contenedores Docker y flujos CI/CD
- ✅ **Configuración**: Variables de entorno y conexiones
- 🔄 **Modelos de Datos**: Principales entidades definidas (70%)
- 🔄 **API Core**: Endpoints básicos implementados (45%)
- ✅ **API de Cursos**: CRUD completo y estadísticas por año académico
- ✅ **API de Estudiantes**: CRUD completo y gestión de registros académicos
- ✅ **API de Profesores**: CRUD completo y asignación de cursos
- ✅ **API de Usuarios**: CRUD completo y gestión de perfiles
- 🔄 **Autenticación**: Sistema de login y permisos (40%)

### Próximas Etapas

1. **Abril 2025**: Completar la implementación de modelos de datos y conexión a BD
2. **Mayo 2025**: Finalizar API REST completa y servicios básicos
3. **Junio 2025**: Implementar frontend con WebAssembly
4. **Julio 2025**: Pruebas integradas y optimización
5. **Agosto 2025**: Lanzamiento Beta para instituciones seleccionadas

## 🔬 Innovación Tecnológica

El SAI representa un avance significativo en los sistemas administrativos educativos en Paraguay, utilizando tecnologías de vanguardia:

### 🦀 Rust como Lenguaje Principal

- **Seguridad de Memoria**: Eliminación de errores comunes en tiempo de compilación
- **Rendimiento Nativo**: Velocidad comparable a C/C++ con abstracciones de alto nivel
- **Concurrencia Segura**: Modelo de propiedad que previene condiciones de carrera
- **Interoperabilidad**: Fácil integración con sistemas existentes

### 🕸️ WebAssembly (WASM)

- **Rendimiento Web de Nivel Nativo**: Ejecución casi a velocidad nativa en navegadores
- **Experiencia de Usuario Fluida**: Interfaces reactivas sin recarga de página
- **Carga Reducida en Servidores**: Procesamiento distribuido entre cliente y servidor
- **Compatibilidad Universal**: Funciona en todos los navegadores modernos

### 🔄 Arquitectura de Microservicios

- **Escalabilidad Horizontal**: Capacidad para crecer según demanda
- **Resiliencia**: Fallos aislados sin afectar todo el sistema
- **Despliegue Independiente**: Actualización de componentes sin interrupciones
- **Especialización Tecnológica**: Cada servicio utiliza herramientas óptimas para su función

### 🛡️ Seguridad Avanzada

- **Cifrado de Datos**: Protección de información sensible mediante algoritmos modernos
- **Autenticación Multi-factor**: Capas múltiples de verificación de identidad
- **Auditoría Completa**: Registro detallado de todas las operaciones
- **Cumplimiento RGPD**: Adaptado a estándares internacionales de protección de datos

## 🛠️ Requisitos

### Requisitos de Desarrollo

- Rust (versión 1.76+)
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

## 📦 Instalación

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

## 🖥️ Uso

### Acceso al Sistema

El sistema estará disponible en `http://localhost:8080` después de iniciar el servidor de desarrollo.

Credenciales por defecto:
- Usuario: `admin`
- Contraseña: `sai_admin_temp`

> **IMPORTANTE**: Cambiar la contraseña inmediatamente después del primer inicio de sesión.

### Módulos Principales

- 🎓 **Gestión Académica**: Administración de estudiantes, profesores y cursos
- 💰 **Finanzas**: Control de pagos, facturación y reportes financieros
- 👥 **Recursos Humanos**: Gestión de personal y nómina
- 📦 **Inventario**: Control de activos y suministros
- 📊 **Reportes**: Generación de informes personalizados y análisis de datos

## 👥 Contribución

Agradecemos y fomentamos las contribuciones al proyecto. Para contribuir:

1. Revisa las [issues abiertas](https://github.com/virtusdormitiva/SAI/issues) o crea una nueva
2. Bifurca (fork) el repositorio
3. Crea una rama para tu contribución (`git checkout -b feature/nueva-funcionalidad`)
4. Realiza tus cambios siguiendo las [guías de estilo de Rust](https://doc.rust-lang.org/1.0.0/style/README.html)
5. Ejecuta los tests (`cargo test`)
6. Envía un Pull Request con una descripción detallada de tus cambios

Para más detalles, consulta el archivo [CONTRIBUTING.md](CONTRIBUTING.md).

## 📄 Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## 📞 Contacto

Para preguntas o soporte, contacta al equipo de desarrollo en [favaratoraphael@gmail.com](mailto:favaratoraphael@gmail.com).

---

<p align="center">
  <img src="https://rustacean.net/assets/rustacean-flat-happy.png" width="200">
  <br>
  <i>Desarrollado con 🦀 Rust y ❤️ en Paraguay</i>
</p>
