-- Migración para crear la tabla users y el enum role
-- Fecha: 2025-03-13

-- Crear el tipo ENUM para roles
CREATE TYPE user_role AS ENUM (
    'Admin',
    'Director',
    'Teacher', 
    'Student',
    'Parent',
    'Secretary',
    'Accountant'
);

-- Crear la tabla de usuarios
CREATE TABLE users (
    id UUID PRIMARY KEY,
    document_id VARCHAR(20) NOT NULL,
    full_name VARCHAR(100) NOT NULL,
    email VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    address TEXT,
    birth_date DATE NOT NULL,
    role user_role NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    
    -- Restricciones de unicidad
    CONSTRAINT users_document_id_unique UNIQUE (document_id),
    CONSTRAINT users_email_unique UNIQUE (email)
);

-- Crear índices para búsquedas frecuentes
CREATE INDEX idx_users_full_name ON users (full_name);
CREATE INDEX idx_users_role ON users (role);
CREATE INDEX idx_users_created_at ON users (created_at);

-- Comentarios en la tabla y columnas
COMMENT ON TABLE users IS 'Tabla principal de usuarios del sistema';
COMMENT ON COLUMN users.id IS 'Identificador único UUID del usuario';
COMMENT ON COLUMN users.document_id IS 'Número de documento de identidad (cédula paraguaya)';
COMMENT ON COLUMN users.full_name IS 'Nombre completo del usuario';
COMMENT ON COLUMN users.email IS 'Correo electrónico de contacto';
COMMENT ON COLUMN users.phone IS 'Número de teléfono de contacto';
COMMENT ON COLUMN users.address IS 'Dirección física del usuario';
COMMENT ON COLUMN users.birth_date IS 'Fecha de nacimiento del usuario';
COMMENT ON COLUMN users.role IS 'Rol del usuario en el sistema (Admin, Director, Teacher, etc.)';
COMMENT ON COLUMN users.created_at IS 'Fecha y hora de creación del registro';
COMMENT ON COLUMN users.updated_at IS 'Fecha y hora de la última actualización del registro';

