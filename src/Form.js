import React from 'react';
import { useEffect, useState } from 'react';
import { signCertificate } from '@phala/sdk';
import { useAtom } from 'jotai';
import { Button } from 'baseui/button';
import { Textarea } from 'baseui/textarea';
import accountAtom from './atoms/account.ts';
import { getSigner } from './lib/polkadotExtension.ts';
import { useFormik } from 'formik';
import { FormControl } from 'baseui/form-control';
import { Input } from 'baseui/input';
import { Block } from 'baseui/block';
import './Form.css';
import { FileUploader } from "baseui/file-uploader";
import { RadioGroup, Radio, ALIGN } from 'baseui/radio';
import 'react-notifications/lib/notifications.css';
import { NotificationContainer, NotificationManager } from 'react-notifications';
import Papa from 'papaparse';
import ContractLoader from './components/ContractLoader.tsx';
import AccountSelect from './components/AccountSelect.tsx';

export default function FormPage() {
    const [certificateData, setCertificateData] = useState()
    const [api, setApi] = useState()
    const [contract, setContract] = useState()
    const account = useAtom(accountAtom)
    const [typeState, setType] = useState("fishers_exact_test")
    const [nameState, setName] = useState("")
    const [threshold, setThreshold] = useState(0.05)

    useEffect(() => {
        setCertificateData(undefined)
    }, [account])

    function handleCSV(file, fileType) {
        Papa.parse(file, {
            header: false,
            dynamicTyping: true,
            complete: function(results) {
                var data = results.data;

                if (fileType === "raw") {
                    initialValues.file = data;
                } else {
                    initialValues.file_preprocessed = data;
                }
                console.log(data);
            }
        });
    }
    let initialValues = {
        trialName: nameState,
        testType: typeState,
        pValueThresh: threshold,
        file: "",
        file_preprocessed: ""
    }

    // what happens when user submits the form
    async function afterSubmit(values) {
        if ((account && api)) {
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
                NotificationManager.success('Certificate successfully signed', 'Certificate signage');
                try {
                    // initialize contract 
                    await contract.tx.default({})
                        .signAndSend(account.address, { signer }); // injected signer object from polkadot extension??
                    NotificationManager.success('Trial block created', 'Contract begin');
                } catch (e) {
                    NotificationManager.error('Could not create trial block', 'Failed block creation', 5000);
                }
                try {
                    // set data conditions
                    await contract.tx.new({}, values.pValueThresh * 100, values.testType)
                        .signAndSend(account.address, { signer }); // injected signer object from polkadot extension??
                    NotificationManager.success('Trial information uploaded successfully', 'Information Upload');
                } catch (e) {
                    NotificationManager.error('Could not upload Trial information', 'Failed information upload', 5000);
                }
                try {
                    // upload preprocessed data
                    await contract.tx.upload_preprocessed({}, values.file_preprocessed)
                        .signAndSend(account.address, { signer }); // injected signer object from polkadot extension??
                    NotificationManager.success('Preprocessed Data uploaded uccessfully', 'Preprocessed Data Upload');
                } catch (e) {
                    NotificationManager.error('Preprocessed Data failed to upload', 'Failed Data Upload', 5000);
                }
                try {
                    // obtain p_value
                    const received_p = await contract.query.get_p_value(certificateData, {});
                    NotificationManager.info(`user p: ${values.pValueThresh}`, "Obtained p-value from form");
                    NotificationManager.info(`received from blockchain: ${received_p}`, "P-value on-chain");
                } catch (e) {
                    NotificationManager.error('Failed to obtain on-chain p-value', 'Failed p-value retrieval', 5000);
                }
                try {
                    // obtain stat_test results
                    const received_result = await contract.query.get_result(certificateData, {});
                    if (received_result) {
                        alert("We have sufficient information to reject the null hypothesis");
                    } else {
                        alert("We do not have sufficient information to reject the null hypothesis");
                    }
                } catch (e) {
                    NotificationManager.error('Failed to obtain trial results', 'Failed result collection', 5000);
                }
            } catch (err) {
                NotificationManager.error(`${err}`, 'Failed to sign certificate', 5000);
            }
        } else {
            alert("No defined account for use")
        }
    }




    return contract ? ( <
        div className = "container" > <
        Block > <
        form onSubmit = {
            (e) => {
                e.preventDefault()
                afterSubmit(initialValues)
            }
        }
        className = "form-container" >
        <
        AccountSelect / >
        <
        FormControl label = "Upload Raw Data" >
        <
        FileUploader accept = ".csv"
        onDrop = {
            (event) => {
                console.log(event[0])
                handleCSV(event[0], "raw");
            }
        }
        name = "file" /
        >
        <
        /FormControl> <
        FormControl label = "Upload Preprocessed Data" >
        <
        FileUploader accept = ".csv"
        onDrop = {
            (event) => {
                console.log(event[0])
                handleCSV(event[0], "processed");
            }
        }
        name = "file" /
        >
        <
        /FormControl> <
        FormControl label = "Provide reference name for Clinical Trial" >
        <
        Textarea placeholder = "Trial Name"
        overrides = {
            {
                Input: {
                    style: {
                        fontFamily: 'monospace',
                    },
                },
            }
        }
        value = { initialValues.trialName }
        onChange = { e => setName(e.currentTarget.value) }
        />< /
        FormControl >
        <
        RadioGroup value = { initialValues.testType }
        onChange = { e => setType(e.currentTarget.value) }
        name = "Test Type"
        align = { ALIGN.horizontal }
        label = "Choose the type of applied statistical test " >
        <
        Radio value = "fishers_exact_test"
        description = "Default Statistical test"
        checked > Fisher 's Exact Test   < /Radio> <
        Radio value = "Difference of means test" > Difference of Means Test < /Radio> < /
        RadioGroup > <
        FormControl label = "Choose significance level threshold" >
        <
        Input placeholder = "0.05"
        overrides = {
            {
                Input: {
                    style: {
                        fontFamily: 'monospace',
                    },
                },
            }
        }
        value = { initialValues.pValueThresh }
        type = 'number'
        onChange = { e => setThreshold(e.currentTarget.value) }
        />< /
        FormControl >

        <
        Button type = 'submit' >
        Submit <
        /Button>< /
        form > <
        NotificationContainer / > < /Block ></div > ) : ( <
        ContractLoader name = "Clinical Trial"
        onLoad = {
            ({ api, contract }) => {
                setApi(api)
                setContract(contract)
            }
        }
        /> 
    )
}
FormPage.title = 'Trial upload page';