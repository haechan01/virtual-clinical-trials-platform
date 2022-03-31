import React, { Component } from 'react';
import logo from './logo.svg';
import './App.css';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import Form from './Form';
import Results from './Results';
import Nav from './Nav'
// import ContractLoader from './components/ContractLoader';

function App() {
  return (
    <Router>
      <Nav />
      <div className="App">
        <Routes>
          <Route path="/" exact element={<Home />} />
          <Route path="/new-trial" element={<Form />} />
          <Route path="/trial-results" element={<Results />} />
        </Routes>
      </div>
    </Router>
  );
}

const Home = () => {
  return (
    <div>
      <h1>
        Welcome to Clinical Trial Platform
      </h1>
      click "New Trial" to get started.
    </div>
  )
}

export default App;
