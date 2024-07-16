import React from 'react';
import { useCourseState } from '../context/CourseProvider';

const CourseSelection = () => {
  const { filteredCourses, handleCourseSelect} = useCourseState();

  return (
    <div className="course-selection">
      <div className="course-list">
        {filteredCourses.length === 0 ? (
          <p>No courses found. Try a different search term.</p>
        ) : (
          filteredCourses.map((course, index) => (
            <div 
              key={index} 
              className="course-item" 
              onClick={() => handleCourseSelect(course)}
            >
              {course.name} - {course.day} {course.slot}
            </div>
          ))
        )}
      </div>
    </div>
  );
};

export default CourseSelection;