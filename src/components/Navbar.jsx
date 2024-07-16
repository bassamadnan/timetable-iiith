import React, { useState } from 'react';
import { FaGithub } from 'react-icons/fa';
import UsageModal from './UsageModel';
import { useCourseState } from '../context/CourseProvider';


const Navbar = () => {
  const {setIsModalOpen, isModalOpen} = useCourseState()
  const openGitHub = () => {
    window.open('https://github.com/bassamadnan/timetable-iiith', '_blank');
  };

  return (
    <nav className="navbar flex justify-between items-center px-4 py-2">
      <h1 className="navbar-title text-xl font-bold">Timetable @ IIITH</h1>
      <div className="flex items-center">
        <button 
          onClick={() => setIsModalOpen(true)}
          className="mr-4 px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Usage
        </button>
        <button onClick={openGitHub} className="text-2xl">
          <FaGithub />
        </button>
      </div>
      <UsageModal isOpen={isModalOpen} onClose={() => setIsModalOpen(false)} />
    </nav>
  );
};

export default Navbar;