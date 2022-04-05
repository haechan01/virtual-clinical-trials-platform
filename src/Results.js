import React from 'react';
import './Results.css';
import { useAtom } from 'jotai';
import { signCertificate } from '@phala/sdk';
import { getSigner } from './lib/polkadotExtension.ts';
import accountAtom from './atoms/account.ts';
import { useEffect, useState } from 'react';
import { toaster } from 'baseui/toast'
import { trial_name } from './Form.js';

export default function Results() {
    const [account] = useAtom(accountAtom);
    const [certificateData, setCertificateData] = useState()
    const [api, setApi] = useState()
    const [contract, setContract] = useState()

    useEffect(() => {
        setCertificateData(undefined)
    }, [account])

    async function returnMessage() {
        var message = ""
        try {
            const signer = await getSigner(account)
                // Save certificate data to state, or anywhere else you want like local storage
            setCertificateData(
                await signCertificate({
                    api,
                    account,
                    signer,
                })
            )
            toaster.positive('Certificate signed', {})
            try {
                // obtain stat_test results
                const received_result = await contract.query.get_result(certificateData, {});
                if (received_result) {
                    message = "We have sufficient information to reject the null hypothesis";
                } else {
                    message = "We do not have sufficient information to reject the null hypothesis";
                }
                return message
            } catch (e) {
                console.log(e)
                return null
            }
        } catch (e) {
            toaster.negative((e).message, {})
            return null
        }
    }
    const value = returnMessage()




    return ( < div > {!value ?
            <
            div >
            <
            h1 > Results
            for { trial_name } < /h1> <
            div className = 'results-container' >
            <
            div className = 'item' > The result on the chained data was : { value } < /div>  < /
            div > <
            /div> :  <
            div > You have not uploaded your trial data yet,
            click on "New trial"
            or "Initialize new contract chain"
            toget started < /div >
        } <
        /div>
    )
}