"use client";

import TextField from "@mui/material/TextField";

export default function PuzzleInput(props: {
	inputData: string;
	setInputData: (inputData: string) => void;
}) {
	return (
		<TextField
			error={props.inputData.length === 0}
			multiline
			label="Input data"
			variant="outlined"
			value={props.inputData}
			required
			rows={10}
			className="m-2 w-full"
			slotProps={{
				htmlInput: {
					className: "font-mono",
				},
			}}
			onChange={(event: React.ChangeEvent<HTMLInputElement>) => {
				props.setInputData(event.target.value);
			}}
		/>
	);
}
