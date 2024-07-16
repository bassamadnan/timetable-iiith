import React from 'react';
import Navbar from './components/Navbar';
import HomePage from './pages/Homepage';
import UsageModal from './components/UsageModel';


const App = () => {
  return (
      <div className="app">
        <Navbar />
        <HomePage />
        <UsageModal />
      </div>
  );
};

export default App;