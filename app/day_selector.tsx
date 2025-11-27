"use client";

import { FormLabel } from "@mui/material";
import MenuItem from "@mui/material/MenuItem";
import Select from "@mui/material/Select";

export default function DaySelector(props: {
	day: number;
	setDay: (day: number) => void;
}) {
	return (
		<div className="w-full m-2 text-xl">
			<FormLabel>Puzzle:</FormLabel>
			<Select
				value={props.day}
				label="Day"
				className="m-2 min-w-[150px]"
				onChange={(event) => props.setDay(event.target.value)}
			>
				{[...Array(12).keys()].map((i) => {
					return (
						<MenuItem value={i + 1} key={i}>
							Day {i + 1}
						</MenuItem>
					);
				})}
			</Select>
		</div>
	);
}
