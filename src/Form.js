import React from 'react';
import { createApi } from './lib/polkadotApi.ts';
import { ContractPromise } from '@polkadot/api-contract';
// import { useEffect, useState } from 'react'
import { create, signCertificate, CertificateData } from '@phala/sdk';
import { Button } from 'baseui/button';
import { ButtonGroup } from 'baseui/button-group';
import { toaster } from 'baseui/toast';
import { useAtom } from 'jotai';
import accountAtom from './atoms/account.ts';
import { getSigner } from './lib/polkadotExtension.ts';

import { useState } from 'react';
import { useFormik } from 'formik';
import './Form.css';
import Papa from 'papaparse';

export default async function Form() {

    function handleCSV(file, fileType) {
        console.log(file)
        var csv = Papa.parse(file, {
            header: false,
            dynamicTyping: true,
            complete: function(results) {
                var data = results.data;

                if (fileType === "raw") {
                    formik.values.file = data
                } else {
                    formik.values.file_preprocessed = data
                }
                console.log(data)

            }
        });
    }

    // imported Polkadot Api
    const endpoint = 'ws://localhost:9944';
    const api = createApi(endpoint);

    // Create a contract object with the metadata and the contract id.
    const pruntimeURL = 'http://127.0.0.1:8000'; // assuming the default port
    const contractId = '0xa5ef1d6cb746b21a481c937870ba491d6fe120747bbeb5304c17de132e8d0392'; // your contract id
    const metadata = require('./metadata.json');
    const contract = new ContractPromise(
        await create({ api, pruntimeURL, contractId }), // Phala's "create" decorator
        JSON.parse(metadata),
        contractId
    );

    const formik = useFormik({
        initialValues: {
            trialName: "",
            testType: "",
            pValueThresh: 0.05,
            file: "",
            file_preprocessed: ""
        },

        // what happens when user submits the form
        onSubmit: async(values) => {

            // try {
            //     // initialize contract 
            //     await contract.tx.default({})
            //         .signAndSend(address, { signer }); // injected signer object from polkadot extension??
            //     console.log("instantiate succeeded")
            // }
            // catch (e) {
            //     console.log(e)
            // }

            try {
                // upload raw
                const received_p = await contract.get_p_value({});
                console.log("user p: %d", values.pValueThresh);
                console.log("received from blockchain: %", received_p);
            } catch (e) {
                console.log(e);
            }
        }
    });





    return ( <
        div className = "container" >
        <
        form onSubmit = { formik.handleSubmit }
        className = "form-container" >

        Upload Raw Data

        < div className = 'file-upload' >
        <
        input id = "file"
        name = "file"
        type = "file"
        className = "upload-field"
        onChange = {
            (event) => {
                handleCSV(event.currentTarget.files[0], "raw");
            }
        }/> 
        </div>
        Upload Preprocessed Data

        < div className = 'file-upload' >
        <
        input id = "file_preprocessed"
        name = "file_preprocessed"
        type = "file"
        className = "upload-field"
        onChange = {
            (event) => {
                handleCSV(event.currentTarget.files[0], "processed");
            }
        }/> 
        </div>
        Give your clinical trial a name

        < div className = "input-block" >
        <
        input className = "input-field"
        id = 'trialName'
        name = 'trialName'
        type = 'text'
        placeholder = "Trial Name"
        onChange = { formik.handleChange }
        value = { formik.values.trialName } />   
        </div>
        Choose the type of test

        < div className = "input-block-radios" >
        <
        input id = 'testType'
        name = 'testType'
        type = 'radio'
        onChange = { formik.handleChange }
        value = "fishers_exact_test"/>
        Fisher 's Exact Test   
        <
        input id = 'testType'
        name = 'testType'
        type = 'radio'
        onChange = { formik.handleChange }
        value = "meandiff" />
        Difference of Means Test 
        </div>

        Choose the significance level threshold

        < div className = "input-block" >
        <
        input className = "input-field"
        id = 'pValueThresh'
        name = 'pValueThresh'
        type = 'number'
        placeholder = "0.05"
        onChange = { formik.handleChange }
        value = { formik.values.pValueThresh }/> 
        </div>

        <
        button type = 'submit'
        className = "button"
        onSubmit = { formik.onSubmit } >
        Submit 
        </button>

        </form> 
        </div >
    )
}
