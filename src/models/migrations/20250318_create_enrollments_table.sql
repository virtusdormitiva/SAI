-- Create enrollments table to track student course registrations
CREATE TABLE IF NOT EXISTS enrollments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    enrollment_date TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    status VARCHAR(20) NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive', 'completed', 'withdrawn', 'on_hold', 'pending')),
    completion_date TIMESTAMP WITH TIME ZONE,
    completion_status VARCHAR(20) CHECK (completion_status IN ('passed', 'failed', 'incomplete', 'exempted')),
    final_grade DECIMAL(5,2),
    notes TEXT,
    payment_status VARCHAR(20) DEFAULT 'pending' CHECK (payment_status IN ('pending', 'partial', 'paid', 'refunded', 'waived')),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_by UUID REFERENCES users(id),
    updated_by UUID REFERENCES users(id),
    UNIQUE(student_id, course_id),
    CHECK ((status = 'completed' AND completion_date IS NOT NULL AND completion_status IS NOT NULL) OR 
           (status != 'completed'))
);

-- Add indexes for performance
CREATE INDEX enrollments_student_id_idx ON enrollments(student_id);
CREATE INDEX enrollments_course_id_idx ON enrollments(course_id);
CREATE INDEX enrollments_status_idx ON enrollments(status);
CREATE INDEX enrollments_payment_status_idx ON enrollments(payment_status);

-- Create function to automatically update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_enrollment_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger to automatically update the updated_at timestamp
CREATE TRIGGER update_enrollment_timestamp
BEFORE UPDATE ON enrollments
FOR EACH ROW
EXECUTE FUNCTION update_enrollment_timestamp();

-- Add comment to document the table
COMMENT ON TABLE enrollments IS 'Stores information about student enrollments in courses';
COMMENT ON COLUMN enrollments.id IS 'Primary key for the enrollment';
COMMENT ON COLUMN enrollments.student_id IS 'Foreign key reference to the student table';
COMMENT ON COLUMN enrollments.course_id IS 'Foreign key reference to the course table';
COMMENT ON COLUMN enrollments.enrollment_date IS 'Date when the student enrolled in the course';
COMMENT ON COLUMN enrollments.status IS 'Current status of the enrollment';
COMMENT ON COLUMN enrollments.completion_date IS 'Date when the student completed the course';
COMMENT ON COLUMN enrollments.completion_status IS 'Final status of completion';
COMMENT ON COLUMN enrollments.final_grade IS 'Final grade received for the course';
COMMENT ON COLUMN enrollments.notes IS 'Additional notes or comments about the enrollment';
COMMENT ON COLUMN enrollments.payment_status IS 'Status of payment for this enrollment';

