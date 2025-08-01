import ical from 'ical-generator';
import { timeslots } from './data_parser';

const semesterEndDate = new Date('2025-11-20T23:59:59');

const processCoursesForExport = (courses) => {
  const courseMap = {
    Monday: 'Thursday',
    Tuesday: 'Friday',
    Wednesday: 'Saturday',
  };

  const duplicatedCourses = courses.reduce((acc, course) => {
    acc.push(course);
    if (courseMap[course.day]) {
      acc.push({ ...course, day: courseMap[course.day] });
    }
    return acc;
  }, []);

  return duplicatedCourses;
};

const getEventDates = (day, timeSlot) => {
    const now = new Date();
    const dayOfWeek = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'].indexOf(day);
    const resultDate = new Date(now.getTime());
    const diff = now.getDay() - dayOfWeek;
    if (diff > 0) {
        resultDate.setDate(now.getDate() + (7 - diff));
    } else {
        resultDate.setDate(now.getDate() + (-diff));
    }
    
    const [startTime, endTime] = timeslots[timeSlot].replace('AM', '').replace('PM', '').split('-');
    const [startHour, startMinute] = startTime.split(':').map(Number);
    const [endHour, endMinute] = endTime.split(':').map(Number);

    const start = new Date(resultDate);
    start.setHours(startHour < 8 ? startHour + 12 : startHour, startMinute || 0, 0, 0);

    const end = new Date(resultDate);
    end.setHours(endHour < 8 ? endHour + 12 : endHour, endMinute || 0, 0, 0);
    
    return { start, end };
}

export const exportToIcal = (selectedCourses) => {
  if (selectedCourses.length === 0) {
    alert("Please select at least one course to export.");
    return;
  }
  
  const cal = ical({ name: 'IIITH Timetable' });
  const coursesToExport = processCoursesForExport(selectedCourses);

  coursesToExport.forEach(course => {
    const { start, end } = getEventDates(course.day, course.slot);
    cal.createEvent({
      start,
      end,
      summary: course.name,
      repeating: {
        freq: 'WEEKLY',
        until: semesterEndDate,
      },
    });
  });

  const blob = new Blob([cal.toString()], { type: 'text/calendar;charset=utf-8' });
  const link = document.createElement('a');
  link.href = URL.createObjectURL(blob);
  link.download = 'timetable.ics';
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
};