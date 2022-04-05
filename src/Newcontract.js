import React from 'react';
import { useEffect, useState } from 'react';
import './Form.css';
import ContractLoader from './components/ContractLoader.tsx';

export default function Resetcontract() {
    const [api, setApi] = useState()
    const [contract, setContract] = useState()
    const trial_name = "New contract"

    return ( <
        ContractLoader name = { trial_name }
        onLoad = {
            ({ api, contract }) => {
                setApi(api)
                setContract(contract)
            }
        }
        />)
    }