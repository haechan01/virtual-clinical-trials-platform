import React from 'react';
import { Link } from 'react-router-dom';
import './Nav.css';


export default function Nav() {

    return ( <
        nav >
        <
        div className = 'nav-links' >
        <
        Link className = 'nav-link'
        to = "/" >
        <
        label > Home < /label> < /
        Link > <
        Link className = 'nav-link'
        to = "/new-trial" >
        <
        label > New trial < /label> < /
        Link > <
        Link className = 'nav-link'
        to = "/new-contract-chain" >
        <
        label > Initialize new contract chain < /label> < /
        Link > <
        /div>

        <
        /nav>
    )
}