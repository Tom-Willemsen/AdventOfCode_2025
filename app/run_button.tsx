"use client";

import Button from "@mui/material/Button";
import CircularProgress from "@mui/material/CircularProgress";

export default function RunButton(props: {
	onClick: () => void;
	busy: boolean;
	disabled: boolean;
}) {
	return (
		<div className="m-2 w-full flex items-center">
			<Button
				variant="contained"
				onClick={props.onClick}
				disabled={props.disabled || props.busy}
			>
				Run solution
			</Button>
			{props.busy ? <CircularProgress size={24} className="mx-4" /> : null}
		</div>
	);
}
