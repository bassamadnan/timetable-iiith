import React from 'react';
import { useCourseState } from '../context/CourseProvider';
import { timeslots } from '../utils/data_parser';

const Timetable = () => {
    const { selectedCourses, handleRemoveCourse } = useCourseState();
    const days = ['Monday', 'Tuesday', 'Wednesday'];
  
    const findCourse = (day, slot) => {
      return selectedCourses.find(course => course.day === day && course.slot === slot);
    };

  return (
    <div className="timetable-grid">
      <div className="grid-header">
        <div className="header-cell"></div>
        {Object.entries(timeslots).map(([slotName, slotTime]) => (
          <div key={slotName} className="header-cell">{slotTime} - {slotName}</div>
        ))}
      </div>
      {days.map(day => (
        <div key={day} className="grid-row">
          <div className="day-cell">{day}</div>
          {Object.keys(timeslots).map(slot => {
            const course = findCourse(day, slot);
            return (
              <div 
                key={`${day}-${slot}`} 
                className="grid-cell"
                onClick={() => course && handleRemoveCourse(course)}
              >
                {course ? (
                  <div className="selected-course">
                    <p className="course-name">{course.name}</p>
                  </div>
                ) : null}
              </div>
            );
          })}
        </div>
      ))}
    </div>
  );
};

export default Timetable;