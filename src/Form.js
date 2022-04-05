import React from 'react';
import { useEffect, useState } from 'react';
import { signCertificate } from '@phala/sdk';
import { useAtom } from 'jotai';
import accountAtom from './atoms/account.ts';
import { getSigner } from './lib/polkadotExtension.ts';
import { useFormik } from 'formik';
import './Form.css';
import { toaster } from 'baseui/toast'
import Papa from 'papaparse';
import ContractLoader from './components/ContractLoader.tsx';

export var trial_name = ""

export default function Form() {
    const [account] = useAtom(accountAtom);
    const [certificateData, setCertificateData] = useState()
    const [api, setApi] = useState()
    const [contract, setContract] = useState()


    useEffect(() => {
        setCertificateData(undefined)
    }, [account])

    function handleCSV(file, fileType) {
        console.log(file)
        Papa.parse(file, {
            header: false,
            dynamicTyping: true,
            complete: function(results) {
                var data = results.data;

                if (fileType === "raw") {
                    formik.values.file = data;
                } else {
                    formik.values.file_preprocessed = data;
                }
                console.log(data);
            }
        });
    }

    const formik = useFormik({
        initialValues: {
            trialName: "",
            testType: "",
            pValueThresh: 0.05,
            file: "",
            file_preprocessed: ""
        },

        validate: (values) => {
            const errors = {};
            if (!values.trialName) {
                errors.trialName = 'Required';
            } else if (!/^[A-Z0-9]$/i.test(values.trialName)) {
                errors.trialName = 'Invalid Trial Name type';
            }
            if (!values.testType) {
                errors.testType = 'Required';
            }
            if (!values.file_preprocessed) {
                errors.file_preprocessed = 'Required';
            }
            return errors;
        },

        // what happens when user submits the form
        onSubmit: async(values) => {
            trial_name = values.trialName;
            if (account && api) {
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
                        // initialize contract 
                        await contract.tx.default({})
                            .signAndSend(account.address, { signer }); // injected signer object from polkadot extension??
                        console.log("instantiate succeeded");
                        await api.disconnect()
                    } catch (e) {
                        console.log(e);
                    }
                    try {
                        // set data conditions
                        await contract.tx.new({}, values.pValueThresh * 100, values.testType)
                            .signAndSend(account.address, { signer }); // injected signer object from polkadot extension??
                        console.log("Property Upload succeeded");
                        await api.disconnect()
                    } catch (e) {
                        console.log(e);
                    }
                    try {
                        // upload preprocessed data
                        await contract.tx.upload_preprocessed({}, values.file_preprocessed)
                            .signAndSend(account.address, { signer }); // injected signer object from polkadot extension??
                        await api.disconnect()
                        console.log("Data upload succeeded");
                    } catch (e) {
                        console.log(e);
                    }
                    try {
                        // obtain p_value
                        const received_p = await contract.query.get_p_value(certificateData, {});
                        await api.disconnect()
                        console.log("user p: %d", values.pValueThresh);
                        console.log("received from blockchain: %", received_p);
                    } catch (e) {
                        console.log(e);
                    }
                } catch (err) {
                    toaster.negative((err).message, {})
                }
            }
        }
    });





    return contract ? ( < div className = "container" >
        <
        form onSubmit = { formik.handleSubmit }
        className = "form-container" >

        Upload Raw Data

        <
        div className = 'file-upload' >
        <
        input id = "file"
        name = "file"
        type = "file"
        className = "upload-field"
        onChange = {
            (event) => {
                handleCSV(event.currentTarget.files[0], "raw");
            }
        }
        /> </div >
        Upload Preprocessed Data

        <
        div className = 'file-upload' >
        <
        input id = "file_preprocessed"
        name = "file_preprocessed"
        type = "file"
        className = "upload-field"
        onChange = {
            (event) => {
                handleCSV(event.currentTarget.files[0], "processed");
            }
        }
        /> </div >
        Give your clinical trial a name

        <
        div className = "input-block" >
        <
        input className = "input-field"
        id = 'trialName'
        name = 'trialName'
        type = 'text'
        placeholder = "Trial Name"
        onChange = { formik.handleChange }
        value = { formik.values.trialName }
        /> </div >
        Choose the type of test

        <
        div className = "input-block-radios" >
        <
        input id = 'testType'
        name = 'testType'
        type = 'radio'
        onChange = { formik.handleChange }
        value = "fishers_exact_test" / >

        Fisher 's Exact Test    

        <
        input id = 'testType'
        name = 'testType'
        type = 'radio'
        onChange = { formik.handleChange }
        value = "meandiff" / >
        Difference of Means Test < /div>

        Choose the significance level threshold

        <
        div className = "input-block" >
        <
        input className = "input-field"
        id = 'pValueThresh'
        name = 'pValueThresh'
        type = 'number'
        placeholder = "0.05"
        onChange = { formik.handleChange }
        value = { formik.values.pValueThresh }
        /> </div >

        <
        button type = 'submit'
        className = "button"
        onSubmit = { formik.onSubmit } >
        Submit < /button>

        <
        /form> </div >
    ) : ( <
        ContractLoader name = { trial_name }
        onLoad = {
            ({ api, contract }) => {
                setApi(api)
                setContract(contract)
            }
        }
        /> 
    )
}