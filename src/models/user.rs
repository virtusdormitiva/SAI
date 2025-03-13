use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Error as SqlxError, postgres::PgQueryResult};
use uuid::Uuid;

use crate::models::Role;

/// Re-exportamos User para facilitar su uso en el módulo models
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Identificador único del usuario
    pub id: Uuid,
    /// Número de documento de identidad (cédula)
    pub document_id: String,
    /// Nombre completo del usuario
    pub full_name: String,
    /// Correo electrónico de contacto
    pub email: String,
    /// Número de teléfono de contacto
    pub phone: Option<String>,
    /// Dirección física del usuario
    pub address: Option<String>,
    /// Fecha de nacimiento
    pub birth_date: NaiveDate,
    /// Rol del usuario en el sistema
    pub role: Role,
    /// Fecha de creación del registro
    pub created_at: DateTime<Utc>,
    /// Última actualización del registro
    pub updated_at: DateTime<Utc>,
}

/// DTO para la creación de un nuevo usuario
#[derive(Debug, Deserialize)]
pub struct CreateUserDto {
    pub document_id: String,
    pub full_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub birth_date: NaiveDate,
    pub role: Role,
}

/// DTO para la actualización de un usuario
#[derive(Debug, Deserialize)]
pub struct UpdateUserDto {
    pub document_id: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub role: Option<Role>,
}

/// Filtros para la búsqueda de usuarios
#[derive(Debug, Deserialize, Default)]
pub struct UserFilter {
    pub id: Option<Uuid>,
    pub document_id: Option<String>,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
}

