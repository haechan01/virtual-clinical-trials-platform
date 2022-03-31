import React from 'react'
import {useState} from 'react'
import { useFormik } from 'formik'
import './Form.css';
import Papa from 'papaparse';

export default function Form() {

    function handleCSV(file, fileType) {
        console.log(file)
        var csv = Papa.parse(file, {
            header: false,
            dynamicTyping: true,
            complete: function (results) {
                var data = results.data;
                
                if (fileType === "raw"){
                    formik.values.file = data
                } else {
                    formik.values.file_preprocessed = data
                }
   
            }
        });
    }

    const formik = useFormik({
        initialValues:{
            trialName: "",
            testType: "",
            pValueThresh: 0.05,
            file: "",
            file_preprocessed: ""
        },
        onSubmit: (values) => {
            
        }
    })



  return (
    <div className="container">
    <form onSubmit={formik.handleSubmit} className="form-container">

            Upload Raw Data

    <div className='file-upload'>
        <input id="file" name="file" type="file" className="upload-field" onChange={(event) => {
  handleCSV(event.currentTarget.files[0], "raw");
}} />
        </div>
        Upload Preprocessed Data

<div className='file-upload'>
    <input id="file_preprocessed" name="file_preprocessed" type="file" className="upload-field" onChange={(event) => {
  handleCSV(event.currentTarget.files[0], "processed");
}} />
    </div>
            Give your clinical trial a name

        <div className="input-block">
            <input
            className="input-field"
            id='trialName'
            name='trialName'
            type='text'
            placeholder="Trial Name"
            onChange={formik.handleChange}
            value = {formik.values.trialName}
             />
        </div>
            Choose the type of test

        <div className="input-block-radios">
            <input
            id='testType'
            name='testType'
            type='radio'
            onChange={formik.handleChange}
            value = "fisher" 
             /> Fisher's Exact Test
             <input
            id='testType'
            name='testType'
            type='radio'
            onChange={formik.handleChange}
            value = "meandiff"
             /> Difference of Means Test
        </div>
        
            Choose the significance level threshold
  
        <div className="input-block">
            <input
            className="input-field"
            id='pValueThresh'
            name='pValueThresh'
            type='number'
            placeholder="0.05"
            onChange={formik.handleChange}
            value = {formik.values.pValueThresh}
             />
        </div>
       
        <button type='submit' className="button">Submit</button>

    </form>
    </div>
  )
}
