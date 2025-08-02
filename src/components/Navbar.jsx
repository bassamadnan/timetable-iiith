import React from 'react';
import { FaGithub, FaSun, FaMoon, FaDownload } from 'react-icons/fa';
import { useCourseState } from '../context/CourseProvider';
import { exportToIcal } from '../utils/export';

const Navbar = () => {
  const { selectedCourses, setIsModalOpen, theme, toggleTheme } = useCourseState();

  const openGitHub = () => {
    window.open('https://github.com/bassamadnan/timetable-iiith', '_blank');
  };

  const handleIcalExport = () => {
    exportToIcal(selectedCourses);
  };

  return (
    <nav className="navbar flex justify-between items-center px-4 py-2">
      <h1 className="navbar-title text-xl font-bold">Timetable @ IIITH</h1>
      <div className="flex items-center">
        <button 
          onClick={() => setIsModalOpen(true)}
          className="mr-4 px-3 py-1 bg-gray-500 text-white rounded hover:bg-gray-600"
        >
          Usage
        </button>
        <button onClick={toggleTheme} className="text-2xl mr-4">
          {theme === 'dark' ? <FaSun /> : <FaMoon />}
        </button>
        <button onClick={handleIcalExport} className="text-2xl mr-4" title="Export as iCal" aria-label="Export as iCal">
          <FaDownload />
        </button>
        <button onClick={openGitHub} className="text-2xl">
          <FaGithub />
        </button>
      </div>
    </nav>
  );
};

export default Navbar;