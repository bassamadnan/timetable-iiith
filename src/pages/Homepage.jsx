import React from 'react';
import Timetable from '../components/Timetable';
import CourseSelection from '../components/CourseSelection';
import ConflictingCourses from '../components/ConflictingCourses';
import { useCourseState } from '../context/CourseProvider';

const HomePage = () => {
  const { searchTerm, handleSearchChange } = useCourseState();

  return (
    <div className="homepage">
      <div className="timetable">
        <Timetable />
      </div>
      <div className="course-management">
        <div className="course-selection-container">
          <input
            type="text"
            placeholder="Search courses..."
            value={searchTerm}
            onChange={(e) => handleSearchChange(e.target.value)}
            className="search-input"
          />
          <CourseSelection />
        </div>
        <ConflictingCourses />
      </div>
    </div>
  );
};

export default HomePage;