'use client'

import init, { run_2025_01 } from "@/aoc2025_wasm/pkg/aoc2025_wasm";
import React, { useEffect, useState } from 'react';
import Button from '@mui/material/Button';
import TextField from '@mui/material/TextField';
import Select from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';
import { FormLabel } from "@mui/material";

function doComputation(day: Number, inputData: string, setOutputData, setTimingData) {
  let start = performance.now();
  setOutputData("");
  setTimingData(null);

  let result: string | null = null;
  if (day === 1) {
    result = run_2025_01(inputData);
  }

  let end = performance.now();

  if (result !== null) {
    setOutputData(result);
    setTimingData(end - start);
  } else {
    setOutputData("😢 No solution for that day yet");
    setTimingData(null);
  }
}

export default function Home() {
  const [outputData, setOutputData] = useState("");
  const [inputData, setInputData] = useState("");
  const [webAssemblyReady, setWebAssemblyReady] = useState(false);
  const [day, setDay] = useState(1);
  const [timingData, setTimingData] = useState<number | null>(null);

  useEffect(() => {
    init().then(() => setWebAssemblyReady(true))
  })

  return (
    <div className="w-4/5 max-w-[1000px]">
      <div className="w-full m-2 text-xl">
        <FormLabel>
          Puzzle:
        </FormLabel>
        <Select
          value={day}
          label="Day"
          className="m-2 min-w-[150px]"
          onChange={(event) => setDay(event.target.value)}
        >
          {
            [...Array(12).keys()].map(i => {
              return <MenuItem value={i + 1}>Day {i + 1}</MenuItem>
            })
          }
        </Select>
      </div>
      <TextField
        error={inputData.length === 0}
        multiline
        label="Input data"
        variant="outlined"
        required
        rows={10}
        className="font-mono m-2 w-full"
        slotProps={{
          htmlInput: { className: 'font-mono', 'white-space': 'nowrap', 'overflow': 'hidden' }
        }}
        onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
          setInputData(event.target.value);
        }}
      />
      <div className="m-2 w-full">
        <Button variant="contained" onClick={() => { doComputation(day, inputData, setOutputData, setTimingData) }} disabled={!webAssemblyReady || inputData.length === 0} >Run solution</Button>

        {timingData !== null ? <p className="float-right max-w-1/2 text-l">⌚ Solution time (browser timing): {timingData.toFixed(3)} ms</p> : null}
      </div>
      <TextField
        multiline
        variant="outlined"
        rows={10}
        className="font-mono w-full m-2"
        slotProps={{ htmlInput: { 'className': 'font-mono', "readOnly": true } }}
        value={outputData}
      />
    </div>
  );
}
