import React from 'react';
import './App.css';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import FormPage from './Form.js';
import Nav from './Nav.js'

function App() {

    return ( <
        Router >
        <
        Nav / >
        <
        div className = "App" >
        <
        Routes >
        <
        Route path = "/"
        exact element = { < Home / > }
        /> <
        Route path = "/new-trial"
        element = { < FormPage / > }
        /> < /
        Routes >
        <
        / div >   < /
        Router >
    );
}

const Home = () => {
    return ( <
        div >
        <
        h1 >
                        Welcome to the Clinical Trial Data Platform secured by Phala Networks <
        /h1>
                        This is the data upload platform to provide a secure, immutable data framework
                        for your clinical trial studies. Click "New Trial"
        to get started. < /
        div >
    )
}

export default App;