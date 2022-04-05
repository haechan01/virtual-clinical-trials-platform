import React from 'react';
import './Results.css';
import { ContractPromise } from '@polkadot/api-contract';
import { ApiPromise } from '@polkadot/api';
import { signCertificate, } from '@phala/sdk';
import { useAtom } from 'jotai';
import accountAtom from './atoms/account.ts';
import { useEffect, useState } from 'react';
import { trial_name } from './Form.js';
import { stringify } from '@polkadot/util';

export default async function Results() {

    const [account] = useAtom(accountAtom);
    const [certificateData, setCertificateData] = useState()
    const [api, setApi] = useState()
    const [contract, setContract] = useState()

    useEffect(
        () => () => {
            api.disconnect()
        }, [api]
    )

    useEffect(() => {
        setCertificateData(undefined)
    }, [account])
    var message = "";
    try {
        // obtain stat_test results
        const received_result = await contract.query.get_result(certificateData, {});
        if (received_result) {
            message = "We have sufficient information to reject the null hypothesis";
        } else {
            message = "We do not have sufficient information to reject the null hypothesis";
        }
    } catch (e) {
        message = stringify(e);
    }



    return ( < div > {
            message ?
            <
            div >
            <
            h1 > Results
            for { trial_name } < /h1> <
            div className = 'results-container' >
            <
            div className = 'item' > The result on the chained data was : { message } < /div>  < /
            div > <
            /div> :  <
            div > You have not uploaded your trial data yet,
            click on "New trial"
            toget started < /div >
        } <
        /div>
    )
}