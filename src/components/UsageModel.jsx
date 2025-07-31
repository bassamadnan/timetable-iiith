import React from 'react';
import { useCourseState } from '../context/CourseProvider';

const UsageModal = () => {
  const { isModalOpen, setIsModalOpen } = useCourseState();

  if (!isModalOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex justify-center items-center z-50">
      <div className="bg-white dark:bg-gray-800 p-6 rounded-lg max-w-lg w-full">
        <h2 className="text-xl font-bold mb-4">How to Use this website</h2>
        <p className="mb-4">
          1. Browse available courses in the left panel.<br />
          2. Click on a course to add it to your timetable.<br />
          3. Conflicting courses will appear in the right panel.<br />
          4. Click on a course in the timetable to remove it.<br />
          5. Use the search bars to filter courses and conflicting courses.
        </p>
        <button 
          onClick={() => setIsModalOpen(false)}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          Close
        </button>
      </div>
    </div>
  );
};

export default UsageModal;