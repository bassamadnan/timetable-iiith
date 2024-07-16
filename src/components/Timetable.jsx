import React from 'react';
import { useCourseState } from '../context/CourseProvider';

const Timetable = () => {
    const { selectedCourses, handleRemoveCourse } = useCourseState();
    const days = ['Monday', 'Tuesday', 'Wednesday'];
    const timeSlots = ['T1', 'T2', 'T3', 'T4', 'T5', 'T6'];
  
    const findCourse = (day, slot) => {
      return selectedCourses.find(course => course.day === day && course.slot === slot);
    };

  return (
    <div className="timetable-grid">
      <div className="grid-header">
        <div className="header-cell"></div>
        {timeSlots.map(slot => (
          <div key={slot} className="header-cell">{slot}</div>
        ))}
      </div>
      {days.map(day => (
        <div key={day} className="grid-row">
          <div className="day-cell">{day}</div>
          {timeSlots.map(slot => {
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