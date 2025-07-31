import { createContext, useContext, useState, useEffect } from "react";
import { courses } from "../utils/data_parser";

const CourseContext = createContext();

const CourseProvider = ({ children }) => {
  const [selectedCourses, setSelectedCourses] = useState([]);
  const [conflictingCourses, setConflictingCourses] = useState([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [availableCourses, setAvailableCourses] = useState([]);
  const [allCourses, setAllCourses] = useState([]);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [theme, setTheme] = useState(localStorage.getItem('theme') || 'light');

  useEffect(() => {
    const root = window.document.documentElement;
    if (theme === 'dark') {
      root.classList.add('dark');
    } else {
      root.classList.remove('dark');
    }
    localStorage.setItem('theme', theme);
  }, [theme]);

  const toggleTheme = () => {
    setTheme(prevTheme => prevTheme === 'light' ? 'dark' : 'light');
  };

  useEffect(() => {
    const flattenedCourses = Object.entries(courses).flatMap(([day, slots]) => 
      Object.entries(slots).flatMap(([slot, courseList]) => 
        courseList.map(course => ({ day, slot, name: course }))
      )
    );
    setAllCourses(flattenedCourses);
    setAvailableCourses(flattenedCourses);
  }, []);

  const filterCourses = () => {
    return availableCourses.filter(course => 
      course.name.toLowerCase().includes(searchTerm.toLowerCase())
    );
  };

  const handleCourseSelect = (selectedCourse) => {
    setSelectedCourses(prev => [...prev, selectedCourse]);

    const newConflictingCourses = allCourses.filter(
      course => course.day === selectedCourse.day && 
                course.slot === selectedCourse.slot && 
                course.name !== selectedCourse.name
    );

    setConflictingCourses(prev => [...prev, ...newConflictingCourses]);

    setAvailableCourses(prev => 
      prev.filter(course => 
        !(course.day === selectedCourse.day && course.slot === selectedCourse.slot)
      )
    );
  };

  const handleRemoveCourse = (courseToRemove) => {
    setSelectedCourses(prev => prev.filter(course => course !== courseToRemove));
    
    const removedConflictingCourses = conflictingCourses.filter(
      course => course.day === courseToRemove.day && course.slot === courseToRemove.slot
    );
    
    setConflictingCourses(prev => 
      prev.filter(course => !(course.day === courseToRemove.day && course.slot === courseToRemove.slot))
    );
    
    setAvailableCourses(prev => [...prev, courseToRemove, ...removedConflictingCourses]);
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
        setIsModalOpen
      }}
    >
      {children}
    </CourseContext.Provider>
  );
};

export const useCourseState = () => {
  return useContext(CourseContext);
};

export default CourseProvider;