import ical from "ical-generator";
import { timeslots, almanac } from "./data_parser";
import { parse, nextDay } from "date-fns";

/*
 * Copies over timetable for the full week
 */
const processCoursesForExport = (courses) => {
  const courseMap = {
    Monday: "Thursday",
    Tuesday: "Friday",
    Wednesday: "Saturday",
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

// Parse day string like 'Tuesday' to JS day index (0–6)
const getDayIndex = (dayStr) => parse(dayStr, "EEEE", new Date()).getDay();

const getEventDates = (day, timeSlot) => {
  const now = new Date();
  const targetDayIndex = getDayIndex(day);

  const isTodayTargetDay = now.getDay() === targetDayIndex;
  const eventDate = isTodayTargetDay ? now : nextDay(now, targetDayIndex);

  const [startStr, endStr] = timeslots[timeSlot].split("-");

  const start = parse(startStr, "h:mma", eventDate);
  const end = parse(endStr, "h:mma", eventDate);

  return { start, end };
};

export const exportToIcal = (selectedCourses) => {
  if (selectedCourses.length === 0) {
    alert("Please select at least one course to export.");
    return;
  }

  const cal = ical({ name: "IIITH Timetable" });
  const coursesToExport = processCoursesForExport(selectedCourses);
  const semesterEndDate = new Date(almanac.last_date);

  coursesToExport.forEach((course) => {
    const { start, end } = getEventDates(course.day, course.slot);
    cal.createEvent({
      start,
      end,
      summary: course.name,
      repeating: {
        freq: "WEEKLY",
        until: semesterEndDate,
      },
    });
  });

  const blob = new Blob([cal.toString()], {
    type: "text/calendar;charset=utf-8",
  });
  const link = document.createElement("a");
  link.href = URL.createObjectURL(blob);
  link.download = "timetable.ics";
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
};
