-- Create attendance table to track student attendance for courses
CREATE TABLE IF NOT EXISTS attendance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    student_id UUID NOT NULL REFERENCES students(id) ON DELETE CASCADE,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE CASCADE,
    attendance_date DATE NOT NULL,
    status VARCHAR(10) NOT NULL CHECK (status IN ('present', 'absent', 'late', 'excused')),
    minutes_late INTEGER DEFAULT 0 CHECK (minutes_late >= 0),
    comments TEXT,
    recorded_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    CONSTRAINT unique_student_course_date UNIQUE (student_id, course_id, attendance_date)
);

-- Create index for faster lookups by student
CREATE INDEX idx_attendance_student ON attendance(student_id);

-- Create index for faster lookups by course
CREATE INDEX idx_attendance_course ON attendance(course_id);

-- Create index for date-based queries
CREATE INDEX idx_attendance_date ON attendance(attendance_date);

-- Add a function to update the updated_at timestamp automatically
CREATE OR REPLACE FUNCTION update_attendance_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to call the function before an update
CREATE TRIGGER set_attendance_updated_at
BEFORE UPDATE ON attendance
FOR EACH ROW
EXECUTE FUNCTION update_attendance_updated_at();

COMMENT ON TABLE attendance IS 'Records attendance of students for each course session';
COMMENT ON COLUMN attendance.status IS 'Attendance status: present, absent, late, or excused';
COMMENT ON COLUMN attendance.minutes_late IS 'If status is late, tracks minutes late';
COMMENT ON COLUMN attendance.recorded_by IS 'User ID of the person who recorded the attendance';

