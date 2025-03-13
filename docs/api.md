# SAI API Documentation

## Overview

This document provides details about the API endpoints available in the SAI (Sistema Administrativo Integral) application.

## Authentication

TBD - Authentication mechanism details will be added when implemented.

## Endpoints

### Courses

- **GET /api/courses** - Retrieve list of all courses
- **GET /api/courses/{id}** - Retrieve a specific course by ID
- **POST /api/courses** - Create a new course
- **PUT /api/courses/{id}** - Update an existing course
- **DELETE /api/courses/{id}** - Delete a course

### Students

- **GET /api/students** - Retrieve list of all students
- **GET /api/students/{id}** - Retrieve a specific student by ID
- **POST /api/students** - Register a new student
- **PUT /api/students/{id}** - Update student information
- **DELETE /api/students/{id}** - Remove a student

### Teachers

- **GET /api/teachers** - Retrieve list of all teachers
- **GET /api/teachers/{id}** - Retrieve a specific teacher by ID
- **POST /api/teachers** - Register a new teacher
- **PUT /api/teachers/{id}** - Update teacher information
- **DELETE /api/teachers/{id}** - Remove a teacher

### Users

- **POST /api/users/login** - User login
- **POST /api/users/register** - Register new user
- **GET /api/users/profile** - Get current user profile
- **PUT /api/users/profile** - Update user profile

## Status Codes

- **200 OK** - Request succeeded
- **201 Created** - Resource created successfully
- **400 Bad Request** - Invalid request parameters
- **401 Unauthorized** - Authentication required
- **403 Forbidden** - User doesn't have permission
- **404 Not Found** - Resource not found
- **500 Server Error** - Internal server error

## Data Models

Detailed data models will be added as the API is implemented.
