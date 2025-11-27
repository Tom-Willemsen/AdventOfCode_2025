'use client'

import TextField from '@mui/material/TextField';
import { Alert } from "@mui/material";
import TimerOutlinedIcon from '@mui/icons-material/TimerOutlined';

export default function PuzzleOutput(props: { output: string, timingData: number | null }) {
    const timingData = props.timingData;
    return (
        <div>
            <TextField
                multiline
                variant="outlined"
                rows={3}
                className="font-mono w-full m-2"
                slotProps={{ htmlInput: { 'className': 'font-mono', "readOnly": true } }}
                value={props.output}
            />
            {timingData !== null ? <Alert severity='info' icon={<TimerOutlinedIcon />} className='m-2'>Browser timing: {timingData.toFixed(3)} ms</Alert> : null}
        </div>
    );
}
