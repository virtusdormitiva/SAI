-- Migration: Create students table
-- Description: Creates the necessary tables and types for student management
-- Date: 2025-03-13

-- Create student status enum type
CREATE TYPE student_status AS ENUM (
    'active',       -- Currently enrolled student
    'inactive',     -- Student temporarily not attending
    'graduated',    -- Student who completed their education
    'transferred',  -- Student who moved to another institution
    'suspended',    -- Student temporarily suspended
    'expelled'      -- Student permanently removed from institution
);

-- Create students table
CREATE TABLE students (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    enrollment_date DATE NOT NULL DEFAULT CURRENT_DATE,
    student_id VARCHAR(20) NOT NULL UNIQUE, -- Institution-specific student ID
    grade INTEGER NOT NULL,
    section VARCHAR(10) NOT NULL,
    status student_status NOT NULL DEFAULT 'active',
    guardian_info JSONB NOT NULL DEFAULT '{}', -- Stores parent/guardian contact information
    medical_info JSONB DEFAULT '{}', -- Stores medical conditions, allergies, etc.
    academic_history JSONB DEFAULT '[]', -- Previous academic records
    
    -- School-specific fields
    scholarship_type VARCHAR(50) DEFAULT NULL,
    scholarship_percentage DECIMAL(5,2) DEFAULT 0.00,
    transportation_route VARCHAR(50) DEFAULT NULL,
    
    -- Metadata
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT grade_check CHECK (grade >= 0 AND grade <= 12)
);

-- Add comments to the table and columns
COMMENT ON TABLE students IS 'Students enrolled in the institution';
COMMENT ON COLUMN students.id IS 'Unique identifier for the student record';
COMMENT ON COLUMN students.user_id IS 'Foreign key to the users table for basic user information';
COMMENT ON COLUMN students.enrollment_date IS 'Date when the student was enrolled in the institution';
COMMENT ON COLUMN students.student_id IS 'Institution-specific identifier for the student';
COMMENT ON COLUMN students.grade IS 'Current grade level of the student (0-12)';
COMMENT ON COLUMN students.section IS 'Class section or division (e.g., A, B, C)';
COMMENT ON COLUMN students.status IS 'Current enrollment status of the student';
COMMENT ON COLUMN students.guardian_info IS 'JSON data containing parent/guardian contact information';
COMMENT ON COLUMN students.medical_info IS 'JSON data containing medical conditions, allergies, etc.';
COMMENT ON COLUMN students.academic_history IS 'JSON array containing previous academic records';
COMMENT ON COLUMN students.scholarship_type IS 'Type of scholarship if applicable (full, partial, merit, etc.)';
COMMENT ON COLUMN students.scholarship_percentage IS 'Percentage of tuition covered by scholarship';
COMMENT ON COLUMN students.transportation_route IS 'Transportation route code if using school transport';

-- Create indexes for common query patterns
CREATE INDEX idx_students_user_id ON students(user_id);
CREATE INDEX idx_students_grade_section ON students(grade, section);
CREATE INDEX idx_students_status ON students(status);
CREATE INDEX idx_students_enrollment_date ON students(enrollment_date);

-- Add trigger to update the updated_at column
CREATE OR REPLACE FUNCTION update_students_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_students_timestamp
BEFORE UPDATE ON students
FOR EACH ROW EXECUTE FUNCTION update_students_timestamp();

