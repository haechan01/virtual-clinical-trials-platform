import React from 'react'
import './Results.css'
import { signCertificate, } from '@phala/sdk';
import { useAtom } from 'jotai';
import accountAtom from './atoms/account.ts';
import { getSigner } from './lib/polkadotExtension.ts';
import { api, contract, trial_name } from './form';
import { stringify } from '@polkadot/util';

export default async function Results() {

    const [account] = useAtom(accountAtom);
    const signer = await getSigner(account);
    const certificateData = await signCertificate({
        api,
        account,
        signer,
    });
    var message = ""
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



    return ( <
        div > {
            message ? < div >
            <
            h1 > Results
            for { trial_name } < /h1> <
            div className = 'results-container' >
            <
            div className = 'item' > The result on the chained data was : { message } < /div> <
            /div> <
            /div> : <div> You have not uploaded your trial data yet, click on "New trial" to get started </div >
        }

        <
        /div>
    )
}