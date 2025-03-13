-- Migration: Create Courses Table
-- Description: Defines the table for storing course information
-- Timestamp: 2025-03-13

-- Create course status enum type
CREATE TYPE course_status AS ENUM (
    'active',         -- Course is currently active
    'upcoming',       -- Course is scheduled but not yet started
    'completed',      -- Course has been completed
    'cancelled',      -- Course has been cancelled
    'archived'        -- Course is archived (past offerings)
);

-- Create course level/grade enum type
CREATE TYPE course_level AS ENUM (
    'preescolar',     -- Pre-school level
    'primaria',       -- Primary education level
    'secundaria',     -- Secondary education level
    'bachillerato',   -- High school level
    'extracurricular' -- Extracurricular programs
);

-- Create courses table
CREATE TABLE courses (
    id SERIAL PRIMARY KEY,
    
    -- Basic course information
    code VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    
    -- Academic information
    level course_level NOT NULL,
    grade INTEGER,                    -- Grade number within level (e.g., 1-6 for primary)
    section VARCHAR(5),               -- Section identifier (e.g., 'A', 'B', etc.)
    academic_year VARCHAR(9) NOT NULL, -- Format: YYYY-YYYY
    semester INTEGER,                 -- 1 or 2, NULL for non-semester courses
    
    -- Related parties
    main_teacher_id INTEGER REFERENCES teachers(id),
    
    -- Schedule and logistics
    schedule JSONB,                   -- Flexible storage for complex schedules
    location VARCHAR(255),            -- Where the course takes place
    max_students INTEGER NOT NULL DEFAULT 30,
    current_students INTEGER NOT NULL DEFAULT 0,
    
    -- Administrative information
    status course_status NOT NULL DEFAULT 'upcoming',
    credits INTEGER NOT NULL DEFAULT 0,
    cost_per_semester NUMERIC(10, 2),
    
    -- Curriculum and content management
    syllabus_url VARCHAR(500),
    materials JSONB,                  -- Required books, tools, etc.
    
    -- Timestamps
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    start_date DATE,
    end_date DATE,
    
    -- Optional custom fields for institution-specific requirements
    custom_fields JSONB,
    
    -- Constraints
    CONSTRAINT valid_grade_check CHECK (
        (level = 'primaria' AND grade BETWEEN 1 AND 6) OR
        (level = 'secundaria' AND grade BETWEEN 1 AND 3) OR
        (level = 'bachillerato' AND grade BETWEEN 1 AND 3) OR
        (level IN ('preescolar', 'extracurricular'))
    ),
    CONSTRAINT valid_semester_check CHECK (semester BETWEEN 1 AND 2 OR semester IS NULL),
    CONSTRAINT valid_student_count_check CHECK (current_students <= max_students)
);

-- Create indices for common queries
CREATE INDEX courses_academic_year_idx ON courses(academic_year);
CREATE INDEX courses_level_grade_idx ON courses(level, grade);
CREATE INDEX courses_main_teacher_idx ON courses(main_teacher_id);
CREATE INDEX courses_status_idx ON courses(status);

-- Add comments
COMMENT ON TABLE courses IS 'Stores information about academic courses offered by the institution';
COMMENT ON COLUMN courses.schedule IS 'JSON structure containing the weekly schedule with days, times, and locations';
COMMENT ON COLUMN courses.materials IS 'JSON structure listing required and optional materials for the course';
COMMENT ON COLUMN courses.custom_fields IS 'Extensible JSON structure for institution-specific requirements';

-- Create a view for active courses with teacher information
CREATE VIEW active_courses_with_teachers AS
SELECT 
    c.id, 
    c.code, 
    c.name, 
    c.level, 
    c.grade, 
    c.section,
    c.academic_year,
    c.schedule,
    c.max_students,
    c.current_students,
    t.id AS teacher_id,
    u.first_name AS teacher_first_name,
    u.last_name AS teacher_last_name
FROM 
    courses c
LEFT JOIN 
    teachers t ON c.main_teacher_id = t.id
LEFT JOIN 
    users u ON t.user_id = u.id
WHERE 
    c.status = 'active';

-- Function to automatically update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_courses_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to call the function before an update
CREATE TRIGGER update_courses_timestamp
BEFORE UPDATE ON courses
FOR EACH ROW
EXECUTE FUNCTION update_courses_updated_at();

