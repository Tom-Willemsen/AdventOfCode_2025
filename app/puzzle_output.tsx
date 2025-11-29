"use client";

import TimerOutlinedIcon from "@mui/icons-material/TimerOutlined";
import { Alert } from "@mui/material";
import TextField from "@mui/material/TextField";

export default function PuzzleOutput(props: {
	output: string;
	timingData: number | null;
}) {
	const output = props.output;
	const timingData = props.timingData;
	return (
		<div>
			<TextField
				multiline
				variant="filled"
				label="Solution output"
				disabled={output.length === 0}
				rows={3}
				className="w-full m-2"
				slotProps={{ htmlInput: { className: "font-mono", readOnly: true } }}
				value={output}
				error={output.startsWith("Error: ")}
			/>
			{timingData !== null ? (
				<Alert severity="info" icon={<TimerOutlinedIcon />} className="m-2">
					Browser timing: {timingData.toFixed(1)} ms
				</Alert>
			) : null}
		</div>
	);
}
