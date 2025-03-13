# SAI Database Schema Documentation

## Overview

This document describes the database schema used in the SAI (Sistema Administrativo Integral) application.

## Tables

### Users

Stores user authentication and permission information.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| username | VARCHAR | Unique username |
| email | VARCHAR | User's email address |
| password_hash | VARCHAR | Hashed user password |
| role | VARCHAR | User role (admin, teacher, etc.) |
| created_at | TIMESTAMP | Account creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

### Students

Stores information about students.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| first_name | VARCHAR | Student's first name |
| last_name | VARCHAR | Student's last name |
| document_id | VARCHAR | National ID number |
| birthdate | DATE | Date of birth |
| address | VARCHAR | Residential address |
| phone | VARCHAR | Contact phone number |
| email | VARCHAR | Contact email |
| enrollment_date | DATE | Date of enrollment |
| status | VARCHAR | Current status (active, inactive) |
| created_at | TIMESTAMP | Record creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

### Teachers

Stores information about teachers.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| user_id | UUID | Reference to Users table |
| first_name | VARCHAR | Teacher's first name |
| last_name | VARCHAR | Teacher's last name |
| document_id | VARCHAR | National ID number |
| specialization | VARCHAR | Teacher's specialization |
| hire_date | DATE | Date of hiring |
| contact_info | VARCHAR | Contact information |
| created_at | TIMESTAMP | Record creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

### Courses

Stores information about academic courses.

| Column | Type | Description |
|--------|------|-------------|
| id | UUID | Primary key |
| name | VARCHAR | Course name |
| code | VARCHAR | Course code |
| description | TEXT | Course description |
| academic_year | INTEGER | Academic year |
| start_date | DATE | Course start date |
| end_date | DATE | Course end date |
| teacher_id | UUID | Reference to teacher |
| credits | INTEGER | Number of credits |
| max_students | INTEGER | Maximum number of students |
| created_at | TIMESTAMP | Record creation timestamp |
| updated_at | TIMESTAMP | Last update timestamp |

## Relationships

- A User can be associated with one Teacher (one-to-one)
- A Teacher can teach multiple Courses (one-to-many)
- Students can enroll in multiple Courses and Courses can have multiple Students (many-to-many)

## Migrations

Database migrations are stored in the src/models/migrations directory and applied sequentially.
