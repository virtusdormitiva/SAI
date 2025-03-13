-- Migration: Create Teachers Table
-- Description: Creates the table for storing teacher-specific information
-- Date: March 13, 2025

-- Create the teacher status enum type
CREATE TYPE teacher_status AS ENUM (
  'active',       -- Currently employed and teaching
  'on_leave',     -- Temporarily on leave (medical, sabbatical, etc.)
  'suspended',    -- Temporarily suspended
  'terminated',   -- Employment terminated
  'retired'       -- Retired from teaching position
);

-- Create the teachers table
CREATE TABLE teachers (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  professional_id VARCHAR(50) UNIQUE,     -- Teacher's professional identification number
  specialization VARCHAR(100) NOT NULL,   -- Main area of specialization
  academic_degree VARCHAR(100),           -- Highest academic degree achieved
  years_experience INTEGER DEFAULT 0,     -- Years of professional teaching experience
  hire_date DATE NOT NULL,                -- Date when teacher started working
  contract_type VARCHAR(50),              -- Type of employment contract
  status teacher_status NOT NULL DEFAULT 'active',  -- Current employment status
  weekly_hours INTEGER DEFAULT 40,        -- Weekly teaching hours
  subjects JSONB DEFAULT '[]',            -- Subjects the teacher is qualified to teach
  certifications JSONB DEFAULT '[]',      -- Professional certifications
  evaluations JSONB DEFAULT '[]',         -- Performance evaluations
  additional_info TEXT,                   -- Any additional relevant information
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for frequent query patterns
CREATE INDEX teachers_user_id_idx ON teachers(user_id);
CREATE INDEX teachers_specialization_idx ON teachers(specialization);
CREATE INDEX teachers_status_idx ON teachers(status);
CREATE INDEX teachers_hire_date_idx ON teachers(hire_date);

-- Add GIN index for efficient JSON querying on subjects
CREATE INDEX teachers_subjects_idx ON teachers USING GIN (subjects);

-- Add comments to document the table and columns
COMMENT ON TABLE teachers IS 'Stores information specific to teachers, linked to the users table';
COMMENT ON COLUMN teachers.professional_id IS 'Unique professional identification number assigned by educational authorities';
COMMENT ON COLUMN teachers.specialization IS 'Main subject area or field in which the teacher specializes';
COMMENT ON COLUMN teachers.subjects IS 'JSON array of subjects the teacher is qualified to teach, may include detailed information per subject';
COMMENT ON COLUMN teachers.evaluations IS 'JSON array storing performance evaluation history';

-- Trigger to update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_teachers_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_teachers_timestamp
BEFORE UPDATE ON teachers
FOR EACH ROW
EXECUTE FUNCTION update_teachers_timestamp();

