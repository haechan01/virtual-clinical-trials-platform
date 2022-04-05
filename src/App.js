import React from 'react';
import logo from './logo.svg';
import './App.css';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom'
import Form from './Form.js';
import Results from './Results.js';
import Nav from './Nav.js'
import Resetcontract from './Newcontract.js';

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
        element = { < Form / > }
        /> <
        Route path = "/new-contract-chain"
        element = { < Resetcontract / > }
        /> <
        Route path = "/trial-results"
        element = { < Results / > }
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
        Welcome to the Clinical Trial Platform use -
        case for Phala Networks <
        /h1>
        This is the data upload platform aimed at providing a secure, immutable platform
        for your clinical services.Click "New Trial"
        to get started. <
        /div>
    )
}

export default App;