impl User {
    /// Crea un nuevo usuario en la base de datos
    pub async fn create(pool: &PgPool, dto: CreateUserDto) -> Result<User, SqlxError> {
        let now = Utc::now();
        let id = Uuid::new_v4();

        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, document_id, full_name, email, phone, address, birth_date, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING id, document_id, full_name, email, phone, address, birth_date, role as "role: Role", created_at, updated_at
            "#,
            id,
            dto.document_id,
            dto.full_name,
            dto.email,
            dto.phone,
            dto.address,
            dto.birth_date,
            dto.role as Role,
            now,
            now
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    /// Encuentra un usuario por su ID
    pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, document_id, full_name, email, phone, address, birth_date, role as "role: Role", created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Encuentra un usuario por su documento de identidad
    pub async fn find_by_document_id(pool: &PgPool, document_id: &str) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, document_id, full_name, email, phone, address, birth_date, role as "role: Role", created_at, updated_at
            FROM users
            WHERE document_id = $1
            "#,
            document_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Encuentra un usuario por su correo electrónico
    pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, SqlxError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, document_id, full_name, email, phone, address, birth_date, role as "role: Role", created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    /// Lista todos los usuarios con opción de filtrado y paginación
    pub async fn find_all(
        pool: &PgPool, 
        filter: UserFilter,
        limit: Option<i64>,
        offset: Option<i64>
    ) -> Result<Vec<User>, SqlxError> {
        // Construimos la consulta base
        let mut query = String::from(
            "SELECT id, document_id, full_name, email, phone, address, birth_date, role, created_at, updated_at 
             FROM users WHERE 1=1"
        );

        // Aplicamos los filtros si existen
        let mut params = Vec::<String>::new();
        let mut param_count = 1;

        if let Some(id) = filter.id {
            query.push_str(&format!(" AND id = ${}", param_count));
            params.push(id.to_string());
            param_count += 1;
        }

        if let Some(document_id) = &filter.document_id {
            query.push_str(&format!(" AND document_id = ${}", param_count));
            params.push(document_id.to_string());
            param_count += 1;
        }

        if let Some(full_name) = &filter.full_name {
            query.push_str(&format!(" AND full_name ILIKE ${}", param_count));
            params.push(format!("%{}%", full_name));
            param_count += 1;
        }

        if let Some(email) = &filter.email {
            query.push_str(&format!(" AND email ILIKE ${}", param_count));
            params.push(format!("%{}%", email));
            param_count += 1;
        }

        if let Some(role) = &filter.role {
            query.push_str(&format!(" AND role = ${}", param_count));
            params.push(format!("{:?}", role));
            param_count += 1;
        }

        // Agregamos paginación
        query.push_str(" ORDER BY created_at DESC");

        if let Some(limit_val) = limit {
            query.push_str(&format!(" LIMIT ${}", param_count));
            params.push(limit_val.to_string());
            param_count += 1;
        }

        if let Some(offset_val) = offset {
            query.push_str(&format!(" OFFSET ${}", param_count));
            params.push(offset_val.to_string());
        }

        // Ejecutamos la consulta dinámica
        let mut q = sqlx::query(&query);
        for param in params {
            q = q.bind(param);
        }

        // Convertimos el resultado a instancias de User
        let rows = q.fetch_all(pool).await?;
        let users = rows
            .iter()
            .map(|row| {
                User {
                    id: row.get("id"),
                    document_id: row.get("document_id"),
                    full_name: row.get("full_name"),
                    email: row.get("email"),
                    phone: row.get("phone"),
                    address: row.get("address"),
                    birth_date: row.get("birth_date"),
                    role: serde_json::from_value(row.get("role")).unwrap_or(Role::Student),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            })
            .collect();

        Ok(users)
    }

    /// Actualiza un usuario existente
    pub async fn update(pool: &PgPool, id: Uuid, dto: UpdateUserDto) -> Result<User, SqlxError> {
        // Primero verificamos si el usuario existe
        let existing_user = Self::find_by_id(pool, id).await?;
        if existing_user.is_none() {
            return Err(SqlxError::RowNotFound);
        }

        let existing_user = existing_user.unwrap();
        let now = Utc::now();

        // Usamos los valores actuales si no se especifican nuevos
        let document_id = dto.document_id.unwrap_or(existing_user.document_id);
        let full_name = dto.full_name.unwrap_or(existing_user.full_name);
        let email = dto.email.unwrap_or(existing_user.email);
        let phone = dto.phone.or(existing_user.phone);
        let address = dto.address.or(existing_user.address);
        let birth_date = dto.birth_date.unwrap_or(existing_user.birth_date);
        let role = dto.role.unwrap_or(existing_user.role);

        let updated_user = sqlx::query_as!(
            User,
            r#"
            UPDATE users 
            SET document_id = $1, full_name = $2, email = $3, phone = $4, address = $5, 
                birth_date = $6, role = $7, updated_at = $8
            WHERE id = $9
            RETURNING id, document_id, full_name, email, phone, address, birth_date, role as "role: Role", created_at, updated_at
            "#,
            document_id,
            full_name,
            email,
            phone,
            address,
            birth_date,
            role as Role,
            now,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(updated_user)
    }

    /// Elimina un usuario por su ID
    pub async fn delete(pool: &PgPool, id: Uuid) -> Result<PgQueryResult, SqlxError> {
        // Verificamos si el usuario existe
        let existing_user = Self::find_by_id(pool, id).await?;
        if existing_user.is_none() {
            return Err(SqlxError::RowNotFound);
        }

        let result = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(result)
    }

    /// Cuenta el número total de usuarios que coinciden con un filtro
    pub async fn count(pool: &PgPool, filter: UserFilter) -> Result<i64, SqlxError> {
        // Construimos la consulta base
        let mut query = String::from("SELECT COUNT(*) FROM users WHERE 1=1");

        // Aplicamos los filtros si existen
        let mut params = Vec::<String>::new();
        let mut param_count = 1;

        if let Some(id) = filter.id {
            query.push_str(&format!(" AND id = ${}", param_count));
            params.push(id.to_string());
            param_count += 1;
        }

        if let Some(document_id) = &filter.document_id {
            query.push_str(&format!(" AND document_id = ${}", param_count));
            params.push(document_id.to_string());
            param_count += 1;
        }

        if let Some(full_name) = &filter.full_name {
            query.push_str(&format!(" AND full_name ILIKE ${}", param_count));
            params.push(format!("%{}%", full_name));
            param_count += 1;
        }

        if let Some(email) = &filter.email {
            query.push_str(&format!(" AND email ILIKE ${}", param_count));
            params.push(format!("%{}%", email));
            param_count += 1;
        }

        if let Some(role) = &filter.role {
            query.push_str(&format!(" AND role = ${}", param_count));
            params.push(format!("{:?}", role));
        }

        // Ejecutamos la consulta dinámica
        let mut q = sqlx::query(&query);
        for param in params {
            q = q.bind(param);
        }

        let row = q.fetch_one(pool).await?;
        let count: i64 = row.get(0);

        Ok(count)
    }

    /// Busca usuarios por rol
    pub async fn find_by_role(pool: &PgPool, role: Role) -> Result<Vec<User>, SqlxError> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, document_id, full_name, email, phone, address, birth_date, role as "role: Role", created_at, updated_at
            FROM users
            WHERE role = $1
            ORDER BY full_name
            "#,
            role as Role
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }

    /// Busca usuarios por coincidencia parcial en el nombre
    pub async fn search_by_name(pool: &PgPool, name_query: &str) -> Result<Vec<User>, SqlxError> {
        let search_pattern = format!("%{}%", name_query);
        
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, document_id, full_name, email, phone, address, birth_date, role as "role: Role", created_at, updated_at
            FROM users
            WHERE full_name ILIKE $1
            ORDER BY full_name
            LIMIT 50
            "#,
            search_pattern
        )
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    

