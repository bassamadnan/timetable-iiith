import React, { useState, useMemo } from 'react';
import { useCourseState } from '../context/CourseProvider';

const ConflictingCourses = () => {
  const { conflictingCourses } = useCourseState();
  const [conflictingSearchTerm, setConflictingSearchTerm] = useState('');

  const filteredConflictingCourses = useMemo(() => {
    return conflictingCourses.filter(course =>
      course.name.toLowerCase().includes(conflictingSearchTerm.toLowerCase())
    );
  }, [conflictingCourses, conflictingSearchTerm]);

  return (
    <div className="conflicting-courses">
      <input
        type="text"
        placeholder="Search conflicting courses..."
        value={conflictingSearchTerm}
        onChange={(e) => setConflictingSearchTerm(e.target.value)}
        className="search-input mb-4"
      />
      <div className="course-list">
        {filteredConflictingCourses.length === 0 ? (
          <p>No conflicting courses found.</p>
        ) : (
          filteredConflictingCourses.map((course, index) => (
            <div key={index} className="course-item conflicting-course">
              {course.name} - {course.day} {course.slot}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default ConflictingCourses;