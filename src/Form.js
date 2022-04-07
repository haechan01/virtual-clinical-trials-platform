import React from 'react';
import { useEffect, useState } from 'react';
import { signCertificate } from '@phala/sdk';
import { useAtom } from 'jotai';
import { Button } from 'baseui/button';
import { Textarea } from 'baseui/textarea';
import accountAtom from './atoms/account.ts';
import { getSigner } from './lib/polkadotExtension.ts';
import { FormControl } from 'baseui/form-control';
import { Input } from 'baseui/input';
import { Block } from 'baseui/block';
import { ToasterContainer } from 'baseui/toast';
import Upload from 'baseui/icon/upload';
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
    const [buttonTextRaw, setButtonTextRaw] = useState()
    const [buttonTextProcessed, setButtonTextProcessed] = useState()
    const [account] = useAtom(accountAtom)
    const [typeState, setType] = useState("fishers_exact_test")
    const [nameState, setName] = useState("")
    const [threshold, setThreshold] = useState(0.05)
    const [fileRawState, setFileRaw] = useState("")
    const [filePreprocessedState, setFilePreprocessed] = useState("")


    useEffect(() => {
        setCertificateData(undefined)
    }, [account])

    function handleCSV(file, fileType) {
        Papa.parse(file, {
            header: false,
            dynamicTyping: true,
            complete: function(results) {
                var data = results.data.slice(1);

                if (fileType === "raw") {
                    setFileRaw(data);
                } else {
                    setFilePreprocessed(data);
                }
            }
        });
    }
    let initialValues = {
        trialName: nameState,
        testType: typeState,
        pValueThresh: threshold,
        file: fileRawState,
        file_preprocessed: filePreprocessedState
    }

    // what happens when user submits the form
    async function afterSubmit(values) {

        if (account && api) {
            try {
                const signer = await getSigner(account)

                // Save certificate data to state, or anywhere else you want like local storage
                const certificate = await signCertificate({
                    api,
                    account,
                    signer,
                });
                console.log(contract)
                NotificationManager.success('Certificate successfully signed', 'Certificate signage', 5000);
                try {
                    // upload preprocessed data
                    await contract.tx.uploadAllPreprocessed({}, api.createType('Vec<Vec<Text>>', values.file_preprocessed))
                        .signAndSend(account.address, { signer }); // injected signer object from polkadot extension??
                    NotificationManager.success('Preprocessed Data uploaded uccessfully', 'Preprocessed Data Upload');
                } catch (e) {
                    console.log(e);
                    NotificationManager.error('Preprocessed Data failed to upload', 'Failed Data Upload', 10000);
                }
                try {
                    // obtain p_value
                    const { received_p } = await contract.query.getPValue(certificate, {});
                    NotificationManager.info(`user p: ${values.pValueThresh}`, "Obtained p-value from form", 5000);
                    NotificationManager.info(`received from blockchain: ${received_p.toHuman()}`, "P-value on-chain", 5000);
                } catch (e) {
                    console.log(e);
                    NotificationManager.error('Failed to obtain on-chain p-value', 'Failed p-value retrieval', 10000);
                }
                try {
                    // obtain stat_test results
                    const { received_result } = await contract.query.getResult(certificate, {});
                    if (received_result.toHuman()) {
                        alert("We have sufficient information to reject the null hypothesis");
                    } else {
                        alert("We do not have sufficient information to reject the null hypothesis");
                    }
                } catch (e) {
                    console.log(e);
                    NotificationManager.error('Failed to obtain Trial results', 'Failed result collection', 10000);
                }
            } catch (err) {
                console.log(err);
                NotificationManager.error(`${err}`, 'Failed to sign certificate', 10000);
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
            ([event]) => {
                setButtonTextRaw(event.name)
                handleCSV(event, "raw");
            }
        }
        overrides = {
            {
                ButtonComponent: {
                    props: {
                        endEnhancer: buttonTextRaw
                    }
                }
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
            ([event]) => {
                setButtonTextProcessed(event.name)
                handleCSV(event, "processed");
            }
        }
        overrides = {
            {
                ButtonComponent: {
                    props: {
                        endEnhancer: buttonTextProcessed
                    }
                }
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
        Button endEnhancer = {
            () => < Upload size = { 24 }
            />} type = 'submit' >
            Submit <
            /Button>< /
            form > <
            NotificationContainer / > < /Block ></div > ): ( < ToasterContainer > <
            ContractLoader name = "Clinical Trial"
            onLoad = {
                ({ api, contract }) => {
                    setApi(api)
                    setContract(contract)
                }
            }
            />< /ToasterContainer >
        )
    }
    FormPage.title = 'Trial upload page';