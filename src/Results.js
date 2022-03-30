import React from 'react'
import './Results.css'

export default function Results() {

    // Testing both cases: when the user has already uploaded trial data and when they havent

    const result = {
        name: "Dummy Clinical Trial",
        pVal: 0.002,
        threshold: 0.05,
        test: "Fischer's Exact Test",
    }

    // const result = null;
    

  return (
    <div>
        {result ? <div>
        <h1>Results for {result.name}</h1>
        <div className='results-container'>
            <div className='item'>Test type: {result.test}</div>
            <div className='item'>p-value: {result.pVal}</div>
            <div className='final-result'>Result: {
                result.pVal < result.threshold ?
                <div className='significant'>
                    Significant
                </div> : 
                <div className='insignificant'>
                    Not significant
                </div>
                }
            </div>
        </div>
        </div> : <div> You have not uploaded your trial data yet, click on "New trial" to get started </div>}
        
    </div>
  )
}
