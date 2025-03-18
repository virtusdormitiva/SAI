-- Create assessments table to track student evaluations
CREATE TABLE IF NOT EXISTS assessments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    assessment_type VARCHAR(50) NOT NULL CHECK (assessment_type IN ('test', 'quiz', 'assignment', 'project', 'exam', 'presentation', 'participation', 'other')),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    score DECIMAL(5, 2),
    max_score DECIMAL(5, 2) NOT NULL,
    weight DECIMAL(5, 2) NOT NULL CHECK (weight >= 0 AND weight <= 100),
    comments TEXT,
    assessment_date TIMESTAMP WITH TIME ZONE NOT NULL,
    due_date TIMESTAMP WITH TIME ZONE,
    submitted_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    
    -- Ensure score doesn't exceed max_score
    CONSTRAINT valid_score CHECK (score IS NULL OR (score >= 0 AND score <= max_score)),
    
    -- Create index for common queries
    CONSTRAINT unique_assessment_per_student_course UNIQUE(student_id, course_id, assessment_type, title)
);

-- Add indexes for performance
CREATE INDEX idx_assessments_student_id ON assessments(student_id);
CREATE INDEX idx_assessments_course_id ON assessments(course_id);
CREATE INDEX idx_assessments_type ON assessments(assessment_type);
CREATE INDEX idx_assessments_date ON assessments(assessment_date);

-- Function to update timestamp on record change
CREATE OR REPLACE FUNCTION update_assessments_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to automatically update timestamps
CREATE TRIGGER update_assessments_timestamp
BEFORE UPDATE ON assessments
FOR EACH ROW
EXECUTE FUNCTION update_assessments_timestamp();

-- Comments for documentation
COMMENT ON TABLE assessments IS 'Stores all student assessment data including tests, quizzes, assignments, and projects';
COMMENT ON COLUMN assessments.id IS 'Unique identifier for the assessment record';
COMMENT ON COLUMN assessments.student_id IS 'Reference to the student being assessed';
COMMENT ON COLUMN assessments.course_id IS 'Reference to the course the assessment belongs to';
COMMENT ON COLUMN assessments.assessment_type IS 'Type of assessment (test, quiz, assignment, project, etc.)';
COMMENT ON COLUMN assessments.title IS 'Title of the assessment';
COMMENT ON COLUMN assessments.description IS 'Detailed description of the assessment';
COMMENT ON COLUMN assessments.score IS 'Score achieved by the student (can be NULL if not yet graded)';
COMMENT ON COLUMN assessments.max_score IS 'Maximum possible score for the assessment';
COMMENT ON COLUMN assessments.weight IS 'Weight of the assessment in the overall course grade (percentage)';
COMMENT ON COLUMN assessments.comments IS 'Teacher comments or feedback on the assessment';
COMMENT ON COLUMN assessments.assessment_date IS 'Date when the assessment was given';
COMMENT ON COLUMN assessments.due_date IS 'Deadline for assessment submission (if applicable)';
COMMENT ON COLUMN assessments.submitted_date IS 'Date when the student submitted the assessment (if applicable)';

