import React, { createContext, useContext, useState, useEffect } from "react";
import PropTypes from "prop-types";
import { courses } from "../utils/data_parser";

const CourseContext = createContext();

const CourseProvider = ({ children }) => {
  const [selectedCourses, setSelectedCourses] = useState(() => {
    const savedCourses = localStorage.getItem("selectedCourses");
    return savedCourses ? JSON.parse(savedCourses) : [];
  });
  const [conflictingCourses, setConflictingCourses] = useState([]);
  const [searchTerm, setSearchTerm] = useState("");
  const [availableCourses, setAvailableCourses] = useState([]);
  const [allCourses, setAllCourses] = useState([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [theme, setTheme] = useState(localStorage.getItem("theme") || "light");

  useEffect(() => {
    localStorage.setItem("selectedCourses", JSON.stringify(selectedCourses));
  }, [selectedCourses]);

  useEffect(() => {
    const root = window.document.documentElement;
    if (theme === "dark") {
      root.classList.add("dark");
    } else {
      root.classList.remove("dark");
    }
    localStorage.setItem("theme", theme);
  }, [theme]);

  const toggleTheme = () => {
    setTheme((prevTheme) => (prevTheme === "light" ? "dark" : "light"));
  };

  useEffect(() => {
    const flattenedCourses = Object.entries(courses).flatMap(([day, slots]) =>
      Object.entries(slots).flatMap(([slot, courseList]) =>
        courseList.map((course) => ({ day, slot, name: course })),
      ),
    );
    setAllCourses(flattenedCourses);
  }, []);

  useEffect(() => {
    const newAvailableCourses = allCourses.filter(
      (course) =>
        !selectedCourses.some(
          (selected) =>
            selected.day === course.day && selected.slot === course.slot,
        ),
    );
    setAvailableCourses(newAvailableCourses);

    const newConflictingCourses = allCourses.filter((course) =>
      selectedCourses.some(
        (selected) =>
          selected.day === course.day &&
          selected.slot === course.slot &&
          selected.name !== course.name,
      ),
    );
    setConflictingCourses(newConflictingCourses);
  }, [selectedCourses, allCourses]);

  const filterCourses = () => {
    return availableCourses.filter((course) =>
      course.name.toLowerCase().includes(searchTerm.toLowerCase()),
    );
  };

  const handleCourseSelect = (selectedCourse) => {
    setSelectedCourses((prev) => [...prev, selectedCourse]);
  };

  const handleRemoveCourse = (courseToRemove) => {
    setSelectedCourses((prev) =>
      prev.filter(
        (course) =>
          !(
            course.name === courseToRemove.name &&
            course.day === courseToRemove.day &&
            course.slot === courseToRemove.slot
          ),
      ),
    );
  };

  const handleSearchChange = (term) => {
    setSearchTerm(term);
  };

  return (
    <CourseContext.Provider
      value={{
        selectedCourses,
        conflictingCourses,
        filteredCourses: filterCourses(),
        searchTerm,
        isModalOpen,
        theme,
        toggleTheme,
        handleCourseSelect,
        handleSearchChange,
        handleRemoveCourse,
        setIsModalOpen,
      }}
    >
      {children}
    </CourseContext.Provider>
  );
};

CourseProvider.propTypes = {
  children: PropTypes.node.isRequired,
};

export const useCourseState = () => {
  return useContext(CourseContext);
};

export default CourseProvider;
