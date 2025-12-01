"use client";

import Button from "@mui/material/Button";
import CircularProgress from "@mui/material/CircularProgress";

export default function RunButton(props: {
	onClick: (bench: boolean) => void;
	busy: boolean;
	disabled: boolean;
}) {
	return (
		<div className="m-2 w-full flex items-center">
			<Button
				className="mx-2"
				variant="contained"
				onClick={() => props.onClick(false)}
				disabled={props.disabled || props.busy}
			>
				Run solution
			</Button>
			<Button
				className="mx-2"
				variant="contained"
				onClick={() => props.onClick(true)}
				disabled={props.disabled || props.busy}
			>
				Bench solution
			</Button>
			{props.busy ? <CircularProgress size={24} className="mx-4" /> : null}
		</div>
	);
}
